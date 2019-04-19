use filters::filter::Filter;
use filters::ops::and::And;
use filters::ops::bool::Bool;
use filters::ops::not::Not;

use crate::config::Configuration;

struct BlackListFilter {
    repo_name: String,
}

impl BlackListFilter {
    pub fn new(repo_name: String) -> Self {
        BlackListFilter { repo_name }
    }
}

impl Filter<String> for BlackListFilter {
    fn filter(&self, element: &String) -> bool {
        element != self.repo_name
    }
}

struct WhiteListFilter {
    repo_name: String,
}

impl Filter<String> for WhiteListFilter {
    fn filter(&self, element: &String) -> bool {
        element == self.repo_name
    }
}

pub fn repo_filter(config: &Configuration) -> Box<Filter<String>> {
    let blacklist = config
        .blacklist()
        .iter()
        .cloned()
        .map(BlackListFilter::new)
        .fold(Box::new(Bool::new(true)), |accu, element| accu.and(element));
    let whitelist = config
        .whitelist()
        .iter()
        .cloned()
        .map(WhiteListFilter::new)
        .fold(Box::new(Bool::new(true)), |accu, element| accu.and(element));

    Box::new(blacklist.not().or(whitelist))
}
