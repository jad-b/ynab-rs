#[test]
fn gets_all_categories_with_targets() {
    let scenario = ensemble::ensemble().new_scenario();

    let budgeter = scenario.new_budgeter();

    let budget = budgeter.has_budget();
    let categories = budget.categories();

    let category_csv = budgeter.can_export_categories("csv");
}

pub mod ensemble {
    use std::sync::OnceLock;

    pub fn ensemble() -> &'static Ensemble<'static> {
        static ENSEMBLE : OnceLock<Ensemble> = OnceLock::new();
        ENSEMBLE.get_or_init(|| Ensemble {})
    }

    pub struct Ensemble<'a> {}

    impl Ensemble<'_> {
        pub fn new_scenario(&self) -> &mut Scenario {
            &mut Scenario {}
        }
    }

    pub struct Scenario<'a> {}

    impl Scenario<'_> {
        pub fn new_budgeter(&self) -> &mut Budgeter {
            &mut Budgeter{}
        }
    }

    pub struct Budgeter<'a> {}

    impl Budgeter<'_> {
        pub fn has_budget(&self) -> &mut Budget {
            &mut Budget{}
        }

        pub fn can_export_categories(&self, format : &'static str) -> &str {
            "category_name,target_type,target_goal"
        }
    }

    pub struct Budget<'a> {}

    impl Budget<'_> {
        pub fn categories(&self) -> Vec<&Category<'_>> {
            vec!(&mut Category{})
        }
    }

    pub struct Category<'a> {}

}

struct Category {}
