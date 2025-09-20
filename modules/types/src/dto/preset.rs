use std::fmt::Display;

pub type PresetId = i64;

#[derive(Debug, Clone, PartialEq)]
pub struct PresetDTO {
    pub id: PresetId,
    pub name: String,
    pub prompt: String,
    pub temperature: f32,
    pub max_tokens: u32,
}

impl PresetDTO {
    #[must_use] pub fn is_similar(&self, other: &Self) -> bool {
        self.name == other.name
            && self.prompt == other.prompt
            && (self.temperature - other.temperature).abs() < f32::EPSILON
            && self.max_tokens == other.max_tokens
    }
}

impl Default for PresetDTO {
    fn default() -> Self {
        Self {
            id: PresetId::default(),
            name: "New Preset".to_string(),
            prompt: String::new(),
            temperature: 0.7,
            max_tokens: 2048,
        }
    }
}

impl Display for PresetDTO {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}
