pub mod archive;
pub mod copy_only;
pub mod get_details;
pub mod install_with_link;
pub mod installation;
pub mod launcher;
pub mod uninstallation;
pub mod validation;

pub use archive::*;
pub use copy_only::{copy_only, CopyOnlyConfig};
pub use get_details::*;
pub use install_with_link::*;
pub use installation::*;
pub use launcher::*;
pub use uninstallation::*;
pub use validation::*;
