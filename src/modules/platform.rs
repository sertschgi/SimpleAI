pub trait WindowLike {
    fn open(&self);
}

#[cfg(not(any(feature = "web", feature = "desktop")))]
pub struct Window {}
#[cfg(not(any(feature = "web", feature = "desktop")))]
impl WindowLike for Window {
    fn open(&self) {}
}
#[cfg(not(any(feature = "web", feature = "desktop")))]
pub fn launch() {}

#[cfg(feature = "desktop")]
pub mod desktop;

#[cfg(feature = "desktop")]
pub use desktop::*;

#[cfg(feature = "web")]
pub mod web;

#[cfg(feature = "web")]
pub use web::*;
