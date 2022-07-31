use shipyard::{NonSendSync, UniqueViewMut, World};
use std::sync::Arc;
use winit::{
    dpi::LogicalSize,
    event::{DeviceEvent, ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::{Window as WinitWindow, WindowBuilder},
};

use crate::scene::SceneManager;

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
        let mut current_scale_facor = 1.0;
        let mut left_mouse_pressed = false;

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
                    WindowEvent::KeyboardInput {
                        input:
                            KeyboardInput {
                                state,
                                virtual_keycode: Some(key),
                                ..
                            },
                        ..
                    } => {
                        world
                            .borrow::<NonSendSync<UniqueViewMut<SceneManager>>>()
                            .unwrap()
                            .get_current_scene_mut()
                            .handle_keyboard_event(*key, *state);
                    }
                    WindowEvent::Resized(size) => {
                        // TODO: Handle Window Resize
                        let logical_size = size.to_logical::<f32>(current_scale_facor);

                        world
                            .borrow::<NonSendSync<UniqueViewMut<SceneManager>>>()
                            .unwrap()
                            .get_current_scene_mut()
                            .resize(logical_size.width as f32, logical_size.height as f32);
                    }
                    WindowEvent::ScaleFactorChanged {
                        new_inner_size,
                        scale_factor,
                    } => {
                        // TODO: Handle Scale Factor Changed
                        current_scale_facor = scale_factor.clone();

                        world
                            .borrow::<NonSendSync<UniqueViewMut<SceneManager>>>()
                            .unwrap()
                            .get_current_scene_mut()
                            .resize(new_inner_size.width as f32, new_inner_size.height as f32);
                    }
                    _ => {}
                },
                // Event::NewEvents(_) => todo!(),
                // Event::RedrawEventsCleared => todo!(),
                Event::DeviceEvent {
                    device_id: _,
                    event,
                } => match event {
                    DeviceEvent::Key(KeyboardInput {
                        state: _state,
                        virtual_keycode: Some(_key),
                        ..
                    }) => {
                        // renderer.handle_keyboard_event(*key, *state);
                    }
                    DeviceEvent::MouseWheel { delta, .. } => {
                        world
                            .borrow::<NonSendSync<UniqueViewMut<SceneManager>>>()
                            .unwrap()
                            .get_current_scene_mut()
                            .handle_scroll_event(&delta);
                    }
                    DeviceEvent::MouseMotion { delta } => {
                        if left_mouse_pressed {
                            //renderer.handle_mouse_event(delta.0, delta.1);
                            world
                                .borrow::<NonSendSync<UniqueViewMut<SceneManager>>>()
                                .unwrap()
                                .get_current_scene_mut()
                                .handle_mouse_event(delta.0, delta.1);
                        }
                    }
                    DeviceEvent::Button {
                        button: 1, // Right Mouse Button, 3 for windows
                        state,
                    } => {
                        println!("mouse state: {:?}", state);
                        if state == ElementState::Pressed {
                            left_mouse_pressed = true;
                            self.winit_window.set_cursor_grab(true).unwrap();
                            self.winit_window.set_cursor_visible(false);

                            // let mesh_id = world
                            //     .borrow::<NonSendSync<UniqueViewMut<RendererEntity>>>()
                            //     .unwrap()
                            //     .0
                            //     .upload_mesh(
                            //         &cube.vertices,
                            //         &cube.colors,
                            //         &cube.normals,
                            //         &cube.indices,
                            //     );

                            // let static_meshes = StaticMesh(mesh_id);
                            // let mesh_entity_id = world.add_entity((static_meshes,));
                            // let render_object = RenderObject::new(
                            //     Mat4::IDENTITY
                            //         .mul_mat4(&Mat4::from_translation(Vec3::new(x, 0.0, 0.0))),
                            // );

                            // x += 2.0;

                            // world.add_entity((
                            //     RenderObjectComponent(render_object),
                            //     MeshEntityId(mesh_entity_id),
                            // ));
                        } else {
                            left_mouse_pressed = false;
                            self.winit_window.set_cursor_grab(false).unwrap();
                            self.winit_window.set_cursor_visible(true);
                        }
                    }
                    _ => {}
                },
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
