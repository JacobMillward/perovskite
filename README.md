# Ferro App

Ferro App is a simple framework for building cross-platform GUI applications in Rust.

This project uses the `winit` and `muda` libraries to create a GUI application. It provides an `App` struct that you can use to create a new application with a menu bar.

## Dependencies

Make sure you have the latest version of Rust installed. You can download Rust from [here](https://www.rust-lang.org/tools/install).

### Linux Only

This depends on [muda](https://github.com/tauri-apps/muda) for cross-platform menu bars, and therefore building on linux requires some additional packages. `gtk` is used for menus. Be sure to install following packages before building:

#### Arch Linux / Manjaro:

```sh
pacman -S gtk3 xdotool
```

#### Debian / Ubuntu:

```sh
sudo apt install libgtk-3-dev libxdo-dev
```

## Usage

Examples can be found in the `examples` directory.

## License

This project is licensed under the MIT License - see the [LICENSE](./LICENSE) file for details.
