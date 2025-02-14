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

- [anyhow](https://docs.rs/anyhow/latest/anyhow/) for error handling
- [fuzzy-matcher](https://docs.rs/fuzzy-matcher/latest/fuzzy_matcher/) for fuzzy search
- [ratatui](https://docs.rs/ratatui/latest/ratatui/) for the interactive UI
- [toml](https://docs.rs/toml/latest/toml/) for parsing `.toml` files
- [xdg](https://docs.rs/xdg/latest/xdg/) for finding the `alacritty.toml` file in the known locations

## Resources

- [Alacritty](https://github.com/alacritty/alacritty) source code
- Ratatui documentation, especially the [Counter App](https://ratatui.rs/tutorials/counter-app/basic-app/) and the [User Input](https://ratatui.rs/examples/apps/user_input/) examples

## Similar projects

- [alacritty-theme-switcher](https://github.com/spacebird-dev/alacritty-theme-switcher) A simple tool to quickly switch between different themes for alacritty, with shell completion!
  - This repo differs from the above in that it displays a TUI with fuzzy search and live preview, where as the project above is a CLI tool with no TUI
