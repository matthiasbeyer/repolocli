use clap::ArgMatches;
use failure::Fallible as Result;

use librepology::v1::api::Api;
use librepology::v1::restapi::RestApi;
use librepology::v1::stdinapi::StdinWrapper;
use librepology::v1::types::*;

use crate::config::Configuration;

/// Helper type for cli implementation
/// for being transparent in what backend we use
pub enum Backend {
    Stdin(StdinWrapper),
    RepologyOrg(RestApi),
}

/// Implement Api for Backend
///
/// With this, we can use the `Backend` object and do not have to care whether we have a librepology::
impl Api for Backend {
    fn project<N: AsRef<str>>(&self, name: N) -> Result<Vec<Package>> {
        match self {
            Backend::Stdin(inner) => inner.project(name),
            Backend::RepologyOrg(inner) => inner.project(name),
        }
    }

    fn problems_for_repo<R: AsRef<str>>(&self, repo: R) -> Result<Vec<Problem>> {
        match self {
            Backend::Stdin(inner) => inner.problems_for_repo(repo),
            Backend::RepologyOrg(inner) => inner.problems_for_repo(repo),
        }
    }

    fn problems_for_maintainer<M: AsRef<str>>(&self, maintainer: M) -> Result<Vec<Problem>> {
        match self {
            Backend::Stdin(inner) => inner.problems_for_maintainer(maintainer),
            Backend::RepologyOrg(inner) => inner.problems_for_maintainer(maintainer),
        }
    }
}

pub fn new_backend(app: &ArgMatches, config: &Configuration) -> Result<Backend> {
    if app.is_present("input_stdin") {
        trace!("Building new STDIN backend");
        Ok(Backend::Stdin(StdinWrapper::from(::std::io::stdin())))
    } else {
        trace!("Building new remote backend");
        let url = config.repology_url().as_str().into();
        trace!("url = {}", url);
        Ok(Backend::RepologyOrg(RestApi::new(url)))
    }
}
