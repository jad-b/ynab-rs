mod common;

use crate::common::{
    assembly::*,
    example_data::*,
};


#[test]
fn gets_all_goals_with_targets() {
    // Scenario
    let scenario = assembly().new_scenario();
    // Retrieve actors for roles
    let mut budgeter = scenario.new_budgeter();
    // Observe system behavior (test) by executing actor tasks
    budgeter.sets_monthly_goals(example_goals());
    budgeter.can_export_goals();
}

