use ferro_app::{
    muda::{AboutMetadata, Menu, MenuItem, PredefinedMenuItem, Submenu},
    winit::event::{Event, WindowEvent},
    AppBuilder, MenuItemExt, MenuItemWithAction,
};
use pixels::{Pixels, SurfaceTexture};

const WIDTH: u32 = 320;
const HEIGHT: u32 = 240;
const BOX_SIZE: i16 = 64;

/// Representation of the application state. In this example, a box will bounce around the screen.
struct World {
    box_x: i16,
    box_y: i16,
    velocity_x: i16,
    velocity_y: i16,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut app_menu = Menu::new();
    let menu_actions = create_menu_items(&mut app_menu)?;
    let mut app = AppBuilder::new()
        .with_window_title("Minimal Example - Pixels")
        .with_window_size(WIDTH, HEIGHT)
        .with_menu_bar(app_menu)
        .with_menu_actions(menu_actions)
        .build()?;

    let mut pixels = {
        let window_size = app.window.inner_size();
        let surface_texture =
            SurfaceTexture::new(window_size.width, window_size.height, &app.window);
        Pixels::new(WIDTH, HEIGHT, surface_texture)?
    };
    let mut world = World::new();

    app.run(|event, event_loop, _| {
        match event {
            Event::AboutToWait => {
                world.draw(pixels.frame_mut());

                if let Err(err) = pixels.render() {
                    eprintln!("pixels.render() failed: {}", err);
                    event_loop.exit();
                    return;
                }
                world.update();
            }
            Event::WindowEvent {
                event: WindowEvent::Resized(size),
                ..
            } => {
                if let Err(err) = pixels.resize_surface(size.width, size.height) {
                    eprintln!("pixels.resize_surface() failed: {}", err);
                    event_loop.exit();
                }
                if let Err(err) = pixels.render() {
                    eprintln!("pixels.render() failed: {}", err);
                    event_loop.exit();
                }
            }
            _ => {}
        };
    })
    .expect("Failed to run event loop");

    Ok(())
}

impl World {
    /// Create a new `World` instance that can draw a moving box.
    fn new() -> Self {
        Self {
            box_x: 24,
            box_y: 16,
            velocity_x: 1,
            velocity_y: 1,
        }
    }

    /// Update the `World` internal state; bounce the box around the screen.
    fn update(&mut self) {
        if self.box_x <= 0 || self.box_x + BOX_SIZE > WIDTH as i16 {
            self.velocity_x *= -1;
        }
        if self.box_y <= 0 || self.box_y + BOX_SIZE > HEIGHT as i16 {
            self.velocity_y *= -1;
        }

        self.box_x += self.velocity_x;
        self.box_y += self.velocity_y;
    }

    /// Draw the `World` state to the frame buffer.
    ///
    /// Assumes the default texture format: `wgpu::TextureFormat::Rgba8UnormSrgb`
    fn draw(&self, frame: &mut [u8]) {
        for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {
            let x = (i % WIDTH as usize) as i16;
            let y = (i / WIDTH as usize) as i16;

            let inside_the_box = x >= self.box_x
                && x < self.box_x + BOX_SIZE
                && y >= self.box_y
                && y < self.box_y + BOX_SIZE;

            let rgba = if inside_the_box {
                [0x5e, 0x48, 0xe8, 0xff]
            } else {
                [0x48, 0xb2, 0xe8, 0xff]
            };

            pixel.copy_from_slice(&rgba);
        }
    }
}

/// Create a menu bar with the default menu items.
fn create_menu_items(menu: &mut Menu) -> Result<Vec<MenuItemWithAction>, ferro_app::muda::Error> {
    let version = option_env!("CARGO_PKG_VERSION").map(|s| s.to_string());
    let authors = option_env!("CARGO_PKG_AUTHORS")
        .map(|s| s.split(':').map(|s| s.trim().to_string()).collect());

    let about = PredefinedMenuItem::about(
        None,
        Some(AboutMetadata {
            name: Some("Minimal Pixels".to_string()),
            version,
            authors,
            ..Default::default()
        }),
    );

    #[cfg(target_os = "macos")]
    {
        let app_m = Submenu::new("App", true);
        menu.append(&app_m);
        app_m.append_items(&[
            &about,
            &PredefinedMenuItem::separator(),
            &PredefinedMenuItem::quit(None),
        ]);
    }

    let file_m = Submenu::with_items(
        "&File",
        true,
        &[&PredefinedMenuItem::close_window(Some("Exit"))],
    )?;
    let open = MenuItem::new("Open", true, None);

    file_m.prepend_items(&[&open])?;

    let help_m = Submenu::with_items("&Help", true, &[&about])?;

    menu.append_items(&[&file_m, &help_m])?;

    let dispatch_map = vec![open.with_action(Box::new(|| {
        println!("Open was clicked!");
    }))];

    Ok(dispatch_map)
}
