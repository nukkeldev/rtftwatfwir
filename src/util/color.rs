pub type Color = glam::DVec3;

fn linear_to_gamma(linear_component: f64) -> f64 {
    linear_component.sqrt()
}

pub fn convert_color(pixel_color: Color, samples_per_pixel: i32) -> [u8; 3] {
    let scale = 1.0 / samples_per_pixel as f64;

    let r = (256.0 * linear_to_gamma(pixel_color.x * scale).clamp(0.0, 0.999)) as u8;
    let g = (256.0 * linear_to_gamma(pixel_color.y * scale).clamp(0.0, 0.999)) as u8;
    let b = (256.0 * linear_to_gamma(pixel_color.z * scale).clamp(0.0, 0.999)) as u8;

    [r, g, b]
}

pub fn write_color(out: &mut impl std::io::Write, pixel_color: Color, samples_per_pixel: i32) {
    let rgb = convert_color(pixel_color, samples_per_pixel);
    write!(out, "{} {} {}\n", rgb[0], rgb[1], rgb[2]).expect("Failed to write color!");
}
