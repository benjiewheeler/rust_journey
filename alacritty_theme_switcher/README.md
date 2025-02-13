# Alacritty Theme Switcher

The first little project I decided to implement in Rust is a small command line utility that allows you to switch themes for [Alacritty](https://alacritty.org/) terminal.

## Goals

- Locate the `alacritty.toml` config file the known [locations](https://alacritty.org/config-alacritty.html#location)
- Locate the `themes/themes` directory (according to this [repo](https://github.com/alacritty/alacritty-theme))
- Scan the `themes` directory for `.toml` files
- Display the names of the themes in the terminal in a searchable format (similar to [Telescope](https://github.com/nvim-telescope/telescope.nvim))

  - Change the theme in real time
  - Save the selected theme if the user hits `Enter`
  - Restore the original theme and exit if the user hits `Esc`

## Used Crates

- [toml](https://docs.rs/toml/latest/toml/) for parsing `.toml` files
- [ratatui](https://docs.rs/ratatui/latest/ratatui/) for the interactive UI

## Similar projects

- [alacritty-theme-switcher](https://github.com/spacebird-dev/alacritty-theme-switcher) A simple tool to quickly switch between different themes for alacritty, with shell completion!
