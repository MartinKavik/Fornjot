//! Append-only object storage

mod blocks;
mod handle;
mod store;

use crate::objects::{Curve, GlobalCurve, Surface};

pub use self::{
    handle::{Handle, HandleWrapper, ObjectId},
    store::{Iter, Reservation, Store},
};

/// The available object stores
///
/// # Implementation Note
///
/// The intention is to eventually manage all objects in here. Making this
/// happen is simply a case of putting in the required work. See [#1021].
///
/// [#1021]: https://github.com/hannobraun/Fornjot/issues/1021
#[derive(Debug, Default)]
pub struct Stores {
    /// Store for curves
    pub curves: Store<Curve>,

    /// Store for global curves
    pub global_curves: Store<GlobalCurve>,

    /// Store for surfaces
    pub surfaces: Store<Surface>,
}

impl Stores {
    /// Construct a new instance of `Stores`
    pub fn new() -> Self {
        Self::default()
    }
}
