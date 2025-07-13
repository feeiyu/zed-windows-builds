use std::{
    ffi::{OsStr, OsString},
    io::{self, Error, ErrorKind},
    os::windows::ffi::OsStrExt,
};

fn ensure_no_nuls<T: AsRef<OsStr>>(s: T) -> io::Result<T> {
    if s.as_ref().encode_wide().any(|b| b == 0) {
        Err(Error::new(
            ErrorKind::InvalidInput,
            "nul byte found in provided data",
        ))
    } else {
        Ok(s)
    }
}

pub(crate) fn append_arg(cmd: &mut Vec<u16>, arg: &OsString, force_quotes: bool) -> io::Result<()> {
    // If an argument has 0 characters then we need to quote it to ensure
    // that it actually gets passed through on the command line or otherwise
    // it will be dropped entirely when parsed on the other end.
    ensure_no_nuls(arg)?;
    let arg_bytes = arg.as_encoded_bytes();
    let quote = if force_quotes {
        true
    } else {
        arg_bytes.iter().any(|c| *c == b' ' || *c == b'\t') || arg_bytes.is_empty()
    };
    if quote {
        cmd.push('"' as u16);
    }

    let mut backslashes: usize = 0;
    for x in arg.encode_wide() {
        if x == '\\' as u16 {
            backslashes += 1;
        } else {
            if x == '"' as u16 {
                // Add n+1 backslashes to total 2n+1 before internal '"'.
                cmd.extend((0..=backslashes).map(|_| '\\' as u16));
            }
            backslashes = 0;
        }
        cmd.push(x);
    }

    if quote {
        // Add n backslashes to total 2n before ending '"'.
        cmd.extend((0..backslashes).map(|_| '\\' as u16));
        cmd.push('"' as u16);
    }
    Ok(())
}

fn make_command_line(argv0: &OsStr, args: &[OsString], force_quotes: bool) -> io::Result<Vec<u16>> {
    // Encode the command and arguments in a command line string such
    // that the spawned process may recover them using CommandLineToArgvW.
    let mut cmd: Vec<u16> = Vec::new();

    // Always quote the program name so CreateProcess to avoid ambiguity when
    // the child process parses its arguments.
    // Note that quotes aren't escaped here because they can't be used in arg0.
    // But that's ok because file paths can't contain quotes.
    cmd.push(b'"' as u16);
    cmd.extend(argv0.encode_wide());
    cmd.push(b'"' as u16);

    for arg in args {
        cmd.push(' ' as u16);
        append_arg(&mut cmd, arg, force_quotes)?;
    }
    Ok(cmd)
}

pub fn cmdline<S: AsRef<OsStr>>(program: S, args: Vec<S>) -> std::io::Result<String> {
    let args: Vec<OsString> = args.iter().map(|arg| arg.as_ref().to_os_string()).collect();
    let mut cmd_str = make_command_line(program.as_ref(), &args, false)?;
    cmd_str.push(0); // add null terminator
    Ok(String::from_utf16_lossy(&cmd_str))
}
