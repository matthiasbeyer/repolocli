extern crate failure;
extern crate serde;
extern crate serde_json;
extern crate url;
extern crate url_serde;
extern crate curl;

#[cfg(feature = "packagefilters")]
extern crate filters;
#[cfg(feature = "packagefilters")]
#[macro_use] extern crate derive_new;

#[macro_use] extern crate serde_derive;
#[macro_use] extern crate log;
#[macro_use] extern crate derive_more;

pub mod v1;

#[cfg(feature = "packagefilters")]
pub mod packagefilters;
