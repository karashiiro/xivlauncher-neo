use crate::game::constants::{FRONTIER_GATE_STATUS, FRONTIER_LOGIN_STATUS};

use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct FrontierStatus {
    status: i8,
}

async fn get_status(url: &str) -> Result<bool, reqwest::Error> {
    let resp = reqwest::get(url).await?;

    /*
    match resp {
        Ok(result) => Ok(result.json()),
        Err(e) => Err(e),
    }
*/

    let res: FrontierStatus = resp.json::<FrontierStatus>().await?;
    Ok(res.status == 1)
}

pub async fn is_gate() -> Result<bool, reqwest::Error> {
    get_status(FRONTIER_GATE_STATUS).await
}

pub async fn is_login() -> Result<bool, reqwest::Error> {
    get_status(FRONTIER_GATE_STATUS).await
}