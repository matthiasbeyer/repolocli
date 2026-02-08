use std::io::Read;

use crate::v1::api::Api;
use crate::v1::error::RepologyError as Error;
use crate::v1::error::Result;
use crate::v1::types::Package;
use crate::v1::types::Problem;

#[derive(Debug)]
pub struct BufferApi {
    buf: String,
}

impl BufferApi {
    pub fn read_from<R>(mut input: R) -> Result<BufferApi>
    where
        R: Read,
    {
        let mut buf = String::new();
        let read = input.read_to_string(&mut buf)?;
        trace!("Read {} bytes from stdin", read);
        Ok(BufferApi { buf })
    }
}

impl Api for BufferApi {
    fn project<N: AsRef<str>>(&self, _name: N) -> Result<Vec<Package>> {
        serde_json::de::from_str(&self.buf).map_err(Error::from)
    }

    fn problems_for_repo<R: AsRef<str>>(&self, _repo: R) -> Result<Vec<Problem>> {
        serde_json::de::from_str(&self.buf).map_err(Error::from)
    }

    fn problems_for_maintainer<M: AsRef<str>, R: AsRef<str>>(&self, _maintainer: M, _repo: R) -> Result<Vec<Problem>> {
        serde_json::de::from_str(&self.buf).map_err(Error::from)
    }
}
