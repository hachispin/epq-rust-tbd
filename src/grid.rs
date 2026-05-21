//! Module responsible for handling and rendering the grid.
#![allow(unused, reason = "WIP")]

use std::ops::Range;

use three_d::{
    ColorMaterial,
    Context,
    CpuMesh,
    Gm,
    Mesh,
    Positions,
    Srgba,
    Vec3,
    Vector3,
    Zero,
    vec3,
};

/// A shape rendered on the grid.
///
/// This can represent a point, line and polygon.
#[derive(Debug)]
struct GridShape {
    /// Points to render.
    ///
    /// If there is more than one point, then the points
    /// are connected in the order that they're declared.
    points: Vec<Vec3>,
    /// The color of the point and the connecting lines, if present.
    color: Srgba,
    /// Whether the last point should connect with the first.
    ///
    /// Has no impact if only two or less points are defined in [`Self::points`].
    closed: bool,
}

/// Represents a 3D grid.
///
/// Generally should be declared as mutable.
pub struct PlottingGrid {
    /// Initially (0, 0, 0).
    centre: Vec3,
    // At a magnification of 1, a plotting unit
    // should be the same size as a world unit.
    // NOTE: think about whether this should be constrained (having a max/min value)
    magnification: f32,
    /// Shapes to render.
    shapes: Vec<GridShape>,
    /// The mesh. Will be updated when needed.
    mesh: CpuMesh,
}

#[expect(clippy::needless_pass_by_ref_mut, reason = "WIP")]
impl PlottingGrid {
    fn zoom_in(&mut self) { todo!() }

    fn zoom_out(&mut self) { todo!() }
}
