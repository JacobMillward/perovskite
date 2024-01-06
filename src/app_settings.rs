use std::time::Duration;

use muda::{Menu, Submenu};

use crate::menu::{MenuDispatchMap, MenuItemWithAction};

/// Defines the settings for an App.
pub struct AppSettings {
    /// The title of the window.
    pub(crate) window_title: String,

    /// The width of the window.
    /// If not set, the frame width will be used.
    pub(crate) window_width: Option<u32>,

    /// The height of the window.
    /// If not set, the frame height will be used.
    pub(crate) window_height: Option<u32>,

    /// The width of the frame drawn by the app. It will be scaled up cleanly to the window size, while maintaining
    /// aspect ratio.
    pub(crate) frame_width: u32,

    /// The height of the frame drawn by the app. It will be scaled up cleanly to the window size, while maintaining
    /// aspect ratio.
    pub(crate) frame_height: u32,

    /// If set, this menu will be used as the menu bar for the app.
    pub(crate) menu_bar: Option<Menu>,

    /// If set, this menu will be used as the context menu for the app and appear on right click.
    pub(crate) context_menu: Option<Submenu>,

    /// A dispatch map for menu items.
    /// Links menu item IDs to closures that will be called when the menu item is activated.
    pub(crate) menu_dispatch_map: MenuDispatchMap,

    /// The target frame time for the app.
    /// The apps `update` function will be called once per target frame time, but may be called mutliple times
    /// before the `draw` function is called.
    pub(crate) target_frame_time: Duration,

    /// The maximum frame time for the app.
    /// The maximum amount of time that can be taken by the `update` function before the `draw` function is called.
    /// Ideally this should be set to a multiple of the target frame time.
    pub(crate) max_frame_time: Duration,
}

impl AppSettings {
    pub fn builder() -> AppBuilder {
        AppBuilder::default()
    }
}

/// A builder for creating an App.
/// This struct is used to configure an App before creating it.
/// The `build` method will create the App.
#[derive(Default)]
pub struct AppBuilder {
    window_title: Option<String>,
    window_width: Option<u32>,
    window_height: Option<u32>,
    frame_width: Option<u32>,
    frame_height: Option<u32>,
    menu_bar: Option<Menu>,
    context_menu: Option<Submenu>,
    menu_dispatch_map: MenuDispatchMap,
    target_frame_time: Option<Duration>,
    max_frame_time: Option<Duration>,
}

impl AppBuilder {
    pub fn new() -> Self {
        Self {
            window_title: None,
            window_width: None,
            window_height: None,
            frame_width: None,
            frame_height: None,
            menu_bar: None,
            context_menu: None,
            menu_dispatch_map: MenuDispatchMap::new(),
            target_frame_time: None,
            max_frame_time: None,
        }
    }

    pub fn with_window_title(mut self, window_title: String) -> Self {
        self.window_title = Some(window_title);
        self
    }

    pub fn with_window_size(mut self, width: u32, height: u32) -> Self {
        self.window_width = Some(width);
        self.window_height = Some(height);
        self
    }

    pub fn with_frame_size(mut self, width: u32, height: u32) -> Self {
        self.frame_width = Some(width);
        self.frame_height = Some(height);
        self
    }

    pub fn with_menu_bar(mut self, menu_bar: Menu) -> Self {
        self.menu_bar = Some(menu_bar);
        self
    }

    pub fn with_context_menu(mut self, context_menu: Submenu) -> Self {
        self.context_menu = Some(context_menu);
        self
    }

    pub fn with_menu_actions(mut self, menu_actions: Vec<MenuItemWithAction>) -> Self {
        for item in menu_actions {
            self.menu_dispatch_map.insert(item.menu_id, item.action);
        }
        self
    }

    pub fn with_target_frame_time(mut self, target_frame_time: Duration) -> Self {
        self.target_frame_time = Some(target_frame_time);
        self
    }

    pub fn with_max_frame_time(mut self, max_frame_time: Duration) -> Self {
        self.max_frame_time = Some(max_frame_time);
        self
    }

    pub fn build(self) -> AppSettings {
        AppSettings {
            window_title: self.window_title.unwrap_or_else(|| "App".to_string()),
            window_width: self.window_width,
            window_height: self.window_height,
            frame_width: self.frame_width.unwrap_or(640),
            frame_height: self.frame_height.unwrap_or(480),
            menu_bar: self.menu_bar,
            context_menu: self.context_menu,
            menu_dispatch_map: self.menu_dispatch_map,
            target_frame_time: self
                .target_frame_time
                .unwrap_or_else(|| Duration::from_millis(16)),
            max_frame_time: self
                .max_frame_time
                .unwrap_or_else(|| Duration::from_millis(32)),
        }
    }
}
