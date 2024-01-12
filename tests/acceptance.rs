use std::convert::TryFrom;
use std::env;

enum AssemblyType{
    Domain,
    Live,
}

impl TryFrom<&str> for AssemblyType {
    type Error = String;

    fn try_from(val: &str) -> Result<Self, Self::Error> {
        match val.to_ascii_lowercase().as_str() {
            "domain" => Ok(Self::Domain),
            "live" => Ok(Self::Live),
            other => Err(format!("'{other}' isn't a supported test assembly")),
        }
    }
}

#[test]
fn gets_all_categories_with_targets() {
    let scenario = assembly::assembly().new_scenario();

    let budgeter = scenario.new_budgeter();

    let budget = budgeter.has_budget();
    let categories = budget.categories();

    let _category_csv = budgeter.can_export_categories("csv", &categories);
}

pub mod assembly {
    use std::sync::OnceLock;

    pub fn assembly() -> &'static Assembly {
        static ASSEMBLY : OnceLock<Assembly> = OnceLock::new();
        ASSEMBLY.get_or_init(|| Assembly{})
    }

    pub struct Assembly { }

    impl Assembly {
        pub fn new_scenario(&self) -> Scenario {
            Scenario{}
        }
    }

    pub struct Scenario {}

    impl Scenario {
        pub fn new_budgeter(&self) -> Budgeter {
            Budgeter{}
        }
    }

    pub struct Budgeter {}

    impl Budgeter {
        pub fn has_budget(&self) -> Budget {
            Budget{}
        }

        pub fn can_export_categories(
            &self,
            _format: &'static str,
            _categories: &Vec<Category>
        ) -> &str {
            "category_name,target_type,target_goal"
        }
    }

    pub struct Budget {}

    impl Budget {
        pub fn categories(&self) -> Vec<Category> {
            vec!(Category{})
        }
    }

    pub struct Category {}
}
