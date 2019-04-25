use std::io::{Stdin, Read};
use std::cell::RefCell;
use std::ops::Deref;
use std::ops::DerefMut;

use failure::Fallible as Result;
use failure::Error;

use crate::v1::types::Problem;
use crate::v1::types::Package;

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

    fn problems_for_maintainer<M: AsRef<str>>(&self, maintainer: M) -> Result<Vec<Problem>>;
}


//
// Api implemented for StdIn (via a Wrapper for interior mutability)
//
// This way we can read the data from stdin and process it
//

pub struct StdinWrapper(RefCell<Stdin>);

impl From<Stdin> for StdinWrapper {
    fn from(inner: Stdin) -> Self {
        StdinWrapper(RefCell::new(inner))
    }
}

impl Deref for StdinWrapper {
    type Target = RefCell<Stdin>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for StdinWrapper {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Api for StdinWrapper {

    fn project<N: AsRef<str>>(&self, _name: N) -> Result<Vec<Package>> {
        let s = read_to_string(self.0.try_borrow_mut()?.deref_mut())?;
        serde_json::de::from_str(&s).map_err(Error::from)
    }

    fn problems_for_repo<R: AsRef<str>>(&self, _repo: R) -> Result<Vec<Problem>> {
        let s = read_to_string(self.0.try_borrow_mut()?.deref_mut())?;
        serde_json::de::from_str(&s).map_err(Error::from)
    }

    fn problems_for_maintainer<M: AsRef<str>>(&self, _maintainer: M) -> Result<Vec<Problem>> {
        let s = read_to_string(self.0.try_borrow_mut()?.deref_mut())?;
        serde_json::de::from_str(&s).map_err(Error::from)
    }

}

fn read_to_string(input: &mut Read) -> Result<String> {
    let mut buffer = String::new();
    let read = input.read_to_string(&mut buffer)?;
    trace!("Read {} bytes from stdin", read);
    Ok(buffer)
}
