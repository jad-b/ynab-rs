#[test]
fn gets_all_categories_with_targets() {
    let scenario = assembly::assembly().new_scenario();

    let budgeter = scenario.new_budgeter();

    let budget = budgeter.has_budget();
    let categories = budget.categories();

    let _category_csv = budgeter.can_export_categories("csv", &categories);
}

pub mod assembly {
    use std::{
        convert::TryFrom,
        env,
        sync::OnceLock,
    };

    const ASSEMBLY_TYPE: &str = "ASSEMBLY_TYPE";

    #[derive(Debug)]
    enum AssemblyType{
        Domain,
        Live,
    }

    impl TryFrom<String> for AssemblyType {
        type Error = String;

        fn try_from(mut val: String) -> Result<Self, Self::Error> {
            val.make_ascii_lowercase();
            match val.as_str() {
                "domain" => Ok(Self::Domain),
                "live" => Ok(Self::Live),
                other => Err(format!("'{other}' isn't a supported test assembly")),
            }
        }
    }

    pub fn assembly() -> &'static Assembly {
        static ASSEMBLY : OnceLock<Assembly> = OnceLock::new();
        let asm_type = AssemblyType::try_from(
                env::var(ASSEMBLY_TYPE).unwrap_or("Domain".to_string()))
            .unwrap();
        print!("Assembly Type = {asm_type:?}");
        ASSEMBLY.get_or_init(|| Assembly{asm_type})
    }

    pub struct Assembly {
        asm_type: AssemblyType,
    }

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
