use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "condition", content = "value")]
pub enum OrderedCondition<T: PartialOrd> {
    #[serde(rename = "anyOf")]
    AnyOf(Vec<T>),
    #[serde(rename = "noneOf")]
    NoneOf(Vec<T>),
    #[serde(rename = ">")]
    GreaterThan(Vec<T>),
    #[serde(rename = ">=")]
    GreaterOrEqualThan(Vec<T>),
    #[serde(rename = "<")]
    LowerThan(Vec<T>),
    #[serde(rename = "<=")]
    LowerOrEqualThan(Vec<T>),
}

impl<T: PartialOrd> OrderedCondition<T> {
    // GETTERS ----------------------------------------------------------------

    pub fn value(&self) -> &Vec<T> {
        match self {
            OrderedCondition::AnyOf(v) => v,
            OrderedCondition::NoneOf(v) => v,
            OrderedCondition::GreaterThan(v) => v,
            OrderedCondition::GreaterOrEqualThan(v) => v,
            OrderedCondition::LowerThan(v) => v,
            OrderedCondition::LowerOrEqualThan(v) => v,
        }
    }

    pub fn verify(&self, value: &T) -> bool {
        match self {
            OrderedCondition::AnyOf(list) => list.iter().any(|v| v == value),
            OrderedCondition::NoneOf(list) => list.iter().all(|v| v != value),
            OrderedCondition::GreaterThan(list) => list.iter().all(|v| value > v),
            OrderedCondition::GreaterOrEqualThan(list) => list.iter().all(|v| value >= v),
            OrderedCondition::LowerThan(list) => list.iter().all(|v| value < v),
            OrderedCondition::LowerOrEqualThan(list) => list.iter().all(|v| value <= v),
        }
    }
}
