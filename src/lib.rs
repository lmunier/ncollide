/*!
ncollide
========

**ncollide** is a 2 and 3-dimensional collision detection library written with
the rust programming language.

As its name suggests, it is generic wrt the dimension: it works with both
2-dimensional and 3-dimensional shapes. It might work with higher
dimensions (never tried).

The official user guide is available [here](http://ncollide.org).
The rustdoc documentation is available [here](http://ncollide.org/rustdoc/ncollide).

## Compilation
You will need the last stable build of the [rust compiler](http://www.rust-lang.org)
and the official package manager: [cargo](https://github.com/rust-lang/cargo).

Simply add the following to your `Cargo.toml` file:

```.ignore
[dependencies]
ncollide = "0.13"
```


## Features
- dynamic bounding volume tree based broad phase
- ball vs. ball collision detection,
- plane vs. any convex object collision detection.
- collision detection between arbitrary convex objects
- compound shapes
- ray-casting
- time of impact computation  for objects without rotational movement (compound vs. compound is not
  yet implemented)

And various traits for collision detectors and broad phase collision detection.
*/

#![deny(non_camel_case_types)]
#![deny(unused_parens)]
#![deny(non_upper_case_globals)]
#![deny(unused_qualifications)]
// #![deny(missing_docs)]
#![deny(unused_results)]
#![warn(unused_imports)]
#![allow(missing_copy_implementations)]
#![doc(html_root_url = "http://ncollide.org/rustdoc")]

// pub extern crate ncollide_geometry;
// pub extern crate ncollide_math;
// pub extern crate ncollide_pipeline;
// pub extern crate ncollide_procedural;
// pub extern crate ncollide_transformation;
// pub extern crate ncollide_utils;

// pub use ncollide_math as math;
// pub use ncollide_utils as utils;
// pub use ncollide_geometry::{bounding_volume, partitioning, query, shape};
// pub use ncollide_pipeline::{broad_phase, events, narrow_phase, world};
// pub use ncollide_procedural as procedural;
// pub use ncollide_transformation as transformation;

extern crate slab;
extern crate alga;
#[macro_use]
extern crate approx;
extern crate nalgebra as na;
extern crate num_traits as num;
extern crate smallvec;

mod utils;
pub mod shape;
pub mod bounding_volume;
pub mod partitioning;
pub mod query;
pub mod pipeline;

/// Compilation flags dependent aliases for mathematical types.
#[cfg(feature = "dim3")]
pub mod math {
  use na::{Isometry3, Point3, Translation3, U3, U6, UnitQuaternion, Vector3, Vector6};

  pub const SPATIAL_DIM: usize = 6;
  pub const ANGULAR_DIM: usize = 3;
  pub const DIM: usize = 3;

  /// The dimension of the ambiant space.
  pub type Dim = U3;

  /// The dimension of a spatial vector.
  pub type SpatialDim = U6;

  /// The dimension of the rotations.
  pub type AngularDim = U3;

  /// The point type.
  pub type Point<N> = Point3<N>;

  /// The angular vector type.
  pub type AngularVector<N> = Vector3<N>;

  /// The vector type.
  pub type Vector<N> = Vector3<N>;

  /// The vector type with dimension `SpatialDim × 1`.
  pub type SpatialVector<N> = Vector6<N>;

  /// The orientation type.
  pub type Orientation<N> = Vector3<N>;

  /// The transformation matrix type.
  pub type Isometry<N> = Isometry3<N>;

  /// The rotation matrix type.
  pub type Rotation<N> = UnitQuaternion<N>;

  /// The translation type.
  pub type Translation<N> = Translation3<N>;
}

/// Compilation flags dependent aliases for mathematical types.
#[cfg(feature = "dim2")]
pub mod math {
  use na::{Isometry2, Point2, Translation2, U2, UnitComplex, Vector1, Vector2};

  pub const DIM: usize = 2;

  /// The dimension of the ambiant space.
  pub type Dim = U2;

  /// The point type.
  pub type Point<N> = Point2<N>;

  /// The vector type.
  pub type Vector<N> = Vector2<N>;

  /// The orientation type.
  pub type Orientation<N> = Vector1<N>;

  /// The transformation matrix type.
  pub type Isometry<N> = Isometry2<N>;

  /// The rotation matrix type.
  pub type Rotation<N> = UnitComplex<N>;

  /// The translation type.
  pub type Translation<N> = Translation2<N>;
}
