pub mod page;
pub mod tasks;

pub use page::Page;
pub use tasks::Tasks;

use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct View {
    pub page: Page,
    pub tasks: Tasks,
}
