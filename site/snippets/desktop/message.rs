# use hypersig_desktop::testing::{Choice, FakeOs};
use hypersig_desktop::{Answer, Level, Message, Os};

/// Asks before the changes to a note are lost, and says whether to close it.
async fn may_close(os: &Os, unsaved: bool) -> bool {
    if !unsaved {
        return true;
    }
    let message = Message::new(Level::Warning, "Save the changes to this note?")
        .body("They are lost if you close it without saving.");
    match os.ask(message).await {
        Answer::Yes => save().await,
        Answer::No => true,
        Answer::Cancel => false,
    }
}
# async fn save() -> bool { true }
# fn main() {
# tokio::runtime::Builder::new_current_thread().build().unwrap().block_on(async {
# let fake = FakeOs::new();
# fake.choose(Choice::Cancel);
# assert!(!may_close(&fake.os(), true).await);
# assert_eq!(fake.messages().len(), 1);
# });
# }
