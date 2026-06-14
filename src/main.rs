extern crate alloc;
extern crate core;

use crate::block::Blocks;
use crate::client::camera::Camera;
use crate::client::engine::GraphicsEngine;
use crate::client::input::Input;
use crate::client::resources::ResourceManager;
use crate::level::Level;
use crate::util::timer::{FrameRateLimit, Timer};
use itertools::Itertools;
use log::info;
use std::num::NonZero;
use winit::application::ApplicationHandler;
use winit::event::{DeviceEvent, DeviceId, StartCause, WindowEvent};
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::window::WindowId;

mod block;
mod client;
mod math;
mod util;
mod level;

enum Game {
    Uninit,
    Init(GameData),
}

struct GameData {
    graphics: GraphicsEngine,
    input: Input,
    camera: Camera,
    timer: Timer,
    level: Level<12>,
    resource_manager: ResourceManager,
}

impl ApplicationHandler for Game {
    fn new_events(&mut self, event_loop: &ActiveEventLoop, cause: StartCause) {
        match cause {
            StartCause::Init => {
                info!("Init");
                let resource_manager = ResourceManager::new();
                let engine = GraphicsEngine::new(&event_loop, &resource_manager.get_texture_manager());
                let mut level = Level::new(-4);
                level.generate_terrain();
                *self = Game::Init(GameData {
                    graphics: engine,
                    input: Input::new(),
                    camera: Camera::new(),
                    timer: Timer::new(NonZero::new(20).unwrap(), FrameRateLimit::Unlimited),
                    level,
                    resource_manager,
                });
                event_loop.set_control_flow(ControlFlow::Poll);
            }
            StartCause::Poll => {
                if let Game::Init(data) = self {
                    data.timer.wait(&data.graphics.get_window());
                }
            }
            _ => {}
        }
    }

    fn resumed(&mut self, _event_loop: &ActiveEventLoop) {
        if let Game::Init(data) = self {
            data.graphics.grab_mouse(true);
        }
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _window_id: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::Resized(_) => {
                if let Game::Init(data) = self {
                    data.graphics.set_window_should_resize(true);
                }
            }
            WindowEvent::Moved(_) => {}
            WindowEvent::CloseRequested => {
                info!("Close requested");
                event_loop.exit();
            }
            WindowEvent::Destroyed => {}
            WindowEvent::DroppedFile(_) => {}
            WindowEvent::HoveredFile(_) => {}
            WindowEvent::HoveredFileCancelled => {}
            WindowEvent::Focused(focused) => {
                if let Game::Init(data) = self {
                    data.graphics.set_window_focused(focused);
                }
            }
            WindowEvent::KeyboardInput {
                event,
                ..
            } => {
                if let Game::Init(data) = self {
                    data.input.process_input(event);
                }
            }
            WindowEvent::ModifiersChanged(_) => {}
            WindowEvent::Ime(_) => {}
            WindowEvent::CursorMoved { .. } => {}
            WindowEvent::CursorEntered { .. } => {}
            WindowEvent::CursorLeft { .. } => {}
            WindowEvent::MouseWheel { .. } => {}
            WindowEvent::MouseInput {
                device_id: _,
                button,
                state
            } => {
                if let Game::Init(data) = self {
                    data.input.process_mouse_button(button, state);
                }
            }
            WindowEvent::RedrawRequested => {
                if let Game::Init(data) = self {
                    data.timer.try_tick(|| {
                        data.input.tick(&mut data.camera, &mut data.graphics);
                    });
                    data.timer.try_frame(|partial_tick| {
                        let engine = &mut data.graphics;
                        let min_y_section = data.level.get_min_y_section();
                        let meshes = data.level.get_chunks_mut().flat_map(|(pos, c)| c.get_sections_mut().iter_mut().flat_map(|s| s.remesh(*pos, min_y_section, data.resource_manager.get_model_manager()))).collect::<Vec<_>>();
                        engine.update_section_meshes(meshes);
                        data.camera.adjust(engine.get_window().inner_size(), partial_tick);
                        engine.update_fps(data.camera.get_pos());
                        engine.resize_or_update_swapchain();
                        engine.render_game(&data.level, &data.camera);
                    });
                }
            }
            _ => {}
        }
    }

    fn device_event(&mut self, _event_loop: &ActiveEventLoop, _device_id: DeviceId, event: DeviceEvent) {
        match event {
            DeviceEvent::Added => {}
            DeviceEvent::Removed => {}
            DeviceEvent::MouseMotion {
                delta
            } => {
                if let Game::Init(data) = self {
                    if data.graphics.is_window_focused() && data.graphics.is_mouse_grabbed() {
                        data.input.process_mouse_motion(delta);
                    }
                }
            }
            DeviceEvent::MouseWheel { .. } => {}
            DeviceEvent::Motion { .. } => {}
            DeviceEvent::Button { .. } => {}
            DeviceEvent::Key(_) => {}
        }
    }
}

#[allow(unstable_name_collisions)]
fn main() {
    unsafe {
        //SAFETY: called from a single threaded environment
        std::env::set_var("RUST_LOG", "debug");
    }
    env_logger::builder().format_source_path(true).format_target(false).init();
    info!("Initializing Evolution VK");
    info!("{}", Blocks::all().map(|b| b.get_name_id()).intersperse(", ").collect::<String>());
    let event_loop = EventLoop::new().unwrap();
    let mut game = Game::Uninit;
    event_loop.run_app(&mut game).unwrap();
    info!("Back to main");
}