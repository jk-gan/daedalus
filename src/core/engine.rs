use super::timer::Timer;
use crate::{
    rendering::{renderer::Renderer, shape::cube::Cube},
    scene::SceneManager,
    window::DaedalusWindow,
};
use glam::Vec3;
use shipyard::{
    borrow::NonSendSync, Component, EntitiesViewMut, IntoIter, UniqueView, UniqueViewMut, View,
    ViewMut, Workload, World,
};
use std::{path::Path, sync::Arc};
use uuid::Uuid;

pub struct Engine {
    pub(crate) world: Arc<World>,
    pub(crate) window: DaedalusWindow,
}

impl Engine {
    pub fn new(window: DaedalusWindow) -> Self {
        let draw_size = window.winit_window.inner_size();

        let world = World::default();

        let scene_manager = SceneManager::new(draw_size.width as f32, draw_size.height as f32);
        let timer = Timer::new();
        let renderer = Renderer::new(&window);

        world.add_unique(timer);
        world.add_unique_non_send_sync(scene_manager);
        world.add_unique_non_send_sync(renderer);

        Workload::new("TICK")
            .with_system(update)
            .with_system(render)
            .add_to_world(&world)
            .unwrap();

        world.run_with_data(
            import_mesh,
            (
                std::path::Path::new("assets/meshes/SciFiHelmet/glTF/SciFiHelmet.gltf"),
                TransformComponent {
                    scale: Vec3::new(1.0, 1.0, 1.0),
                    position: Vec3::new(0.0, 2.5, 0.0),
                    ..Default::default()
                },
            ),
        );

        world.run_with_data(
            add_cube,
            TransformComponent {
                scale: Vec3::new(10.0, 0.5, 10.0),
                ..Default::default()
            },
        );

        Self {
            world: Arc::new(world),
            window,
        }
    }

    pub fn start_game_loop(self) {
        self.window.start_game_loop(self.world.clone());
    }
}

pub fn update(
    mut timer: UniqueViewMut<Timer>,
    mut scene_manager: NonSendSync<UniqueViewMut<SceneManager>>,
) {
    let delta_time = timer.get_delta_time();
    println!("delta_time: {:?}", delta_time);

    scene_manager.get_current_scene_mut().update(delta_time);

    // renderer.0.update(dt);
}

pub fn render(
    mut renderer: NonSendSync<UniqueViewMut<Renderer>>,
    mut scene_manager: NonSendSync<UniqueViewMut<SceneManager>>,
    meshes: View<MeshComponent>,
    transforms: View<TransformComponent>,
) {
    let mut renderables = Vec::new();

    for (mesh, transform) in (&meshes, &transforms).iter() {
        renderables.push((&mesh.id, transform));
    }

    let current_scene = scene_manager.get_current_scene_mut();
    renderer.tick(renderables, current_scene);
}

pub fn import_mesh(
    (file_path, transform): (&Path, TransformComponent),
    renderer: NonSendSync<UniqueView<Renderer>>,
    mut scene_manager: NonSendSync<UniqueViewMut<SceneManager>>,
    mut meshes: ViewMut<MeshComponent>,
    mut transforms: ViewMut<TransformComponent>,
    mut entities: EntitiesViewMut,
) {
    let mesh_id = scene_manager.get_current_scene_mut().add_mesh(
        &file_path,
        &renderer.device,
        &renderer.command_queue,
    );

    entities.add_entity(
        (&mut meshes, &mut transforms),
        (MeshComponent { id: mesh_id }, transform),
    );
}

pub fn add_cube(
    transform: TransformComponent,
    renderer: NonSendSync<UniqueView<Renderer>>,
    mut scene_manager: NonSendSync<UniqueViewMut<SceneManager>>,
    mut meshes: ViewMut<MeshComponent>,
    mut transforms: ViewMut<TransformComponent>,
    mut entities: EntitiesViewMut,
) {
    let cube = Cube::new();

    let mesh_id = scene_manager.get_current_scene_mut().add_shape(
        &renderer.device,
        &cube.vertices,
        &cube.normals,
        &cube.indices,
    );

    entities.add_entity(
        (&mut meshes, &mut transforms),
        (MeshComponent { id: mesh_id }, transform),
    );
}

#[derive(Component)]
pub struct MeshComponent {
    id: Uuid,
}

#[derive(Component, Default)]
pub struct TransformComponent {
    pub position: Vec3,
    pub rotation: Vec3,
    pub scale: Vec3,
}
