use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "condition", content = "value")]
pub enum OptionCondition<T: PartialEq> {
    #[serde(rename = "anyOf")]
    AnyOf(Vec<T>),
    #[serde(rename = "noneOf")]
    NoneOf(Vec<T>),
}

impl<T: PartialEq> OptionCondition<T> {
    // GETTERS ----------------------------------------------------------------

    pub fn value(&self) -> &Vec<T> {
        match self {
            OptionCondition::AnyOf(v) => v,
            OptionCondition::NoneOf(v) => v,
        }
    }

    pub fn verify(&self, value: &T) -> bool {
        match self {
            OptionCondition::AnyOf(list) => list.iter().any(|v| v == value),
            OptionCondition::NoneOf(list) => list.iter().all(|v| v != value),
        }
    }
}
