extern crate core;

use crate::block::Blocks;
use crate::chunk::Chunk;
use crate::client::camera::Camera;
use crate::client::engine::GraphicsEngine;
use crate::client::input::Input;
use crate::client::mesh::{Mesh, MeshBuilder};
use crate::client::texture::TextureManager;
use crate::client::vertex::{Vertex, VertexPosCol};
use crate::math::chunk_pos::ChunkPos;
use crate::math::mat4::Mat4;
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
mod chunk;

enum Game {
    Uninit,
    Init(GameData),
}

struct GameData {
    graphics: GraphicsEngine,
    input: Input,
    camera: Camera,
    timer: Timer,
    col_meshes: Vec<Mesh<VertexPosCol>>,
    chunk: Chunk<4>,
    texture_manager: TextureManager,
}

impl ApplicationHandler for Game {
    fn new_events(&mut self, event_loop: &ActiveEventLoop, cause: StartCause) {
        match cause {
            StartCause::Init => {
                info!("Init");
                let vc1 = Vertex::new().pos(1.0, 0.0, -1.0).color(1.0, 0.0, 0.0);
                let vc2 = Vertex::new().pos(1.0, 1.0, -1.0).color(0.0, 1.0, 0.0);
                let vc3 = Vertex::new().pos(0.0, 1.0, -1.0).color(0.0, 0.0, 1.0);
                let engine = GraphicsEngine::new(&event_loop);
                let allocator = engine.get_allocator().clone();
                let chunk = Chunk::new(ChunkPos::new(0, 0));
                *self = Game::Init(GameData {
                    graphics: engine,
                    input: Input::new(),
                    camera: Camera::new(),
                    timer: Timer::new(NonZero::new(20).unwrap(), FrameRateLimit::Unlimited),
                    col_meshes: vec![
                        MeshBuilder::new(Mat4::IDENTITY).triangle([vc1, vc2, vc3]).build(allocator.clone()).unwrap(),
                    ],
                    chunk,
                    texture_manager: TextureManager::new(),
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
                        let pos = data.chunk.get_pos();
                        data.chunk.get_sections_mut().iter_mut().for_each(|s| s.remesh(pos, engine.get_allocator().clone()));
                        data.camera.adjust(engine.get_window().inner_size(), partial_tick);
                        engine.update_fps();
                        engine.resize_or_update_swapchain();
                        engine.swap_buffers(
                            ((data.camera.get_view(), data.camera.get_proj()).into(), data.chunk.get_sections().iter().map(|s| s.get_mesh()).flatten()),
                            ((data.camera.get_view(), data.camera.get_proj()).into(), &data.col_meshes),
                        );
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
        std::env::set_var("RUST_LOG", "info");
    }
    env_logger::builder().format_source_path(true).format_target(false).init();
    info!("Initializing Evolution VK");
    let x = Blocks::all().map(|b| b.get_name_id()).intersperse(", ").collect::<String>();
    info!("{}", x);
    let event_loop = EventLoop::new().unwrap();
    let mut game = Game::Uninit;
    event_loop.run_app(&mut game).unwrap();
    info!("Back to main");
}