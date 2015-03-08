//! Bounding volumes.

#[doc(inline)]
pub use bounding_volume::bounding_volume::{HasBoundingVolume, BoundingVolume};
#[doc(inline)]
pub use bounding_volume::aabb::{HasAABB, AABB};
#[doc(inline)]
pub use bounding_volume::bounding_sphere::{HasBoundingSphere, BoundingSphere};

pub use bounding_volume::aabb_utils::{implicit_shape_aabb, point_cloud_aabb};
pub use bounding_volume::aabb_ball::ball_aabb;
pub use bounding_volume::bounding_sphere_utils::{point_cloud_bounding_sphere_with_center, point_cloud_bounding_sphere};
pub use bounding_volume::bounding_volume_bvt::BoundingVolumeInterferencesCollector;

use na::{Pnt2, Pnt3, Vec2, Vec3};

#[doc(hidden)]
pub mod bounding_volume;
mod bounding_volume_bvt;

#[doc(hidden)]
pub mod aabb;
mod aabb_cuboid;
mod aabb_support_map;
mod aabb_ball;
mod aabb_plane;
mod aabb_convex;
mod aabb_compound;
mod aabb_mesh;
mod aabb_utils;
mod aabb_repr;

#[doc(hidden)]
pub mod bounding_sphere;
mod bounding_sphere_cuboid;
mod bounding_sphere_cone;
mod bounding_sphere_ball;
mod bounding_sphere_cylinder;
mod bounding_sphere_capsule;
mod bounding_sphere_plane;
mod bounding_sphere_convex;
mod bounding_sphere_compound;
mod bounding_sphere_triangle;
mod bounding_sphere_segment;
mod bounding_sphere_mesh;
mod bounding_sphere_utils;
mod bounding_sphere_repr;

/*
 *
 * Aliases.
 *
 */
/// A 2D bounding sphere.
pub type BoundingSphere2<N> = BoundingSphere<N, Pnt2<N>, Vec2<N>>;
/// A 2D AABB.
pub type AABB2<N> = AABB<N, Pnt2<N>, Vec2<N>>;

/// A 3D bounding sphere:
pub type BoundingSphere3<N> = BoundingSphere<N, Pnt3<N>, Vec2<N>>;
/// A 3D AABB.
pub type AABB3<N> = AABB<N, Pnt3<N>, Vec2<N>>;
