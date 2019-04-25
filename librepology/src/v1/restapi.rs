use std::result::Result as RResult;

use failure::Error;
use failure::Fallible as Result;
use curl::easy::Easy2;

use crate::v1::types::{Package, Problem};
use crate::v1::api::Api;

/// Private helper type for collecting data from the curl library
struct Collector(Vec<u8>);
impl curl::easy::Handler for Collector {
    fn write(&mut self, data: &[u8]) -> RResult<usize, curl::easy::WriteError> {
        self.0.extend_from_slice(data);
        Ok(data.len())
    }
}

/// Representational object for the REST Api of repology
pub struct RestApi {
    /// Base url
    repology: String,
}

impl RestApi {
    pub fn new(repology: String) -> Self {
        Self { repology }
    }

    /// Helper function for sending a request via the curl library
    fn send_request<U: AsRef<str>>(&self, request: U) -> Result<String> {
        let mut easy = Easy2::new(Collector(Vec::new()));
        easy.get(true)?;
        easy.url(request.as_ref())?;
        easy.perform()?;
        let content = easy.get_ref().0.clone(); // TODO: Ugh...
        String::from_utf8(content).map_err(Error::from)
    }
}

impl Api for RestApi {

    fn project<N: AsRef<str>>(&self, name: N) -> Result<Vec<Package>> {
        let request_url = format!("{}api/v1/project/{}", self.repology, name.as_ref());

        self.send_request(request_url)
            .and_then(|r| {
                serde_json::from_str(&r).map_err(Error::from)
            })
    }

    fn problems_for_repo<R: AsRef<str>>(&self, repo: R) -> Result<Vec<Problem>> {
        let request_url = format!("{}api/v1/repository/{}/problems", self.repology, repo.as_ref());

        self.send_request(request_url)
            .and_then(|r| {
                serde_json::from_str(&r).map_err(Error::from)
            })
    }

    fn problems_for_maintainer<M: AsRef<str>>(&self, maintainer: M) -> Result<Vec<Problem>> {
        let request_url = format!("{}api/v1/maintainer/{}/problems", self.repology, maintainer.as_ref());

        self.send_request(request_url)
            .and_then(|r| {
                serde_json::from_str(&r).map_err(Error::from)
            })
    }

}