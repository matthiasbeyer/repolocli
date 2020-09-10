use std::io::{Stdin, Read};
use std::cell::RefCell;
use std::ops::Deref;
use std::ops::DerefMut;

use failure::Fallible as Result;
use failure::Error;

use crate::v1::types::Problem;
use crate::v1::types::Package;
use crate::v1::api::Api;

/// Wrapper for "stdin"
///
/// This way we can implement the `Api` trait for StdIn (via a Wrapper for interior mutability)
/// This way we can read the data from stdin and process it.
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

fn read_to_string(input: &mut dyn Read) -> Result<String> {
    let mut buffer = String::new();
    let read = input.read_to_string(&mut buffer)?;
    trace!("Read {} bytes from stdin", read);
    Ok(buffer)
}
