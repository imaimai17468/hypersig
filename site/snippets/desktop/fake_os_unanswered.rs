# use hypersig_desktop::testing::FakeOs;
# tokio::runtime::Builder::new_current_thread().build().unwrap().block_on(async {
let fake = FakeOs::new();
let os = fake.os();
// No `answer_dialog` or `cancel_dialog` came first, so this panics.
let _ = os.dialog().open_file().await;
# });
