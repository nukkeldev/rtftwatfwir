use anyhow::Result;

use ray_tracing::scenes;

fn main() -> Result<()> {
    rayon::ThreadPoolBuilder::new()
        .num_threads(8)
        .build_global()
        .unwrap();

    scenes::random_spheres()
}
