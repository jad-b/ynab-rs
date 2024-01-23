use assembly::{Budgeter, Scenario};
use domain::{Goal, GoalFreq};


fn example_goals() -> Vec<Goal> {
    vec!(
        Goal::new(
            String::from("Groceries"),
            GoalFreq::Monthly,
            800,
        )
    )
}

#[test]
fn gets_all_goals_with_targets() {
    // Scenario
    let scenario = assembly::assembly().new_scenario();
    // Retrieve actors for roles
    let mut budgeter = scenario.new_budgeter();
    // Observe system behavior (test) by executing actor tasks
    budgeter.sets_monthly_goals(example_goals());
    budgeter.can_export_goals().unwrap();
}

pub mod assembly {
    use domain::DomainScenario;
    use std::{
        convert::TryFrom,
        env,
        io,
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
        fn sets_monthly_goals<'a>(&'a mut self, goals: Vec<Goal>);

        fn has_goals(&self) -> &[Goal];

        fn can_export_goals(&mut self) -> io::Result<()>;
    }

    pub mod domain {
        use std::{
            io,
        };

        use csv::Writer;

        use super::{Budgeter,Scenario};
        use crate::domain::Goal;

        pub struct DomainScenario {}

        impl Scenario for DomainScenario {
            fn new_budgeter(&self) -> impl Budgeter {
                DomainBudgeter{
                    goals: Vec::new(),
                    csv_output: Writer::from_writer(vec![]),
                }
            }
        }

        pub struct DomainBudgeter {
            goals: Vec<Goal>,
            csv_output: Writer<Vec<u8>>
        }

        impl Budgeter for DomainBudgeter {
            fn sets_monthly_goals<'a>(&'a mut self, goals: Vec<Goal>) {
                self.goals = goals;
            }

            fn has_goals(&self) -> &[Goal] {
                &self.goals
            }

            fn can_export_goals(&mut self) -> io::Result<()> {
                self.goals.iter()
                    .for_each(|g| self.csv_output.serialize(g).unwrap());
                self.csv_output.flush()?;

                // todo| check the correct CSV was written
                Ok(())
            }
        }
    }
}

pub mod domain {
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
}
