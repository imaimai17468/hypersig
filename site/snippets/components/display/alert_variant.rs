use hypersig::Signals;
use hypersig_ui::{alert, alert_title};
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct Builds {}

// Without a variant, the alert has no role.
let failed = alert::<Builds>().child(alert_title().text("Build failed"));
