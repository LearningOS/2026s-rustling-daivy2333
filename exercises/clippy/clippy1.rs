use std::f32;

fn main() {
    let pi = 3.14f32;
    let radius = 5.00f32;

    // Clippy suggests: radius * radius is clearer than f32::powi(radius, 2)
    let area = pi * radius * radius;

    println!(
        "The area of a circle with radius {:.2} is {:.5}!",
        radius, area
    )
}