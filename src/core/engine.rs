use super::timer::Timer;
use crate::{rendering::renderer::Renderer, window::DaedalusWindow};
use shipyard::{borrow::NonSendSync, UniqueViewMut, Workload, World};
use std::{sync::Arc, time::Instant};

const INITIAL_WINDOW_WIDTH: u32 = 1280;
const INITIAL_WINDOW_HEIGHT: u32 = 720;

pub struct Engine {
    pub(crate) world: Arc<World>,
    pub(crate) window: DaedalusWindow,
}

impl Engine {
    pub fn new(window: DaedalusWindow) -> Self {
        let timer = Timer::new();
        let world = World::default();
        let renderer = Renderer::new(&window);

        world.add_unique(timer);
        world.add_unique_non_send_sync(renderer);

        Workload::new("TICK")
            .with_system(update)
            .with_system(render)
            .add_to_world(&world)
            .unwrap();

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
    // mut renderer: NonSendSync<UniqueViewMut<RendererEntity>>,
    mut timer: UniqueViewMut<Timer>,
) {
    let delta_time = timer.get_delta_time();
    println!("delta_time: {:?}", delta_time);

    // renderer.0.update(dt);
}

pub fn render(
    mut renderer: NonSendSync<UniqueViewMut<Renderer>>,
    // transforms: View<TransformComponent>,
    // mesh_ids: View<MeshEntityId>,
    // meshes: View<StaticMesh>,
) {
    // let mut renderable = vec![];

    // for (transform, mesh_id) in (&transforms, &mesh_ids).iter() {
    //     renderable.push((&transform.0, meshes.get(mesh_id.0).unwrap().0.clone()));
    // }

    renderer.tick();
}
