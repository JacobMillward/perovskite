use anyhow::Result;
use perovskite::{
    menu::{MenuItemExt, MenuItemWithAction},
    muda::{AboutMetadata, Menu, MenuItem, PredefinedMenuItem, Submenu},
    App, AppSettings, RenderContext,
};

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

fn main() -> Result<()> {
    let world = World::new();

    App::run(world)?;

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
}

impl App for World {
    fn init(&mut self) -> Result<AppSettings> {
        let mut app_menu = Menu::new();
        let menu_actions = create_menu_items(&mut app_menu)?;

        let settings = AppSettings::builder()
            .with_window_title("Minimal Example - Pixels".to_string())
            .with_frame_size(WIDTH, HEIGHT)
            .with_menu_bar(app_menu)
            .with_menu_actions(menu_actions)
            .build();

        Ok(settings)
    }

    fn update(&mut self, _: &mut RenderContext) -> Result<()> {
        self.box_x += self.velocity_x;
        self.box_y += self.velocity_y;

        if self.box_x <= 0 || self.box_x + BOX_SIZE >= WIDTH as i16 {
            self.velocity_x *= -1;
        }

        if self.box_y <= 0 || self.box_y + BOX_SIZE >= HEIGHT as i16 {
            self.velocity_y *= -1;
        }

        Ok(())
    }

    /// Draw the `World` state to the frame buffer.
    ///
    /// Assumes the default texture format: `wgpu::TextureFormat::Rgba8UnormSrgb`
    fn draw(&mut self, ctx: &mut RenderContext) -> Result<()> {
        {
            let frame = ctx.pixels_mut().frame_mut();

            for (i, cur_pixel) in frame.chunks_exact_mut(4).enumerate() {
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

                cur_pixel.copy_from_slice(&rgba);
            }
        }

        ctx.window().pre_present_notify();
        ctx.pixels_mut().render()?;

        Ok(())
    }
}

/// Create a menu bar with the default menu items.
fn create_menu_items(menu: &mut Menu) -> Result<Vec<MenuItemWithAction>, perovskite::muda::Error> {
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
        menu.append(&app_m)?;
        app_m.append_items(&[
            &about,
            &PredefinedMenuItem::separator(),
            &PredefinedMenuItem::quit(None),
        ])?;
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
