use csv::Writer;

use super::{Budgeter,Scenario};
use ynabrs::domain::Goal;


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

    fn can_export_goals(&mut self) {
        self.goals.iter()
            .for_each(|g| self.csv_output.serialize(g).unwrap());
        self.csv_output.flush().unwrap();

        let data = String::from_utf8(self.csv_output.get_ref().to_vec()).unwrap();
        let exp_data = "\
            name,frequency,target\n\
            Mortgage,Monthly,2545\n\
            Groceries,Monthly,1200\n\
            Spotify,Monthly,16\n\
            Tax Professional,Yearly,730\n";
        assert_eq!(&data, exp_data);
    }
}
