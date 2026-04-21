use crate::client::camera::Camera;
use crate::client::engine::GraphicsEngine;
use crate::client::input::Input;
use crate::client::vertex::{VPTUniformTransform, Vertex, VertexPosTex};
use crate::math::mat4::Mat4;
use crate::util::timer::{FrameRateLimit, Timer};
use log::info;
use std::num::NonZero;
use winit::application::ApplicationHandler;
use winit::event::{DeviceEvent, DeviceId, StartCause, WindowEvent};
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::window::WindowId;

mod util;
mod math;
mod client;

enum Game {
    Uninit,
    Init(GameData),
}

struct GameData {
    graphics: GraphicsEngine<VPTUniformTransform, VertexPosTex>,
    input: Input,
    camera: Camera,
    timer: Timer,
}

impl ApplicationHandler for Game {
    fn new_events(&mut self, event_loop: &ActiveEventLoop, cause: StartCause) {
        match cause {
            StartCause::Init => {
                info!("Init");
                // let v1 = Vertex::new().pos(0.5, 0.5, -1.0).color(1.0, 0.0, 0.0);
                // let v2 = Vertex::new().pos(0.5, 0.0, -1.0).color(0.0, 1.0, 0.0);
                // let v3 = Vertex::new().pos(0.0, 0.5, -1.0).color(0.0, 0.0, 1.0);
                let v1 = Vertex::new().pos(0.5, 0.5, -1.0).uv(1.0, 0.0);
                let v2 = Vertex::new().pos(0.5, 0.0, -1.0).uv(1.0, 1.0);
                let v3 = Vertex::new().pos(0.0, 0.5, -1.0).uv(0.0, 0.0);
                *self = Game::Init(GameData {
                    graphics: GraphicsEngine::new(&event_loop, vec![v1, v2, v3]),
                    input: Input::new(),
                    camera: Camera::new(),
                    timer: Timer::new(NonZero::new(20).unwrap(), FrameRateLimit::Unlimited),
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
                        data.input.tick(&mut data.camera, || data.graphics.toggle_grab_mouse());
                    });
                    data.timer.try_frame(|partial_tick| {
                        let engine = &mut data.graphics;
                        data.camera.adjust(engine.get_window().inner_size(), partial_tick);
                        engine.update_fps();
                        engine.update_swapchain();
                        engine.swap_buffers(VertexPosTex::new_uniform(Mat4::IDENTITY, data.camera.get_view(), data.camera.get_proj()));
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

fn main() {
    unsafe {
        std::env::set_var("RUST_LOG", "info");
    }
    env_logger::init();
    info!("Initializing Evolution VK");
    let event_loop = EventLoop::new().unwrap();
    let mut game = Game::Uninit;
    event_loop.run_app(&mut game).unwrap();
    info!("Back to main");
}