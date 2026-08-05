<div align="center">
    <img src="./assets/icon.svg" alt="icon" width="120px">
</div>

<h1 align="center">Portproc</h1>

<div align="center">
    <strong>Quickly map any port to a pid!</strong>
</div>
<br>
<div align="center">
    <a href="https://github.com/Bishops-exe/portproc/blob/main/LICENSE" title="View license!">
        <img alt="License" src="https://img.shields.io/github/license/bishops-exe/portproc?color=880088"/>
    </a>
    <img alt="Static Badge" src="https://img.shields.io/badge/unsafe-1_block-008800">
    <a href="https://github.com/Bishops-exe/portproc/pulls" title="Open a pull request!">
        <img alt="Open a pull request!" src="https://img.shields.io/badge/PR-create one!-008800">
    </a>
    <a href="https://github.com/Bishops-exe/portproc/issues/new" title="Report a bug!">
        <img alt="Report a bug!" src="https://img.shields.io/badge/Bug%3F-report_it!-880000">
    </a>

</div>

## Installation

You can install the binary from the [releases tab on GitHub](https://github.com/Bishops-exe/portproc/releases).

## Usage

```sh
$ portproc --help

Quickly map any port to a pid!

Usage: portproc.exe [OPTIONS] --log-level <LOG_LEVEL> <--all|--port <PORT>>

Options:
  -a, --all                    Operate on every port currently in use, instead of a specific list
  -p, --port <PORT>            Port(s) to operate on, e.g. "t8080" (TCP), "u53" (UDP), or a preset name like "ssh"
  -k, --kill                   Kill the process bound to the given port(s)
  -r, --restart                Kill the process bound to the given port(s) and restart it with the same command, args, and working directory
  -c, --attach                 When restarting, stay attached to the new process instead of detaching it (requires --restart, only one port)
  -i, --ignore-unused          Dont error out when a given port has no process using it
  -l, --log-level <LOG_LEVEL>  Set the loglevel off this instance [possible values: off, error, warn, info, debug, trace]
  -h, --help                   Print help
  -V, --version                Print version
```

These predefined ports can be found
inside [./src/portmap.rs](https://github.com/Bishops-exe/portproc/blob/main/src/portmap.rs#L11)

## Support development!

- <a href="https://github.com/Bishops-exe/portproc/pulls" title="Create a pull request!">Pull requests</a>
and <a href="https://github.com/Bishops-exe/portproc/issues" title="Create an issue!">issues for bugs and crashes</a> are always welcome!

- You can also suggest a new feature to make this tool even better!

- If you find this repository interesting, consider giving us a star!

- You can also donate through [GitHub Sponsors!](https://github.com/sponsors/bishops-exe)

## License

Portproc is licensed under
the [GNU General Public License v3.0](https://github.com/Bishops-exe/portproc/blob/main/LICENSE)!