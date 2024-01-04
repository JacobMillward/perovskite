use std::collections::HashMap;

use muda::{ContextMenu, IsMenuItem, Menu, MenuEvent, MenuId, Submenu};
use winit::{
    dpi::LogicalSize,
    error::EventLoopError,
    event::{ElementState, Event, MouseButton, WindowEvent},
    event_loop::{ControlFlow, EventLoop, EventLoopBuilder, EventLoopWindowTarget},
    window::{Window, WindowBuilder},
};

#[cfg(target_os = "macos")]
use winit::platform::macos::{EventLoopBuilderExtMacOS, WindowExtMacOS};
#[cfg(target_os = "linux")]
use winit::platform::unix::WindowExtUnix;
#[cfg(target_os = "windows")]
use winit::platform::windows::EventLoopBuilderExtWindows;

use super::input::InputManager;

/// A dispatch map for menu items.
/// This is a map from menu item IDs to closures that will be called when the menu item is
/// activated.
pub type MenuAction = Box<dyn Fn()>;
pub type MenuDispatchMap = HashMap<MenuId, MenuAction>;

pub struct MenuItemWithAction {
    pub menu_id: MenuId,
    pub action: MenuAction,
}
pub trait MenuItemExt
where
    Self: IsMenuItem,
{
    fn with_action(&self, action: MenuAction) -> MenuItemWithAction
    where
        Self: Sized;
}
impl<T> MenuItemExt for T
where
    T: IsMenuItem + Sized + Clone,
{
    fn with_action(&self, action: MenuAction) -> MenuItemWithAction {
        MenuItemWithAction {
            menu_id: self.clone().into_id(),
            action,
        }
    }
}

/// A builder for creating an App.
/// This struct is used to configure an App before creating it.
/// The `build` method will create the App.
pub struct AppBuilder {
    window_title: String,
    window_width: u32,
    window_height: u32,
    menu_bar: Option<Menu>,
    context_menu: Option<Submenu>,
    menu_dispatch_map: MenuDispatchMap,
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
        }
    }

    pub fn with_window_title(mut self, window_title: &str) -> Self {
        self.window_title = window_title.to_string();
        self
    }

    pub fn with_window_size(mut self, width: u32, height: u32) -> Self {
        self.window_width = width;
        self.window_height = height;
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

    pub fn build(self) -> Result<App, Box<dyn std::error::Error>> {
        App::new(
            self.window_title,
            self.window_width,
            self.window_height,
            self.menu_bar,
            self.menu_dispatch_map,
            self.context_menu,
        )
    }
}

impl Default for AppBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// A wrapper around a winit Window and a muda Menu.
/// This struct is used to create and manage a window and menu bar.
/// It also provides a callback for handling your application loop.
pub struct App {
    pub window: Window,
    menu_bar: Option<Menu>,
    context_menu: Option<Submenu>,
    menu_dispatch_map: MenuDispatchMap,
    event_loop: Option<EventLoop<()>>,
    input_manager: InputManager,
}

impl App {
    /// Create new App with a menu bar.
    /// It should be called before any other menu-related functions.
    pub fn new(
        window_title: String,
        width: u32,
        height: u32,
        menu_bar: Option<Menu>,
        menu_dispatch_map: MenuDispatchMap,
        context_menu: Option<Submenu>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let mut event_loop_builder = EventLoopBuilder::new();

        if let Some(menu_bar) = menu_bar.as_ref() {
            Self::init_menu_hooks(&mut event_loop_builder, menu_bar);
        }

        let event_loop = event_loop_builder.build()?;

        let size = LogicalSize::new(width, height);
        let window = WindowBuilder::new()
            .with_title(window_title)
            .with_inner_size(size)
            .with_min_inner_size(size)
            .build(&event_loop)?;

        let mut app = Self {
            window,
            menu_bar,
            context_menu,
            menu_dispatch_map,
            event_loop: Some(event_loop),
            input_manager: InputManager::new(),
        };

        app.init()?;
        Ok(app)
    }

    fn init_menu_hooks(event_loop_builder: &mut EventLoopBuilder<()>, menu: &Menu) {
        #[cfg(target_os = "windows")]
        {
            let menu_bar = menu.clone();
            event_loop_builder.with_msg_hook(move |msg| {
                use windows_sys::Win32::UI::WindowsAndMessaging::{TranslateAcceleratorW, MSG};
                unsafe {
                    let msg = msg as *const MSG;
                    let translated = TranslateAcceleratorW((*msg).hwnd, menu_bar.haccel(), msg);
                    translated == 1
                }
            });
        }

        #[cfg(target_os = "macos")]
        event_loop_builder.with_default_menu(false);
    }

    /// Initialize the App
    /// This function sets up the menu bar for the given window.
    /// This function is platform-specific, and should only be called once.
    fn init(&mut self) -> Result<(), muda::Error> {
        if self.menu_bar.is_none() {
            return Ok(());
        }

        let menu_bar = self.menu_bar.as_ref().unwrap();

        #[cfg(target_os = "windows")]
        {
            use winit::raw_window_handle::*;
            if let RawWindowHandle::Win32(handle) = self.window.window_handle().unwrap().as_raw() {
                menu_bar.init_for_hwnd(handle.hwnd.get())?
            }
        }
        #[cfg(target_os = "macos")]
        {
            menu_bar.init_for_nsapp()?
        }
        #[cfg(target_os = "linux")]
        {
            let gtk_window = self.window.gtk_window();
            let vertical_gtk_box = self.window.default_vbox();
            menu_bar.init_for_gtk_window(&gtk_window, Some(&vertical_gtk_box))?
        }

        Ok(())
    }

    /// Show the context menu for the app's window.
    /// The context menu is shown at the current mouse position.
    pub fn show_context_menu(&self) {
        if self.context_menu.is_none() {
            return;
        }

        let context_menu = self.context_menu.as_ref().unwrap();

        #[cfg(target_os = "windows")]
        {
            use winit::raw_window_handle::*;
            if let RawWindowHandle::Win32(handle) = self.window.window_handle().unwrap().as_raw() {
                context_menu.show_context_menu_for_hwnd(handle.hwnd.get(), None);
            }
        }
        #[cfg(target_os = "macos")]
        {
            use winit::raw_window_handle::*;
            if let RawWindowHandle::AppKit(handle) = self.window.window_handle().unwrap().as_raw() {
                context_menu.show_context_menu_for_nsview(handle.ns_view.as_ptr() as _, None);
            }
        }
        #[cfg(target_os = "linux")]
        {
            let gtk_window = self.window.gtk_window();
            let vertical_gtk_box = self.window.default_vbox();
            context_menu.show_context_menu_for_gtk_window(&gtk_window, vertical_gtk_box);
        }
    }

    /// A callback for handling window events.
    /// This function should be called from the event loop, for every event.
    fn handle_window_event(&mut self, event: &Event<()>, event_loop: &EventLoopWindowTarget<()>) {
        if let Event::WindowEvent { event, .. } = event {
            match event {
                WindowEvent::CloseRequested => event_loop.exit(),
                WindowEvent::KeyboardInput { event, .. } => {
                    self.input_manager
                        .handle_keyboard_event(event.physical_key, event.state);
                }
                WindowEvent::MouseInput {
                    state: ElementState::Pressed,
                    button: MouseButton::Right,
                    ..
                } => {
                    self.show_context_menu();
                }
                _ => {}
            }
        }

        // Handle menu events
        let menu_channel = MenuEvent::receiver();
        if let Ok(event) = menu_channel.try_recv() {
            if let Some(dispatch) = self.menu_dispatch_map.get(&event.id) {
                dispatch();
            }
        }
    }

    /// Run the application loop.
    /// This function will block until the application is closed.
    /// The event loop is handled internally, and is ran in polling mode.
    /// The given callback will be called for every event after internal event handling.
    pub fn run<F>(&mut self, mut app_loop_handler: F) -> Result<(), EventLoopError>
    where
        F: FnMut(Event<()>, &EventLoopWindowTarget<()>, &InputManager),
    {
        let event_loop = self.event_loop.take().expect("Event loop already consumed");

        event_loop.run(move |event, event_loop| {
            event_loop.set_control_flow(ControlFlow::Poll);
            self.handle_window_event(&event, event_loop);
            app_loop_handler(event, event_loop, &self.input_manager);
        })
    }
}
