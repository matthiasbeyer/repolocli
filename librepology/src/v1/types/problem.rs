use crate::v1::types::*;

#[derive(Eq, PartialEq, Clone, Debug, Serialize, Deserialize)]
pub struct Problem {
    #[serde(rename = "project_name")]
    project_name: Name,

    #[serde(rename = "maintainer")]
    maintainer: Maintainer,

    #[serde(rename = "type")]
    problem_type: String,

    #[serde(rename = "srcname", default)]
    srcname: Option<String>,

    #[serde(rename = "version", default)]
    version: Option<Version>,
}

impl Problem {
    pub fn project_name(&self) -> &Name {
        &self.project_name
    }

    pub fn maintainer(&self) -> &Maintainer {
        &self.maintainer
    }

    pub fn problem_type(&self) -> &String {
        &self.problem_type
    }

    pub fn srcname(&self) -> Option<&String> {
        self.srcname.as_ref()
    }

    pub fn version(&self) -> Option<&Version> {
        self.version.as_ref()
    }
}
