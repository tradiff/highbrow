# Browser Fork

A simple browser chooser for Linux with configurable rules to automatically route web URLs to different browsers.

![screenshot](media/screenshot.png)

## Features

- Rules-based URL routing: Automatically open URLs in specific browsers based on regex patterns
- Browser selector: When no rule matches, use the simple UI to choose the browser

## Getting Started

1. Clone the repository:
```sh
git clone https://github.com/tradiff/browser-fork.git
cd browser-fork
```

2. Build in release mode:

```sh
cargo build --release
```

3. Install the binary into your PATH:

```sh
cp target/release/browser-fork ~/.local/bin/
```

4. Run once to set Browser Fork as your default browser:

```
browser-fork
```

Use the UI prompt to set Browser Fork as your default browser.

## Configuration
Create a configuration file at `~/.config/browser-fork.toml` with the following structure:

```toml
[[browsers]]
id = "firefox"
label = "_Firefox"
command = "firefox"
icon_name = "firefox"

[[browsers]]
id = "chromium"
label = "_Chromium"
command = "chromium-browser"
icon_name = "chromium"

[[rules]]
regex = "https?://(?:.*\\.)?facebook\\.com/.*"
browser_id = "chromium"

[[rules]]
regex = "https?://mail\\.google\\.com/.*"
browser_id = "firefox"
```

**browsers**: define each browser with an id, display label, executable command, and icon_name.

**rules**: list zero or more regex-based rules to map URLs to a browser. Any URLs that do not match a rule will display the selection UI.
