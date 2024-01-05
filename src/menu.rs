use std::collections::HashMap;

use anyhow::Result;
use muda::{ContextMenu, IsMenuItem, Menu, MenuId};
use winit::{event_loop::EventLoopBuilder, window::Window};

#[cfg(target_os = "macos")]
use winit::platform::macos::{EventLoopBuilderExtMacOS, WindowExtMacOS};
#[cfg(target_os = "linux")]
use winit::platform::unix::WindowExtUnix;
#[cfg(target_os = "windows")]
use winit::platform::windows::EventLoopBuilderExtWindows;

/// A dispatch map for menu items.
/// This is a map from menu item IDs to closures that will be called when the menu item is
/// activated.
pub type MenuDispatchMap = HashMap<MenuId, MenuAction>;
pub type MenuAction = Box<dyn Fn()>;

/// A wrapper around a MenuId and a function that will be called when the menu item is activated.
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

/// Initialize the platform-specific menu hooks for the app's window.
pub fn init_menu_hooks(event_loop_builder: &mut EventLoopBuilder<()>, menu: &Menu) {
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

/// Adds a `Menu` to the given `Window`.
/// This function is platform-specific, and should only be called once.
pub fn init_menu(window: &Window, menu_bar: &Menu) -> Result<()> {
    #[cfg(target_os = "windows")]
    {
        use winit::raw_window_handle::*;
        if let RawWindowHandle::Win32(handle) = window.window_handle().unwrap().as_raw() {
            menu_bar.init_for_hwnd(handle.hwnd.get())?
        }
    }
    #[cfg(target_os = "macos")]
    {
        menu_bar.init_for_nsapp()?
    }
    #[cfg(target_os = "linux")]
    {
        let gtk_window = window.gtk_window();
        let vertical_gtk_box = window.default_vbox();
        menu_bar.init_for_gtk_window(&gtk_window, Some(&vertical_gtk_box))?
    }

    Ok(())
}

/// Show the context menu for the app's window.
/// The context menu is shown at the current mouse position.
pub fn show_context_menu_for_window(window: &Window, context_menu: &dyn ContextMenu) {
    #[cfg(target_os = "windows")]
    {
        use winit::raw_window_handle::*;
        if let RawWindowHandle::Win32(handle) = window.window_handle().unwrap().as_raw() {
            context_menu.show_context_menu_for_hwnd(handle.hwnd.get(), None);
        }
    }
    #[cfg(target_os = "macos")]
    {
        use winit::raw_window_handle::*;
        if let RawWindowHandle::AppKit(handle) = window.window_handle().unwrap().as_raw() {
            context_menu.show_context_menu_for_nsview(handle.ns_view.as_ptr() as _, None);
        }
    }
    #[cfg(target_os = "linux")]
    {
        let gtk_window = window.gtk_window();
        let vertical_gtk_box = window.default_vbox();
        context_menu.show_context_menu_for_gtk_window(&gtk_window, vertical_gtk_box);
    }
}
