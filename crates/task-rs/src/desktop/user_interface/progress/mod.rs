use serde::{Deserialize, Serialize};
use smart_default::SmartDefault;

#[derive(Debug, SmartDefault, Serialize, Deserialize)]
pub enum Progress {
    #[default]
    Operational,
    Wait,
}
