# Unofficial builds of Zed for Windows

**NOTE: This is not a support channel for Zed on Windows.**

These builds are for those who want to live on the bleeding edge or just want to test Zed out on Windows.

Any issues with the Windows build should go through official channels, as this repository does not concern itself with the source code of Zed or issues found therein.

If you have suggestions for improvements to the build process, please start a discussion or make a PR.

## Remote Development from windows to linux

The Windows remote development functionality in this build is not production-ready and should be considered a temporary solution until official support becomes available.

- upload `remote_server` to `~/.zed_server`
- Completed SSH trust setup from Windows to Linux.

## Is it safe?

This repository include the remote develop patch and a [simple GitHub workflow](./.github/workflows/build.yml) that builds Zed from `main` and publishes a release every night at UTC+0000. (Additionally on push for testing).

See the [Zed homepage](https://zed.dev/) or [official repository](https://github.com/zed-industries/zed) for more details.
