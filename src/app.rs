use anyhow::{Context, Result};
use muda::{Menu, MenuEvent, Submenu};
use std::time::{Duration, Instant};
use winit::{
    dpi::LogicalSize,
    event::{ElementState, Event, MouseButton, WindowEvent},
    event_loop::{ControlFlow, EventLoop, EventLoopBuilder, EventLoopWindowTarget},
    window::{Window, WindowBuilder},
};

use crate::{
    menu::{init_menu, init_menu_hooks, show_context_menu_for_window, MenuDispatchMap},
    AppBuilder, RenderContext,
};

/// A wrapper around a winit Window and a muda Menu.
/// This struct is used to create and manage a window and menu bar.
/// It also provides a callback for handling your application loop.
pub struct AppRunner<T>
where
    T: App,
{
    app: T,
    menu_bar: Option<Menu>,
    context_menu: Option<Submenu>,
    menu_dispatch_map: MenuDispatchMap,
}

impl<T> AppRunner<T>
where
    T: App,
{
    /// Create new App with a menu bar.
    /// It should be called before any other menu-related functions.
    pub fn new(app: T) -> Self {
        Self {
            app,
            menu_bar: None,
            context_menu: None,
            menu_dispatch_map: MenuDispatchMap::new(),
        }
    }

    fn create_window(
        &mut self,
        width: u32,
        height: u32,
        event_loop: &EventLoop<()>,
    ) -> Result<Window> {
        let size = LogicalSize::new(width, height);
        let window = WindowBuilder::new()
            .with_title("App")
            .with_inner_size(size)
            .with_min_inner_size(size)
            .build(event_loop)?;

        if let Some(menu_bar) = self.menu_bar.as_ref() {
            init_menu(&window, menu_bar)?;
        }

        Ok(window)
    }

    fn pre_run_setup(&mut self) -> Result<(EventLoop<()>, RenderContext)> {
        let mut event_loop_builder = EventLoopBuilder::new();

        let mut builder = AppBuilder::new();
        self.app.init(&mut builder)?;

        self.menu_bar = builder.menu_bar;
        self.context_menu = builder.context_menu;
        self.menu_dispatch_map = builder.menu_dispatch_map;

        if let Some(menu_bar) = self.menu_bar.as_ref() {
            init_menu_hooks(&mut event_loop_builder, menu_bar);
        }

        let event_loop = event_loop_builder.build()?;

        let mut window = self
            .create_window(320, 240, &event_loop)
            .with_context(|| "Failed to create window")?;

        self.app.init_window(&mut window)?;

        let render_context =
            RenderContext::new(window, builder.target_frame_time, builder.max_frame_time);

        Ok((event_loop, render_context))
    }

    /// Run the application loop.
    /// This function will block until the application is closed, or an error occurs.
    pub fn run(&mut self) -> Result<()> {
        let (event_loop, mut render_context) = self.pre_run_setup()?;

        let mut current_time = Instant::now();
        let mut accumulated_time = Duration::ZERO;

        event_loop.set_control_flow(ControlFlow::Poll);
        event_loop.run(move |event, event_loop| {
            event_loop.set_control_flow(ControlFlow::Poll);

            // Let the app handle the event
            if handle_error(self.app.handle_event(&event), event_loop).is_err() {
                return;
            }

            // Handle menu events
            let menu_channel = MenuEvent::receiver();
            if let Ok(event) = menu_channel.try_recv() {
                if let Some(dispatch) = self.menu_dispatch_map.get(&event.id) {
                    dispatch();
                }
            }

            // Process any input events
            render_context.input.handle_event(&event);

            match event {
                Event::WindowEvent { event, .. } => match event {
                    WindowEvent::CloseRequested => event_loop.exit(),

                    WindowEvent::MouseInput {
                        state: ElementState::Pressed,
                        button: MouseButton::Right,
                        ..
                    } => {
                        if let Some(context_menu) = self.context_menu.as_ref() {
                            show_context_menu_for_window(render_context.window(), context_menu);
                        }
                    }

                    WindowEvent::RedrawRequested => {
                        let mut delta_time = current_time.elapsed();
                        current_time = Instant::now();

                        if delta_time > render_context.max_frame_time() {
                            delta_time = render_context.max_frame_time();
                        }

                        accumulated_time += delta_time;

                        render_context.input.update();

                        while accumulated_time >= render_context.target_frame_time() {
                            render_context.delta_time = render_context.target_frame_time();

                            if handle_error(self.app.update(&mut render_context), event_loop)
                                .is_err()
                            {
                                return;
                            }

                            accumulated_time -= render_context.target_frame_time();
                        }

                        if handle_error(self.app.render(&mut render_context), event_loop).is_err() {
                            #[allow(clippy::needless_return)]
                            // Although this return is not needed, it makes the code more readable.
                            return;
                        }
                    }

                    _ => {}
                },

                Event::AboutToWait => {
                    render_context.window().request_redraw();
                }

                _ => {}
            }
        })?;

        Ok(())
    }
}

fn handle_error(result: Result<()>, event_loop: &EventLoopWindowTarget<()>) -> Result<()> {
    if let Err(error) = &result {
        eprintln!("{}", error);
        event_loop.exit();
    }

    result
}

/// A trait for creating an application.
pub trait App: Sized {
    #[allow(unused_variables)]
    /// Initialize the app.
    /// Is called once, before the first update.
    /// Use this function to initialize any resources, or perform any setup.
    /// The AppBuilder can be used to configure the application before it is created.
    fn init(&mut self, builder: &mut AppBuilder) -> Result<()> {
        Ok(())
    }

    #[allow(unused_variables)]
    /// A hook called after the window is created.
    /// Use this function to initialize any resources that depend on the window.
    /// This is called after `init`, and before `update` and `render`.
    fn init_window(&mut self, window: &mut Window) -> Result<()> {
        Ok(())
    }

    /// Update the app.
    /// Is called in a loop, before `render`.
    /// It is guaranteed to be called at least once per minimum frame time,
    /// but may be called more than once per frame before `render` is called.
    /// The delta time is guaranteed to not exceed the maximum frame time.
    fn update(&mut self, context: &mut RenderContext) -> Result<()>;

    /// Render the app.
    /// Is called in a loop, after calls to `update` have finished.
    fn render(&mut self, context: &mut RenderContext) -> Result<()>;

    #[allow(unused_variables)]
    /// Handle a winit event.
    /// Is called before the any other event handling, and before `update` and `render`.
    fn handle_event(&mut self, event: &Event<()>) -> Result<()> {
        Ok(())
    }
}
