pub type Color = glam::DVec3;

fn linear_to_gamma(linear_component: f64) -> f64 {
    linear_component.sqrt()
}

pub fn write_color(out: &mut impl std::io::Write, pixel_color: Color, samples_per_pixel: i32) {
    let scale = 1.0 / samples_per_pixel as f64;

    let r = (256.0 * linear_to_gamma(pixel_color.x * scale).clamp(0.0, 0.999)) as i32;
    let g = (256.0 * linear_to_gamma(pixel_color.y * scale).clamp(0.0, 0.999)) as i32;
    let b = (256.0 * linear_to_gamma(pixel_color.z * scale).clamp(0.0, 0.999)) as i32;

    write!(out, "{r} {g} {b}\n").expect("Failed to write color!");
}
