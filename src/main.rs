use anyhow::Result;
use ray_tracing::scenes;

fn main() -> Result<()> {
    scenes::two_perlin_spheres()
}
