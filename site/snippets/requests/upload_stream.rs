use axum::Router;
use hypersig::{Patches, RequestError, Signals, Upload, UploadStream};
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct Library {
    received: u64,
}

hypersig::routes! {
    fn pages(Router<()>);
    upload VIDEO: Upload<Library> = stream "/video" max_bytes 512 * 1024 * 1024 => receive;
}

// Each file arrives in chunks, and none is kept in memory unless the handler keeps it.
async fn receive((): (), mut upload: UploadStream<()>) -> Result<Patches<Library>, RequestError> {
    let mut received = 0;
    while let Some(mut file) = upload.next_file().await? {
        while let Some(chunk) = file.chunk().await? {
            // Write `chunk` to a file or a bucket here.
            received += chunk.len() as u64;
        }
    }
    Ok(Library::received().patch(received).into())
}

let _app: Router = pages(Router::new());
