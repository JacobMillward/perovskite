# Perovskite

Perovskite is a simple framework for building cross-platform GUI applications in Rust. It allows you to create a window and draw to it using the `pixels` crate. It also provides a simple way to create a menu bar using the `muda` crate.

The API is heavily inspired by XNA/Monogame.

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

```rust
use perovskite::App;

fn main() {
    let app = MyApp;
    App::run(app).unwrap();
}

struct MyApp;
impl App for MyApp {
    fn update(&mut self, _ctx: &mut perovskite::RenderContext) -> perovskite::anyhow::Result<()> {
        // Update function here
        Ok(())
    }

    fn draw(&mut self, _ctx: &mut perovskite::RenderContext) -> perovskite::anyhow::Result<()> {
        // Draw function here
        // You can use the ctx to draw
        // let frame = ctx.pixels_mut().frame_mut();
        // self.draw_frame(frame);
        // ctx.pixels_mut().render();

        Ok(())
    }
}

```

## License

This project is licensed under the MIT License - see the [LICENSE](./LICENSE) file for details.
