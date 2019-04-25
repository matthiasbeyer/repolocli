//! Module containing all _types_ of data for the API implementation
//!
//! Tne types have no functionality themselves but only represent objects which are returned by the´
//! repology API.
//!
//! This top-level module exports all types of the submodules publicly.
//!

mod category;
mod download;
mod effname;
mod license;
mod maintainer;
mod name;
mod package;
mod problem;
mod repo;
mod status;
mod summary;
mod version;
mod www;

pub use category::Category;
pub use download::Download;
pub use effname::EffName;
pub use license::License;
pub use maintainer::Maintainer;
pub use name::Name;
pub use package::Package;
pub use problem::Problem;
pub use repo::Repo;
pub use status::Status;
pub use summary::Summary;
pub use version::Version;
pub use www::Www;

