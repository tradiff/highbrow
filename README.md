# Highbrow

<p align="center"><img src="./media/highbrow-256x256.png"></p>

For users who appreciate the finer things in life, Highbrow is a lightweight, rule-based browser selector for Linux.

![screenshot](media/screenshot.png)

## Features

- Rules-based URL routing: Automatically open URLs in specific browsers based on regex patterns
- Browser selector: When no rule matches, use the simple UI to choose the browser

## Getting Started

1. Clone the repository:
```sh
git clone https://github.com/tradiff/highbrow.git
cd highbrow
```

2. Build in release mode:

```sh
cargo build --release
```

3. Run once to set Highbrow as your default browser:

```sh
target/release/highbrow
```

Use the UI prompt to set Highbrow as your default browser.

## Configuration
Create a configuration file at `~/.config/highbrow.toml` with the following structure:

```toml
# Optional: Set a default browser for URLs that don't match any patterns
default_browser = "_Firefox"

[[browsers]]
label = "_Firefox" # Underscore prefix creates the keyboard shortcut Alt+f
command = "firefox" # Command to launch the browser
icon_name = "firefox" # System icon name
patterns = [
    'https://facebook\.com/.*',
    'https://spotify\.com/.*',
]

[[browsers]]
label = "_Chromium"
command = "chromium-browser"
icon_name = "chromium"
patterns = [
    'https://mycompany\.com/.*',
]
```

- **label**: Browser name shown in the selector UI. Prefix a letter with underscore (_) to create an Alt+Key keyboard shortcut.
- **command**: The executable command to launch the browser. You can include arguments (e.g. `firefox --private-window` or `firefox -P personal`)
- **icon_name**: The system icon name from your icon theme.
- **patterns**: List of regular expressions to match URLs. When a URL matches a pattern, it automatically opens in the corresponding browser. Any URLs that do not match a rule will display the selection UI.

### Example patterns

```toml
patterns = [
    # Match exact domain
    'https://example\.com(/.*)?',
    
    # Match http or https, and any subdomains
    'https?://(.*\.)?example\.com(/.*)?',
]
```