mod arc;
mod line;

use crate::algorithms::Tolerance;

pub use self::{arc::Arc, line::Line};

use fj_math::{Point, Transform, Vector};

/// A one-dimensional shape
///
/// The word "curve" is used as an umbrella term for all one-dimensional shapes,
/// and doesn't imply that those shapes need to be curved. Straight lines are
/// included.
///
/// The nomenclature is inspired by Boundary Representation Modelling Techniques
/// by Ian Stroud. "Curve" refers to unbounded one-dimensional geometry, while
/// while edges are bounded portions of curves.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub enum Curve {
    /// An arc
    Arc(Arc),

    /// A line
    Line(Line),
}

impl Curve {
    /// Construct a `Curve` that represents the x-axis
    pub fn x_axis() -> Self {
        Self::Line(Line {
            origin: Point::origin(),
            direction: Vector::unit_x(),
        })
    }

    /// Access the origin of the curve's coordinate system
    pub fn origin(&self) -> Point<3> {
        match self {
            Self::Arc(curve) => curve.origin(),
            Self::Line(curve) => curve.origin(),
        }
    }

    /// Create a new instance that is reversed
    #[must_use]
    pub fn reverse(self) -> Self {
        match self {
            Self::Arc(curve) => Self::Arc(curve.reverse()),
            Self::Line(curve) => Self::Line(curve.reverse()),
        }
    }

    /// Create a new instance that is transformed by `transform`
    #[must_use]
    pub fn transform(self, transform: &Transform) -> Self {
        match self {
            Self::Arc(curve) => Self::Arc(curve.transform(transform)),
            Self::Line(curve) => Self::Line(curve.transform(transform)),
        }
    }

    /// Convert a point in model coordinates to curve coordinates
    ///
    /// Projects the point onto the curve before computing curve coordinate.
    /// This is done to make this method robust against floating point accuracy
    /// issues.
    ///
    /// Callers are advised to be careful about the points they pass, as the
    /// point not being on the curve, intentional or not, will never result in
    /// an error.
    pub fn point_model_to_curve(&self, point: &Point<3>) -> Point<1> {
        match self {
            Self::Arc(curve) => curve.point_model_to_curve(point),
            Self::Line(curve) => curve.point_model_to_curve(point),
        }
    }

    /// Convert a point on the curve into model coordinates
    pub fn point_curve_to_model(&self, point: &Point<1>) -> Point<3> {
        match self {
            Self::Arc(curve) => curve.point_curve_to_model(point),
            Self::Line(curve) => curve.point_curve_to_model(point),
        }
    }

    /// Convert a vector on the curve into model coordinates
    pub fn vector_curve_to_model(&self, point: &Vector<1>) -> Vector<3> {
        match self {
            Self::Arc(curve) => curve.vector_curve_to_model(point),
            Self::Line(curve) => curve.vector_curve_to_model(point),
        }
    }

    /// Compute an approximation of the curve
    ///
    /// `tolerance` defines how far the approximation is allowed to deviate from
    /// the actual edge.
    ///
    /// # Implementation Note
    ///
    /// This only works as it is, because edges are severely limited and don't
    /// define which section of the curve they inhabit. Once they do that, we
    /// need an `approximate_between(a, b)` method instead, where `a` and `b`
    /// are the vertices that bound the edge on the curve.
    ///
    /// The `approximate_between` methods of the curves then need to make sure
    /// to only return points in between those vertices, not the vertices
    /// themselves.
    pub fn approx(&self, tolerance: Tolerance, out: &mut Vec<Point<3>>) {
        match self {
            Self::Arc(curve) => curve.approx(tolerance, out),
            Self::Line(_) => {}
        }
    }
}