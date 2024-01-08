#[test]
fn gets_all_categories_with_targets() {
    let scenario = production.new_scenario();

    let budgeter = scenario.new_budgeter();

    let budget = budgeter.has_budget();
    let categories : Vec<&Category> = budget.categories();

    let category_csv : &str = budgeter.can_export_categories("csv");
}
