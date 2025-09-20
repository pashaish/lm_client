use std::fmt::Display;

pub type ProviderID = i64;

#[derive(Clone, Debug)]
pub struct LmModel {
    pub model_name: String,
    pub provider: Option<ProviderDTO>,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ProviderDTO {
    #[allow(dead_code)]
    pub id: ProviderID,
    #[allow(dead_code)]
    pub name: String,
    pub url: String,
    pub api_key: String,
    pub default_model: String,
}

impl ProviderDTO {
    #[must_use] pub fn is_similar(&self, dto: &Self) -> bool {
        self.name == dto.name
            && self.url == dto.url
            && self.api_key == dto.api_key
            && self.default_model == dto.default_model
    }
}

impl Display for ProviderDTO {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}
