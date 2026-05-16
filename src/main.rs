use three_d::{
    Camera, ClearState, ColorMaterial, CpuMesh, FrameOutput, Geometry, Gm, Indices, Mesh,
    Positions, Quat, Rotation3, Srgba, Window, WindowSettings, degrees, radians, vec3,
};

/// Width of grid lines in units.
const LINE_WIDTH: f32 = 1.0;

/// Rotates a triangle.
///
/// # Panics
///
/// Window failed to be created.
pub fn main() {
    let window = Window::new(WindowSettings {
        title: "Triangle!".to_string(),
        max_size: Some((1280, 720)),
        ..Default::default()
    })
    .unwrap();

    let ctx = window.gl();

    // A "perspective" camera (as opposed to an orthographic one) is
    // often preferred as it's more natural to the eye. Though, for a
    // graphing tool, orthographic may be desired for its consistency.
    let mut camera = Camera::new_perspective(
        window.viewport(),
        vec3(0.0, 0.0, 5.0),
        vec3(0.0, 0.0, 0.0),
        vec3(0.0, 1.0, 0.0),
        degrees(45.0),
        0.1,
        10.0,
    );

    let positions = Positions::F32(vec![
        vec3(-5.0, -LINE_WIDTH / 2.0, 0.0),
        vec3(-5.0, LINE_WIDTH / 2.0, 0.0),
        vec3(5.0, -LINE_WIDTH / 2.0, 0.0),
        vec3(5.0, LINE_WIDTH / 2.0, 0.0),
    ]);

    let indices = Indices::U8(vec![0, 1, 2, 1, 2, 3]);

    // redundant field names lint doesn't trigger
    let cpu_mesh = CpuMesh {
        positions,
        indices,
        ..Default::default()
    };

    let mut model = Gm::new(Mesh::new(&ctx, &cpu_mesh), ColorMaterial::default());
    model.set_animation(|time| Quat::from_angle_y(radians(time * 0.0005)).into());

    window.render_loop(move |frame_input| {
        camera.set_viewport(frame_input.viewport);

        #[expect(
            clippy::as_conversions,
            reason = "f32 works for anything under ~1e28 years"
        )]
        model.animate(frame_input.accumulated_time as f32);

        frame_input
            .screen()
            .clear(ClearState::color_and_depth(0.8, 0.8, 0.8, 1.0, 1.0))
            .render(&camera, &model, &[]);

        FrameOutput::default()
    });
}
