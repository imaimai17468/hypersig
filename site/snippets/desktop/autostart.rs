# use hypersig_desktop::testing::FakeOs;
use hypersig_desktop::Os;

/// Turns the start at login on or off from a switch on the settings page.
async fn set_start_at_login(os: &Os, on: bool) -> Result<(), hypersig_desktop::Error> {
    if on {
        os.autostart().enable().await
    } else {
        os.autostart().disable().await
    }
}
# fn main() {
# tokio::runtime::Builder::new_current_thread().build().unwrap().block_on(async {
# let fake = FakeOs::new();
# set_start_at_login(&fake.os(), true).await.unwrap();
# assert!(fake.os().autostart().is_enabled().await.unwrap());
# });
# }
