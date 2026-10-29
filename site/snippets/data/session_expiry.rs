use tower_sessions::cookie::SameSite;
use tower_sessions::cookie::time::Duration;
use tower_sessions::{Expiry, MemoryStore, SessionManagerLayer};

let sessions = SessionManagerLayer::new(MemoryStore::default())
    .with_name("session")
    .with_same_site(SameSite::Lax)
    .with_expiry(Expiry::OnInactivity(Duration::days(7)));
# let _ = sessions;
