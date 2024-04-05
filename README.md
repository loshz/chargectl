# chargectl
[![Build Status](https://github.com/loshz/chargectl/workflows/ci/badge.svg)](https://github.com/loshz/chargectl/actions) [![MIT licensed](https://img.shields.io/badge/license-MIT-blue)](LICENSE)

Linux daemon for managing charge thresholds on modern laptop machines. Inspired by [TLP](https://linrunner.de/tlp/).

## Usage
```
$ chargectl --help
Linux daemon for managing charge thresholds on modern laptop machines.

Usage: chargectl <COMMAND>

Commands:
  set    Set start and stop charge thresholds
  full   Set threshold to enable immediate charging until full
  start  Run as a dameon, periodically resetting charge thresholds

Options:
  -h, --help     Print help
  -V, --version  Print version
```

### systemd
An example service unit can be found [here](./extra/chargectl.service). You may need to modify the `ExecStart` path depending on your installation configuration.
