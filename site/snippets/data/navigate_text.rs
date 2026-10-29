# use hypersig::{Patches, Signals};
# use serde::{Deserialize, Serialize};
# #[derive(Signals, Serialize, Deserialize)]
# struct Login {
#     problem: String,
# }
// `navigate` takes a `Url`, which only a route's `url` builds.
let _signed_in: Patches<Login> = Patches::new().navigate("/");
