# Color Scheme

Simple color configuration for configuration files.

Currently supports the following style colors. Feel free to submit an issue if you would be interested.

```
hex: #rrggbb  
rrggbb: rrggbbaa
```

## Usage
Create a file next to you config file, with the extension: `{color-type}.cscheme`, with tokens instead of colors. E.g. for `alacritty.toml`:

```toml
# alacritty.toml.hex.cscheme
[colors.primary]
background = '$black'
foreground = '$light-gray'
```

This will generate:
```toml
#alacritty.toml
[colors.primary]
background = '#1b2b34'
foreground = '#c0c5ce'
```

It requires path so your configuration directory, usually `.config` and path to a color scheme file in the following format:

```json
{
  "font-family": "JetBrainsMono NL",
  "black": "#1b2b34",
  "light-black": "#343d46",
  "dark-gray": "#4f5b66",
  "gray": "#65737e",
  "faint-gray": "#a7adba",
  "light-gray": "#c0c5ce",
  "dark-white": "#cdd3de",
  "white": "#d8dee9",
  "red": "#db686b",
  "orange": "#f99157",
  "yellow": "#f2ca73",
  "green": "#a2c699",
  "cyan": "#74b1b2",
  "blue": "#7198c8",
  "purple": "#c594c5",
  "brown": "#ab7967"
}

```
