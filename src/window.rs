use shipyard::World;
use std::sync::Arc;
use winit::{
    dpi::LogicalSize,
    event::{ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::{Window as WinitWindow, WindowBuilder},
};

pub struct DaedalusWindow {
    event_loop: EventLoop<()>,
    pub winit_window: WinitWindow,
}

impl DaedalusWindow {
    pub fn new(title: &str, width: u32, height: u32) -> Self {
        let event_loop = EventLoop::new();
        let size = LogicalSize::new(width, height);
        let winit_window = WindowBuilder::new()
            .with_inner_size(size)
            .with_title(title)
            .build(&event_loop)
            .unwrap();

        Self {
            event_loop,
            winit_window,
        }
    }

    pub fn start_game_loop(self, world: Arc<World>) {
        self.event_loop.run(move |event, _, control_flow| {
            *control_flow = ControlFlow::Poll;

            match event {
                Event::MainEventsCleared => self.winit_window.request_redraw(),
                Event::WindowEvent {
                    ref event,
                    window_id,
                } if window_id == self.winit_window.id() => match event {
                    WindowEvent::CloseRequested
                    | WindowEvent::KeyboardInput {
                        input:
                            KeyboardInput {
                                state: ElementState::Pressed,
                                virtual_keycode: Some(VirtualKeyCode::Escape),
                                ..
                            },
                        ..
                    } => *control_flow = ControlFlow::Exit,
                    _ => {}
                },
                // Event::NewEvents(_) => todo!(),
                // Event::RedrawEventsCleared => todo!(),
                // Event::DeviceEvent { device_id, event } => todo!(),
                // Event::UserEvent(_) => todo!(),
                // Event::Suspended => todo!(),
                // Event::Resumed => todo!(),
                Event::RedrawRequested(_) => {
                    world.run_workload("TICK").unwrap();
                }
                Event::LoopDestroyed => {
                    // Visualize the ECS
                    std::fs::write(
                        "ecs.json",
                        serde_json::to_string(&world.workloads_type_usage()).unwrap(),
                    )
                    .unwrap();
                }
                _ => {}
            }
        });
    }
}
