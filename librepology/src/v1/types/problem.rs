use crate::v1::types::*;

#[derive(Eq, PartialEq, Clone, Debug, Serialize, Deserialize)]
pub struct Problem {
    #[serde(rename = "repo")]
    repo: Repo,

    #[serde(rename = "name")]
    name: Name,

    #[serde(rename = "effname")]
    effname: EffName,

    #[serde(rename = "maintainer")]
    maintainer: Maintainer,

    #[serde(rename = "problem")]
    problem: String,
}

impl Problem {
    pub fn repo(&self) -> &Repo {
        &self.repo
    }

    pub fn name(&self) -> &Name {
        &self.name
    }

    pub fn effname(&self) -> &EffName {
        &self.effname
    }

    pub fn maintainer(&self) -> &Maintainer {
        &self.maintainer
    }

    pub fn problem_description(&self) -> &String {
        &self.problem
    }
}
