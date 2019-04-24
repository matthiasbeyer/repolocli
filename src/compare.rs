#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
pub struct ComparePackage {
    name: String,
    version: String,
}

impl ComparePackage {
    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn version(&self) -> &String {
        &self.version
    }
}