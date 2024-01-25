use serde::Serialize;

#[derive(Debug, Serialize)]
pub enum GoalFreq {
    Monthly,
    Yearly,
}

#[derive(Debug, Serialize)]
pub struct Goal {
    name: String,
    frequency: GoalFreq,
    // todo| currency/decimal support
    target: usize,
}

impl Goal {
    pub fn new(name: String, frequency: GoalFreq, target: usize) -> Goal {
        Goal{
            name,
            frequency,
            target,
        }
    }
}
