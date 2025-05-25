pub mod archive;
pub mod copy_only;
pub mod get_details;
pub mod install;
pub mod install_with_link;
pub mod launcher;
pub mod uninstall;
pub mod validation;

pub use archive::*;
pub use copy_only::{copy_only, CopyOnlyConfig};
pub use get_details::*;
pub use install::*;
pub use install_with_link::*;
pub use launcher::*;
pub use uninstall::*;
pub use validation::*;
