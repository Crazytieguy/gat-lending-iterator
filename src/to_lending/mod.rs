mod into_lending;
mod lend_refs;
mod lend_refs_mut;
#[cfg(feature = "alloc")]
mod windows;
#[cfg(feature = "alloc")]
mod windows_mut;
pub use self::into_lending::IntoLending;
pub use self::lend_refs::LendRefs;
pub use self::lend_refs_mut::LendRefsMut;
#[cfg(feature = "alloc")]
pub use self::windows::Windows;
#[cfg(feature = "alloc")]
pub use self::windows_mut::WindowsMut;

// TODO: seperate non-buffered and buffered windows
