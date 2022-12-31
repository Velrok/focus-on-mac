[![Rust](https://github.com/Velrok/focus-on-mac/actions/workflows/rust.yml/badge.svg)](https://github.com/Velrok/focus-on-mac/actions/workflows/rust.yml)

# focus-on-mac

Super simple focus app for Mac. Set focus and call it to notify you every so often.

# Usage

```
❯ focus-on -h
Usage: focus-on [OPTIONS] [FOCUS]...

Arguments:
  [FOCUS]...

Options:
  -n, --notify
  -h, --help     Print help information
  -V, --version  Print version information
```

```
focus-on Finishing important project.
```

Also need to setup a cron to run in which ever interval you want like once per hour:

```
focus-on --notify
```
