use ynabrs::domain::{Goal, GoalFreq};


pub fn example_goals() -> Vec<Goal> {
    vec!(
        Goal::new(
            String::from("Mortgage"),
            GoalFreq::Monthly,
            2545,
        ),
        Goal::new(
            String::from("Groceries"),
            GoalFreq::Monthly,
            1200,
        ),
        Goal::new(
            String::from("Spotify"),
            GoalFreq::Monthly,
            16,
        ),
        Goal::new(
            String::from("Tax Professional"),
            GoalFreq::Yearly,
            730,
        ),
    )
}

