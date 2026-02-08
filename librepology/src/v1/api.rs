use crate::v1::error::Result;
use crate::v1::types::Package;
use crate::v1::types::Problem;

/// The high-level functionality of the repology API is represented in this trait
///
/// Each "functionality" is represented via one function.
///
/// # Note
///
/// This is implemented as a _trait_ rather than a _struct_ because this way we can reuse the
/// functionality for operating on a stream, for example on stdin as a source of data.
pub trait Api {
    fn project<N: AsRef<str>>(&self, name: N) -> Result<Vec<Package>>;

    fn problems_for_repo<R: AsRef<str>>(&self, repo: R) -> Result<Vec<Problem>>;

    fn problems_for_maintainer<M: AsRef<str>, R: AsRef<str>>(&self, maintainer: M, repo: R) -> Result<Vec<Problem>>;
}
