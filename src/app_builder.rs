use std::{collections::HashMap, time::Duration};

use muda::{Menu, Submenu};

use crate::menu::{MenuDispatchMap, MenuItemWithAction};

/// A builder for creating an App.
/// This struct is used to configure an App before creating it.
/// The `build` method will create the App.
pub struct AppBuilder {
    pub(crate) window_title: String,
    pub(crate) window_width: u32,
    pub(crate) window_height: u32,
    pub(crate) menu_bar: Option<Menu>,
    pub(crate) context_menu: Option<Submenu>,
    pub(crate) menu_dispatch_map: MenuDispatchMap,
    pub(crate) target_frame_time: Duration,
    pub(crate) max_frame_time: Duration,
}

impl AppBuilder {
    pub fn new() -> Self {
        Self {
            window_title: "App".to_string(),
            window_width: 320,
            window_height: 240,
            menu_bar: None,
            context_menu: None,
            menu_dispatch_map: HashMap::new(),
            target_frame_time: Duration::from_millis(16),
            max_frame_time: Duration::from_millis(32),
        }
    }

    pub fn with_window_title(&mut self, window_title: &str) -> &mut Self {
        self.window_title = window_title.to_string();
        self
    }

    pub fn with_window_size(&mut self, width: u32, height: u32) -> &mut Self {
        self.window_width = width;
        self.window_height = height;
        self
    }

    pub fn with_menu_bar(&mut self, menu_bar: Menu) -> &mut Self {
        self.menu_bar = Some(menu_bar);
        self
    }

    pub fn with_context_menu(&mut self, context_menu: Submenu) -> &mut Self {
        self.context_menu = Some(context_menu);
        self
    }

    pub fn with_menu_actions(&mut self, menu_actions: Vec<MenuItemWithAction>) -> &mut Self {
        for item in menu_actions {
            self.menu_dispatch_map.insert(item.menu_id, item.action);
        }
        self
    }

    pub fn with_target_frame_time(&mut self, target_frame_time: Duration) -> &mut Self {
        self.target_frame_time = target_frame_time;
        self
    }

    pub fn with_max_frame_time(&mut self, max_frame_time: Duration) -> &mut Self {
        self.max_frame_time = max_frame_time;
        self
    }
}

impl Default for AppBuilder {
    fn default() -> Self {
        Self::new()
    }
}
