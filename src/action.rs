use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MendokusaiOp {
    operations: Vec<Operation>,
}

impl IntoIterator for MendokusaiOp {
    type Item = Operation;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.operations.into_iter()
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Operation {
    WebDriver(DriverAction),
    Slack,
}

impl From<DriverAction> for Operation {
    fn from(act: DriverAction) -> Self {
        Self::WebDriver(act)
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum DriverAction {
    Open { url: Url },
    Input { css: Loc, value: Value },
    Click { css: Loc },
    End,
}

type Url = String;
type Loc = String;
type Value = String;
