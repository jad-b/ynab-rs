use assembly::{Budgeter, Scenario};

#[test]
fn gets_all_goals_with_targets() {
    let scenario = assembly::assembly().new_scenario();

    let budgeter = scenario.new_budgeter();

    let goals = budgeter.has_monthly_goals();

    let _goals_csv = budgeter.can_export_goals("csv", &goals);
}

pub mod assembly {
    use domain::DomainScenario;
    use std::{
        convert::TryFrom,
        env,
        sync::OnceLock,
    };
    use super::domain::Goal;

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
        pub fn new_scenario(&self) -> impl Scenario {
            match self.asm_type {
                AssemblyType::Domain => DomainScenario{},
                AssemblyType::Live => DomainScenario{},
            }
        }
    }

    pub trait Scenario {
        fn new_budgeter(&self) -> impl Budgeter;
    }

    pub trait Budgeter {
        fn has_monthly_goals(&self) -> Vec<Goal>;

        fn can_export_goals(
            &self,
            _format: &'static str,
            _goals: &Vec<Goal>
        ) -> &str;
    }


    pub mod domain {
        use super::{Budgeter,Scenario};
        use crate::domain::Goal;

        pub struct DomainScenario {}

        impl Scenario for DomainScenario {
            fn new_budgeter(&self) -> impl Budgeter {
                DomainBudgeter{}
            }
        }

        pub struct DomainBudgeter {}

        impl Budgeter for DomainBudgeter {
            fn has_monthly_goals(&self) -> Vec<Goal> {
                vec!(Goal{})
            }

            fn can_export_goals(
                &self,
                _format: &'static str,
                _goals: &Vec<Goal>
            ) -> &str {
                "name,frequency,total_amount,monthly_allocation"
            }
        }
    }
}

pub mod domain {
    pub struct Goal {}
}
