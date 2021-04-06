// package status
#[derive(Eq, PartialEq, Clone, Debug, Serialize, Deserialize, Display)]
pub enum Status {
    #[serde(rename = "newest")]
    #[display(fmt = "newest")]
    Newest,

    #[serde(rename = "devel")]
    #[display(fmt = "devel")]
    Devel,

    #[serde(rename = "unique")]
    #[display(fmt = "unique")]
    Unique,

    #[serde(rename = "outdated")]
    #[display(fmt = "outdated")]
    Outdated,

    #[serde(rename = "legacy")]
    #[display(fmt = "legacy")]
    Legacy,

    #[serde(rename = "rolling")]
    #[display(fmt = "rolling")]
    Rolling,

    #[serde(rename = "noscheme")]
    #[display(fmt = "noscheme")]
    Noscheme,

    #[serde(rename = "incorrect")]
    #[display(fmt = "incorrect")]
    Incorrect,

    #[serde(rename = "untrusted")]
    #[display(fmt = "untrusted")]
    Untrusted,

    #[serde(rename = "ignored")]
    #[display(fmt = "ignored")]
    Ignored,
}
