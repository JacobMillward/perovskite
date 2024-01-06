use anyhow::{Context, Result};
use muda::MenuEvent;
use std::time::{Duration, Instant};
use winit::{
    dpi::LogicalSize,
    event::{ElementState, Event, MouseButton, WindowEvent},
    event_loop::{ControlFlow, EventLoop, EventLoopBuilder, EventLoopWindowTarget},
    window::{Window, WindowBuilder},
};

use crate::{
    menu::{init_menu, init_menu_hooks, show_context_menu_for_window},
    AppSettings, RenderContext,
};

/// A trait for creating an application, utilising a fixed timestep.
pub trait App: Sized {
    /// Initialize the app.
    /// Is called once, before the first update.
    /// Use this function to initialize any resources, or perform any setup.
    /// It should return the AppSettings for the app, which will be used to create the window.
    fn init(&mut self) -> Result<AppSettings> {
        Ok(AppSettings::builder().build())
    }

    /// Update the app.
    /// Is called in a loop, before `draw`.
    /// It is guaranteed to be called at least once per minimum frame time,
    /// but may be called more than once per frame before `draw` is called.
    /// The delta time is guaranteed to not exceed the maximum frame time.
    fn update(&mut self, ctx: &mut RenderContext) -> Result<()>;

    /// Render the app.
    /// Is called in a loop, after calls to `update` have finished.
    fn draw(&mut self, ctx: &mut RenderContext) -> Result<()>;

    #[allow(unused_variables)]
    /// Handle a winit event.
    /// Is called before the any other event handling, and before `update` and `draw`.
    fn handle_event(&mut self, event: &Event<()>) -> Result<()> {
        Ok(())
    }

    fn run(mut app: Self) -> Result<()> {
        let mut event_loop_builder = EventLoopBuilder::new();

        let settings = app.init()?;

        if let Some(menu_bar) = settings.menu_bar.as_ref() {
            init_menu_hooks(&mut event_loop_builder, menu_bar);
        }

        let event_loop = event_loop_builder.build()?;

        let window =
            create_window(&settings, &event_loop).with_context(|| "Failed to create window")?;

        let mut render_context = RenderContext::new(
            window,
            settings.target_frame_time,
            settings.max_frame_time,
            settings.frame_width,
            settings.frame_height,
        )?;

        let mut current_time = Instant::now();
        let mut accumulated_time = Duration::ZERO;
        let mut skip_update = false;

        event_loop.set_control_flow(ControlFlow::Poll);
        event_loop.run(move |event, event_loop| {
            event_loop.set_control_flow(ControlFlow::Poll);

            // Let the app handle the event
            if handle_error(app.handle_event(&event), event_loop).is_err() {
                return;
            }

            // Handle menu events
            let menu_channel = MenuEvent::receiver();
            if let Ok(event) = menu_channel.try_recv() {
                if let Some(dispatch) = settings.menu_dispatch_map.get(&event.id) {
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
                        if let Some(context_menu) = settings.context_menu.as_ref() {
                            show_context_menu_for_window(render_context.window(), context_menu);
                        }
                    }

                    WindowEvent::Resized(size) => {
                        let pixels = render_context.pixels_mut();

                        let resize_result = pixels
                            .resize_surface(size.width, size.height)
                            .with_context(|| {
                                format!(
                                    "Failed to resize pixels surface to {}x{}",
                                    size.width, size.height
                                )
                            });

                        if handle_error(resize_result, event_loop).is_err() {
                            #[allow(clippy::needless_return)]
                            return;
                        }

                        // Skip the next update, as the redraw event will be sent immediately after this one
                        skip_update = true;
                    }

                    WindowEvent::RedrawRequested => {
                        if !skip_update {
                            let mut delta_time = current_time.elapsed();
                            current_time = Instant::now();

                            if delta_time > render_context.max_frame_time() {
                                delta_time = render_context.max_frame_time();
                            }

                            accumulated_time += delta_time;

                            render_context.input.update();

                            while accumulated_time >= render_context.target_frame_time() {
                                render_context.delta_time = render_context.target_frame_time();

                                if handle_error(app.update(&mut render_context), event_loop)
                                    .is_err()
                                {
                                    return;
                                }

                                accumulated_time -= render_context.target_frame_time();
                            }
                        } else {
                            skip_update = false;
                        }

                        if handle_error(app.draw(&mut render_context), event_loop).is_err() {
                            #[allow(clippy::needless_return)]
                            return;
                        }
                    }

                    _ => {}
                },

                Event::AboutToWait => {
                    render_context.window().request_redraw();
                }

                _ => {}
            };
        })?;

        Ok(())
    }
}

fn create_window(settings: &AppSettings, event_loop: &EventLoop<()>) -> Result<Window> {
    let size = LogicalSize::new(
        settings.window_width.unwrap_or(settings.frame_width),
        settings.window_height.unwrap_or(settings.frame_height),
    );
    let min_size = LogicalSize::new(settings.frame_width, settings.frame_height);
    let window = WindowBuilder::new()
        .with_title(&settings.window_title)
        .with_inner_size(size)
        .with_min_inner_size(min_size)
        .build(event_loop)?;

    if let Some(menu) = settings.menu_bar.as_ref() {
        init_menu(&window, menu)?;
    }

    Ok(window)
}

fn handle_error<T>(result: Result<T>, event_loop: &EventLoopWindowTarget<()>) -> Result<T> {
    if let Err(error) = &result {
        eprintln!("{}", error);
        event_loop.exit();
    }
    result
}
