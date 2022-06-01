use crate::{
    event::{KeyboardEvent, UiEvent, UpdateDom, WindowEvent},
    setting::{DioxusSettings, UpdateMode},
    window::DioxusWindows,
};
use bevy::{
    app::{App, AppExit},
    ecs::{
        event::{Events, ManualEventReader},
        world::World,
    },
    input::{keyboard::KeyboardInput, mouse::MouseMotion},
    log::{info, warn},
    math::{ivec2, Vec2},
    utils::Instant,
    window::{
        CreateWindow, FileDragAndDrop, ReceivedCharacter, RequestRedraw,
        WindowBackendScaleFactorChanged, WindowCloseRequested, WindowFocused, WindowId, WindowMode,
        WindowMoved, WindowResized, WindowScaleFactorChanged, Windows,
    },
};
use futures_intrusive::channel::shared::{Receiver, Sender};
use std::fmt::Debug;
use tokio::runtime::Runtime;
use wry::application::{
    dpi::LogicalSize,
    event::{DeviceEvent, Event, StartCause, WindowEvent as TaoWindowEvent},
    event_loop::{ControlFlow, EventLoop, EventLoopWindowTarget},
};

pub fn runner<CoreCommand, UiCommand, Props>(mut app: App)
where
    CoreCommand: 'static + Send + Sync + Clone + Debug,
    UiCommand: 'static + Send + Sync + Clone + Debug,
    Props: 'static + Send + Sync + Clone + Default,
{
    let event_loop = app
        .world
        .remove_non_send_resource::<EventLoop<UiEvent<CoreCommand>>>()
        .unwrap();
    let core_rx = app
        .world
        .remove_resource::<Receiver<CoreCommand>>()
        .unwrap();
    let runtime = app.world.get_resource::<Runtime>().unwrap();
    let proxy = event_loop.create_proxy();

    let mut tao_state = TaoPersistentState::default();

    runtime.spawn(async move {
        while let Some(cmd) = core_rx.receive().await {
            proxy.clone().send_event(UiEvent::CoreCommand(cmd)).unwrap();
        }
    });

    event_loop.run(
        move |event: Event<UiEvent<CoreCommand>>,
              _event_loop: &EventLoopWindowTarget<UiEvent<CoreCommand>>,
              control_flow: &mut ControlFlow| {
            match event {
                Event::NewEvents(start) => {
                    let dioxus_settings = app.world.non_send_resource::<DioxusSettings<Props>>();
                    let windows = app.world.resource::<Windows>();
                    let focused = windows.iter().any(|w| w.is_focused());
                    let auto_timeout_reached =
                        matches!(start, StartCause::ResumeTimeReached { .. });
                    let now = Instant::now();
                    let manual_timeout_reached = match dioxus_settings.update_mode(focused) {
                        UpdateMode::Continuous => false,
                        UpdateMode::Reactive { max_wait }
                        | UpdateMode::ReactiveLowPower { max_wait } => {
                            now.duration_since(tao_state.last_update) >= *max_wait
                        }
                    };
                    tao_state.low_power_event = false;
                    tao_state.timeout_reached = auto_timeout_reached || manual_timeout_reached;
                }
                Event::WindowEvent {
                    event,
                    window_id: tao_window_id,
                    ..
                } => {
                    let world = app.world.cell();
                    let dioxus_windows = world.get_non_send_mut::<DioxusWindows>().unwrap();
                    let mut windows = world.get_resource_mut::<Windows>().unwrap();
                    let window_id =
                        if let Some(window_id) = dioxus_windows.get_window_id(tao_window_id) {
                            window_id
                        } else {
                            warn!(
                                "Skipped event for unknown winit Window Id {:?}",
                                tao_window_id
                            );
                            return;
                        };

                    let window = if let Some(window) = windows.get_mut(window_id) {
                        window
                    } else {
                        info!("Skipped event for closed window: {:?}", window_id);
                        return;
                    };
                    tao_state.low_power_event = true;

                    match event {
                        TaoWindowEvent::Resized(size) => {
                            window.update_actual_size_from_backend(size.width, size.height);
                            let mut resize_events =
                                world.get_resource_mut::<Events<WindowResized>>().unwrap();
                            resize_events.send(WindowResized {
                                id: window_id,
                                width: window.width(),
                                height: window.height(),
                            });
                        }
                        TaoWindowEvent::CloseRequested => {
                            let mut window_close_requested_events = world
                                .get_resource_mut::<Events<WindowCloseRequested>>()
                                .unwrap();
                            window_close_requested_events
                                .send(WindowCloseRequested { id: window_id });
                        }
                        // No event emitted. probably webview interrupts window underneath
                        // WindowEvent::KeyboardInput { event, .. } => {
                        //     println!("event: {:?}", event);
                        // }
                        // WindowEvent::CursorMoved { device_id, .. } => {
                        //     println!("device_id: {:?}", device_id);
                        // }
                        // WindowEvent::CursorEntered { device_id } => {
                        //     println!("device_id: {:?}", device_id);
                        // }
                        // WindowEvent::CursorLeft { device_id } => {
                        //     println!("device_id: {:?}", device_id);
                        // }
                        // WindowEvent::MouseInput { device_id, .. } => {
                        //     println!("device_id: {:?}", device_id);
                        // }
                        // WindowEvent::MouseWheel { device_id, .. } => {
                        //     println!("device_id: {:?}", device_id);
                        // }
                        // WindowEvent::Touch(touch) => {
                        //     println!("touch: {:?}", touch);
                        // }
                        // it doesn't event exist in tao or wry but in winit
                        // WindowEvent::ReceivedCharacter(char) => {
                        //     println!("char: {}", char);
                        // }
                        TaoWindowEvent::ScaleFactorChanged {
                            scale_factor,
                            new_inner_size,
                        } => {
                            let mut backend_scale_factor_change_events = world
                                .get_resource_mut::<Events<WindowBackendScaleFactorChanged>>()
                                .unwrap();
                            backend_scale_factor_change_events.send(
                                WindowBackendScaleFactorChanged {
                                    id: window_id,
                                    scale_factor,
                                },
                            );
                            let prior_factor = window.scale_factor();
                            window.update_scale_factor_from_backend(scale_factor);
                            let new_factor = window.scale_factor();
                            if let Some(forced_factor) = window.scale_factor_override() {
                                *new_inner_size = LogicalSize::new(
                                    window.requested_width(),
                                    window.requested_height(),
                                )
                                .to_physical::<u32>(forced_factor);
                            } else if approx::relative_ne!(new_factor, prior_factor) {
                                let mut scale_factor_change_events = world
                                    .get_resource_mut::<Events<WindowScaleFactorChanged>>()
                                    .unwrap();

                                scale_factor_change_events.send(WindowScaleFactorChanged {
                                    id: window_id,
                                    scale_factor,
                                });
                            }

                            let new_logical_width = new_inner_size.width as f64 / new_factor;
                            let new_logical_height = new_inner_size.height as f64 / new_factor;
                            if approx::relative_ne!(window.width() as f64, new_logical_width)
                                || approx::relative_ne!(window.height() as f64, new_logical_height)
                            {
                                let mut resize_events =
                                    world.get_resource_mut::<Events<WindowResized>>().unwrap();
                                resize_events.send(WindowResized {
                                    id: window_id,
                                    width: new_logical_width as f32,
                                    height: new_logical_height as f32,
                                });
                            }
                            window.update_actual_size_from_backend(
                                new_inner_size.width,
                                new_inner_size.height,
                            );
                        }
                        TaoWindowEvent::Focused(focused) => {
                            window.update_focused_status_from_backend(focused);
                            let mut focused_events =
                                world.get_resource_mut::<Events<WindowFocused>>().unwrap();
                            focused_events.send(WindowFocused {
                                id: window_id,
                                focused,
                            });
                        }
                        TaoWindowEvent::DroppedFile(path_buf) => {
                            let mut events =
                                world.get_resource_mut::<Events<FileDragAndDrop>>().unwrap();
                            events.send(FileDragAndDrop::DroppedFile {
                                id: window_id,
                                path_buf,
                            });
                        }
                        TaoWindowEvent::HoveredFile(path_buf) => {
                            let mut events =
                                world.get_resource_mut::<Events<FileDragAndDrop>>().unwrap();
                            events.send(FileDragAndDrop::HoveredFile {
                                id: window_id,
                                path_buf,
                            });
                        }
                        TaoWindowEvent::HoveredFileCancelled => {
                            let mut events =
                                world.get_resource_mut::<Events<FileDragAndDrop>>().unwrap();
                            events.send(FileDragAndDrop::HoveredFileCancelled { id: window_id });
                        }
                        TaoWindowEvent::Moved(position) => {
                            let position = ivec2(position.x, position.y);
                            window.update_actual_position_from_backend(position);
                            let mut events =
                                world.get_resource_mut::<Events<WindowMoved>>().unwrap();
                            events.send(WindowMoved {
                                id: window_id,
                                position,
                            });
                        }
                        _ => {}
                    }
                }
                Event::UserEvent(user_event) => match user_event {
                    UiEvent::WindowEvent(window_event) => {
                        let world = app.world.cell();
                        let mut windows = world.get_non_send_mut::<Windows>().unwrap();
                        let window = windows.get_primary_mut().unwrap();
                        let id = WindowId::primary();

                        let mut dioxus_windows = world.get_non_send_mut::<DioxusWindows>().unwrap();
                        let tao_window = dioxus_windows.get_tao_window(id).unwrap();

                        match window_event {
                            WindowEvent::Update => {
                                let dioxus_window = dioxus_windows.get_mut(id).unwrap();
                                dioxus_window.update();
                            }
                            WindowEvent::CloseWindow => {
                                let mut events = world
                                    .get_resource_mut::<Events<WindowCloseRequested>>()
                                    .unwrap();
                                events.send(WindowCloseRequested { id });
                            }
                            WindowEvent::DragWindow => {
                                if tao_window.fullscreen().is_none() {
                                    tao_window.drag_window().unwrap();
                                }
                            }
                            WindowEvent::Visible(visible) => {
                                tao_window.set_visible(visible);
                            }
                            WindowEvent::Minimize(minimized) => {
                                window.set_minimized(minimized);
                            }
                            WindowEvent::Maximize(maximized) => {
                                window.set_maximized(maximized);
                            }
                            WindowEvent::MaximizeToggle => {
                                let maximized = !tao_window.is_maximized();
                                tao_window.set_maximized(maximized);
                            }
                            WindowEvent::Fullscreen(_fullscreen) => {
                                let mode = match window.mode() {
                                    WindowMode::Windowed => WindowMode::Fullscreen,
                                    _ => WindowMode::Windowed,
                                };
                                window.set_mode(mode);
                            }
                            WindowEvent::FocusWindow => {
                                window.update_focused_status_from_backend(true);

                                let mut events =
                                    world.get_resource_mut::<Events<WindowFocused>>().unwrap();
                                events.send(WindowFocused { id, focused: true });
                            }
                            WindowEvent::Resizable(resizable) => {
                                window.set_resizable(resizable);
                            }
                            WindowEvent::AlwaysOnTop(allways_on_top) => {
                                tao_window.set_always_on_top(allways_on_top);
                            }
                            WindowEvent::CursorVisible(visible) => {
                                tao_window.set_cursor_visible(visible);
                            }
                            WindowEvent::CursorGrab(grab) => {
                                let _ = tao_window.set_cursor_grab(grab);
                            }
                            WindowEvent::SetTitle(title) => {
                                tao_window.set_title(&title);
                            }
                            WindowEvent::SetDecorations(decorations) => {
                                tao_window.set_decorations(decorations);
                            }
                            WindowEvent::SetZoomLevel(scale_factor) => {
                                let dioxus_window = dioxus_windows.get_mut(id).unwrap();
                                dioxus_window.webview.zoom(scale_factor);
                            }
                            WindowEvent::Print => {
                                let dioxus_window = dioxus_windows.get_mut(id).unwrap();
                                if let Err(e) = dioxus_window.webview.print() {
                                    log::warn!("Open print modal failed: {e}");
                                }
                            }
                            WindowEvent::DevTool => {
                                let dioxus_window = dioxus_windows.get_mut(id).unwrap();
                                dioxus_window.webview.open_devtools();
                            }
                            WindowEvent::Eval(code) => {
                                let dioxus_window = dioxus_windows.get_mut(id).unwrap();
                                dioxus_window
                                    .webview
                                    .evaluate_script(code.as_str())
                                    .expect("eval shouldn't panic");
                            }
                        };

                        let mut request_redraw =
                            world.get_resource_mut::<Events<RequestRedraw>>().unwrap();
                        request_redraw.send(RequestRedraw);
                    }
                    UiEvent::CoreCommand(cmd) => {
                        let mut events = app
                            .world
                            .get_resource_mut::<Events<CoreCommand>>()
                            .expect("Provide CoreCommand event to bevy");
                        events.send(cmd);
                    }
                    UiEvent::KeyboardEvent(event) => {
                        let mut keyboard_events = app
                            .world
                            .get_resource_mut::<Events<KeyboardEvent>>()
                            .unwrap();
                        keyboard_events.send(event.clone());
                        let mut keyboard_input_events = app
                            .world
                            .get_resource_mut::<Events<KeyboardInput>>()
                            .unwrap();
                        keyboard_input_events.send(event.to_input());
                        let mut request_redraw = app.world.resource_mut::<Events<RequestRedraw>>();
                        request_redraw.send(RequestRedraw);

                        match event.try_to_char() {
                            Some(c) => {
                                let mut received_character_events = app
                                    .world
                                    .get_resource_mut::<Events<ReceivedCharacter>>()
                                    .unwrap();
                                received_character_events.send(c);
                            }
                            None => {}
                        }
                    }
                },
                Event::DeviceEvent {
                    event: DeviceEvent::MouseMotion { delta, .. },
                    ..
                } => {
                    let mut mouse_motion_events = app.world.resource_mut::<Events<MouseMotion>>();
                    mouse_motion_events.send(MouseMotion {
                        delta: Vec2::new(delta.0 as f32, delta.1 as f32),
                    });
                }
                Event::Suspended => {
                    tao_state.active = false;
                }
                Event::Resumed => {
                    tao_state.active = true;
                }
                Event::MainEventsCleared => {
                    handle_create_window_events::<CoreCommand, UiCommand, Props>(&mut app.world);
                    let dioxus_settings = app.world.non_send_resource::<DioxusSettings<Props>>();
                    let update = if tao_state.active {
                        let windows = app.world.resource::<Windows>();
                        let focused = windows.iter().any(|w| w.is_focused());
                        match dioxus_settings.update_mode(focused) {
                            UpdateMode::Continuous { .. } => true,
                            UpdateMode::Reactive { .. } | UpdateMode::ReactiveLowPower { .. } => {
                                tao_state.low_power_event
                                    || tao_state.redraw_request_sent
                                    || tao_state.timeout_reached
                            }
                        }
                    } else {
                        false
                    };
                    let dom_update_tx = app.world.resource::<Sender<UpdateDom>>();

                    if update {
                        tao_state.last_update = Instant::now();

                        let _ = dom_update_tx.try_send(UpdateDom);
                        app.update();
                    }
                }
                Event::RedrawEventsCleared => {
                    {
                        let dioxus_settings =
                            app.world.non_send_resource::<DioxusSettings<Props>>();
                        let windows = app.world.non_send_resource::<Windows>();
                        let focused = windows.iter().any(|w| w.is_focused());
                        let now = Instant::now();
                        use UpdateMode::*;
                        *control_flow = match dioxus_settings.update_mode(focused) {
                            Continuous => ControlFlow::Poll,
                            Reactive { max_wait } | ReactiveLowPower { max_wait } => {
                                ControlFlow::WaitUntil(now + *max_wait)
                            }
                        };
                    }
                    let mut redraw = false;
                    if let Some(app_redraw_events) =
                        app.world.get_resource::<Events<RequestRedraw>>()
                    {
                        let mut redraw_event_reader = ManualEventReader::<RequestRedraw>::default();
                        if redraw_event_reader.iter(app_redraw_events).last().is_some() {
                            *control_flow = ControlFlow::Poll;
                            redraw = true;
                        }
                    }

                    if let Some(app_exit_events) = app.world.get_resource::<Events<AppExit>>() {
                        let mut app_exit_event_reader = ManualEventReader::<AppExit>::default();
                        if app_exit_event_reader.iter(app_exit_events).last().is_some() {
                            *control_flow = ControlFlow::Exit;
                        }
                    }
                    tao_state.redraw_request_sent = redraw;
                }
                _ => {}
            }
        },
    );
}

fn handle_create_window_events<CoreCommand, UiCommand, Props>(world: &mut World)
where
    CoreCommand: 'static + Send + Sync + Clone + Debug,
    UiCommand: 'static + Send + Sync + Clone + Debug,
    Props: 'static + Send + Sync + Clone,
{
    let world = world.cell();
    // let mut dioxus_windows = world.get_non_send_mut::<DioxusWindows>().unwrap();
    // let mut windows = world.get_resource_mut::<Windows>().unwrap();
    let create_window_events = world.get_resource::<Events<CreateWindow>>().unwrap();
    let mut create_window_events_reader = ManualEventReader::<CreateWindow>::default();
    // let mut window_created_events = world.get_resource_mut::<Events<WindowCreated>>().unwrap();

    for _create_window_event in create_window_events_reader.iter(&create_window_events) {
        warn!("Multiple Windows isn't supported yet!");
        //     let window = dioxus_windows.create::<CoreCommand, UiCommand, Props>(
        //         &world,
        //         create_window_event.id,
        //         &create_window_event.descriptor,
        //     );
        //     windows.add(window);
        //     window_created_events.send(WindowCreated {
        //         id: create_window_event.id,
        //     });
    }
}

struct TaoPersistentState {
    active: bool,
    low_power_event: bool,
    redraw_request_sent: bool,
    timeout_reached: bool,
    last_update: Instant,
}

impl Default for TaoPersistentState {
    fn default() -> Self {
        Self {
            active: true,
            low_power_event: false,
            redraw_request_sent: false,
            timeout_reached: false,
            last_update: Instant::now(),
        }
    }
}
