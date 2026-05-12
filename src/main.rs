use kiss3d::prelude::*;

// Ideally, these variables should by "dynamic".
// This is something to take note of in the future.
//
// SPACING should be greater than LINE_WIDTH.

/// Grid is square and has edges:
/// (r, r), (-r, r), (r, -r), (-r, -r)
const RADIUS: f32 = 250.0;

/// Spacing between grid lines.
const SPACING: f32 = 25.0;

/// Width of grid lines in pixels.
const LINE_WIDTH: f32 = 1.0;

#[kiss3d::main]
async fn main() {
    let mut window = Window::new("2D Grid").await;
    let mut camera = OrbitCamera3d::default();
    let mut scene = SceneNode3d::empty();

    camera.set_dist(RADIUS * 3.0);

    // Draws a 2D grid.
    //
    // In the future, this should be a 3D grid. A toggle for a
    // "2D mode" would lock the camera and hide z grid lines.
    while window.render_3d(&mut scene, &mut camera).await {
        eprintln!("{}", camera.dist());
        let mut i = -RADIUS;

        // Draws the grid lines.
        while i <= RADIUS {
            // Horizontal.
            window.draw_line(
                Vec3::new(-RADIUS, i, 0.0),
                Vec3::new(RADIUS, i, 0.0),
                WHITE,
                LINE_WIDTH,
                false,
            );

            // Vertical.
            window.draw_line(
                Vec3::new(i, -RADIUS, 0.0),
                Vec3::new(i, RADIUS, 0.0),
                WHITE,
                LINE_WIDTH,
                false,
            );

            i += SPACING;
        }
    }
}
