pub mod domain;

use domain::DomainScenario;
use std::{
    convert::TryFrom,
    env,
    sync::OnceLock,
};
use ynabrs::domain::Goal;

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

    fn can_export_goals(&mut self);
}

