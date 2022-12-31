[![Rust](https://github.com/Velrok/focus-on-mac/actions/workflows/rust.yml/badge.svg)](https://github.com/Velrok/focus-on-mac/actions/workflows/rust.yml)

# focus-on-mac

Super simple focus app for Mac. Set focus and call it to notify you every so often.

<img width="375" alt="Screenshot 2022-12-31 at 19 19 17" src="https://user-images.githubusercontent.com/34974/210153754-3a0c6ec6-6e73-4946-8796-697fcdfdc90a.png">


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
