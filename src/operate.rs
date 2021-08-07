use fantoccini::Locator;

use crate::{DriverAction, Mendokusai, MendokusaiError, MendokusaiOp, Operation};

pub async fn operate(app: &mut Mendokusai, op: MendokusaiOp) -> Result<(), MendokusaiError> {
    for operation in op.into_iter() {
        run(app, &operation).await?;
    }
    Ok(())
}

async fn run(app: &mut Mendokusai, op: &Operation) -> Result<(), MendokusaiError> {
    match op {
        Operation::WebDriver(action) => run_webdriver(app, action).await,
        Operation::Slack => todo!(),
    }
}

async fn run_webdriver(app: &mut Mendokusai, action: &DriverAction) -> Result<(), MendokusaiError> {
    match action {
        DriverAction::Open { url } => {
            app.client().goto(url).await?;
            Ok(())
        }
        DriverAction::Input {
            css: locator,
            value,
        } => {
            let mut ele = app.client().find(Locator::Css(locator)).await?;
            ele.send_keys(value).await?;
            Ok(())
        }
        DriverAction::Click { css: locator } => {
            let ele = app.client().find(Locator::Css(locator)).await?;
            ele.click().await?;
            Ok(())
        }
        DriverAction::End => {
            app.client().close().await?;
            Ok(())
        }
    }
}
