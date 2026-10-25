//! Actions that receive the files picked in a file input.

use std::fmt;
use std::marker::PhantomData;

use serde_json::json;

use crate::action::Call;
use crate::expr::{IntoExpr, Stmt};
use crate::html::{El, attr, input};
use crate::route::Base;

/// The value of [`Upload::DEFAULT_MAX_BYTES`], which `routes!` names without a type.
pub(crate) const DEFAULT_MAX_BYTES: usize = 8 * 1024 * 1024;

/// A server endpoint on the page whose signals are `S`, which receives
/// arguments `A` and the files picked in a [`FileInput`].
///
/// Declare one on an `upload` line of [`routes!`](crate::routes), which
/// names its handler, as with an [`Action`](crate::Action), and call it from
/// the page with [`Upload::call`] or [`Upload::call_with`]. The handler
/// receives an [`Uploaded<A>`](crate::Uploaded), or an
/// [`UploadStream<A>`](crate::UploadStream) where the line writes `stream`
/// before the path. The browser sends a `multipart/form-data` body, which a
/// form on another site can send too, so the runtime adds a header such a
/// form cannot set, and the router refuses a request without it.
///
/// The body may be [`Upload::DEFAULT_MAX_BYTES`] long unless the line writes
/// `max_bytes` and a number after the path. A longer one gets `413 Payload
/// Too Large`.
///
/// ```
/// use hypersig::html::{attr, button, div};
/// use hypersig::{FileInput, Patches, Signals, Upload, Uploaded};
/// use serde::{Deserialize, Serialize};
///
/// #[derive(Signals, Serialize, Deserialize)]
/// struct Profile {
///     uploading: bool,
/// }
///
/// const PICTURE: FileInput<Profile> = FileInput::new("picture");
///
/// hypersig::routes! {
///     fn uploads(Router<()>);
///     upload AVATAR: Upload<Profile> = "/avatar" max_bytes 1024 * 1024 => avatar;
/// }
///
/// async fn avatar((): (), uploaded: Uploaded<()>) -> Patches<Profile> {
///     let _picture = uploaded.files().first();
///     Profile::uploading().patch(false).into()
/// }
///
/// let _form = div()
///     .child(PICTURE.input().attr(attr::ACCEPT, "image/*"))
///     .child(button().on_click(AVATAR.call(PICTURE).indicator(Profile::uploading())).text("Upload"));
/// ```
pub struct Upload<S, A = ()> {
    base: Base,
    path: &'static str,
    max_bytes: usize,
    marker: PhantomData<fn(S, A)>,
}

impl<S, A> Upload<S, A> {
    /// The longest body an upload accepts unless its line in `routes!` says otherwise: 8 MiB.
    pub const DEFAULT_MAX_BYTES: usize = DEFAULT_MAX_BYTES;

    /// An upload served on `POST path` under `base`, accepting a body of at
    /// most `max_bytes` bytes, the arguments and the multipart framing
    /// included. [`routes!`](crate::routes) calls it in a `const`, so a path
    /// that does not start with `/` is a compile error.
    pub(crate) const fn at(base: Base, path: &'static str, max_bytes: usize) -> Self {
        assert!(
            !path.is_empty() && path.as_bytes()[0] == b'/',
            "an upload path starts with `/`"
        );
        Self {
            base,
            path,
            max_bytes,
            marker: PhantomData,
        }
    }

    /// The path the upload is served on, without its [`Base`].
    #[must_use]
    pub fn path(&self) -> &'static str {
        self.path
    }

    /// The longest body the upload accepts, in bytes.
    #[must_use]
    pub fn limit(&self) -> usize {
        self.max_bytes
    }

    /// A call that sends the files picked in `files` and `args`.
    ///
    /// `args` is evaluated in the browser when the call runs, as for
    /// [`Action::call_with`](crate::Action::call_with). A file input with
    /// no file picked sends no file.
    #[must_use]
    pub fn call_with(&self, files: FileInput<S>, args: impl IntoExpr<S, A>) -> Call<S> {
        Call::upload(
            self.base,
            self.path,
            args.into_expr().into_json(),
            files.name,
        )
    }
}

impl<S> Upload<S, ()> {
    /// A call that sends the files picked in `files`, for an upload that takes no arguments.
    #[must_use]
    pub fn call(&self, files: FileInput<S>) -> Call<S> {
        Call::upload(self.base, self.path, serde_json::Value::Null, files.name)
    }
}

impl<S, A> Clone for Upload<S, A> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<S, A> Copy for Upload<S, A> {}

impl<S, A> fmt::Debug for Upload<S, A> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Upload")
            .field("base", &self.base.as_str())
            .field("path", &self.path)
            .field("max_bytes", &self.max_bytes)
            .finish()
    }
}

/// A file input on a page whose signals are `S`, whose files an [`Upload`] sends.
///
/// Declare one as a `const`, put [`FileInput::input`] on the page, and pass
/// the same value to the call. Inside a list item the browser looks for the
/// input in the same item first, as for a [`NodeRef`](crate::NodeRef). The
/// browser looks up file inputs apart from node refs, so the two never share
/// a name. Two file inputs with one name are checked as two node refs with
/// one name are.
pub struct FileInput<S> {
    name: &'static str,
    marker: PhantomData<fn() -> S>,
}

impl<S> FileInput<S> {
    /// A file input named `name`.
    ///
    /// # Panics
    ///
    /// Panics if `name` is empty. In a `const`, that is a compile error.
    #[must_use]
    pub const fn new(name: &'static str) -> Self {
        assert!(!name.is_empty(), "a file input name is never empty");
        Self {
            name,
            marker: PhantomData,
        }
    }

    /// The `<input type="file">` element. Add `multiple` or `accept` to it as to any element.
    #[must_use]
    pub fn input(self) -> El<S> {
        input().attr(attr::TYPE, "file").file_input(self.name)
    }

    /// A statement that empties the input, such as after its files were sent.
    #[must_use]
    pub fn clear(self) -> Stmt<S> {
        Stmt::new(json!(["clear-files", self.name]))
    }
}

impl<S> Clone for FileInput<S> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<S> Copy for FileInput<S> {}

impl<S> fmt::Debug for FileInput<S> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("FileInput").field(&self.name).finish()
    }
}

/// What an upload's handler receives: the arguments and the files.
#[cfg(feature = "axum")]
#[derive(Debug)]
pub struct Uploaded<A> {
    args: A,
    files: Vec<UploadedFile>,
}

#[cfg(feature = "axum")]
impl<A> Uploaded<A> {
    pub(crate) fn new(args: A, files: Vec<UploadedFile>) -> Self {
        Self { args, files }
    }

    /// The arguments the call passed.
    #[must_use]
    pub fn args(&self) -> &A {
        &self.args
    }

    /// The files, in the order the visitor picked them.
    #[must_use]
    pub fn files(&self) -> &[UploadedFile] {
        &self.files
    }

    /// The arguments and the files, to take ownership of both.
    #[must_use]
    pub fn into_parts(self) -> (A, Vec<UploadedFile>) {
        (self.args, self.files)
    }
}

/// One file of an upload, read into memory.
#[cfg(feature = "axum")]
#[derive(Debug, Clone)]
pub struct UploadedFile {
    file_name: Option<String>,
    content_type: Option<String>,
    bytes: Vec<u8>,
}

#[cfg(feature = "axum")]
impl UploadedFile {
    pub(crate) fn new(
        file_name: Option<String>,
        content_type: Option<String>,
        bytes: Vec<u8>,
    ) -> Self {
        Self {
            file_name,
            content_type,
            bytes,
        }
    }

    /// The file's name on the visitor's computer, as the browser sent it.
    ///
    /// The visitor controls it, so it is no safe path to write to.
    #[must_use]
    pub fn file_name(&self) -> Option<&str> {
        self.file_name.as_deref()
    }

    /// The media type the browser sent, such as `image/png`. The browser
    /// guesses it from the file name, so check the bytes before trusting it.
    #[must_use]
    pub fn content_type(&self) -> Option<&str> {
        self.content_type.as_deref()
    }

    /// The file's contents.
    #[must_use]
    pub fn bytes(&self) -> &[u8] {
        &self.bytes
    }

    /// The file's contents, owned.
    #[must_use]
    pub fn into_bytes(self) -> Vec<u8> {
        self.bytes
    }
}

/// What a streaming upload's handler receives: the arguments, then the files
/// one at a time, each in chunks as they arrive.
///
/// An `upload` line of [`routes!`](crate::routes) that writes `stream`
/// before its path hands one to its handler, which keeps no file
/// in memory unless it collects the chunks itself. The arguments are read
/// before the handler runs, because the runtime sends them as the first part
/// of the body. The limit of the [`Upload`] holds while the handler reads:
/// past it, the read fails with a [`RequestError`](crate::RequestError)
/// whose status is `413 Payload Too Large`, which the handler can return as
/// its error.
///
/// # Examples
///
/// A handler that counts the bytes of each file without holding one in
/// memory. Writing each chunk to a file works the same way.
///
/// ```
/// use axum::Router;
/// use hypersig::{Patches, RequestError, Signals, Upload, UploadStream};
/// use serde::{Deserialize, Serialize};
///
/// #[derive(Signals, Serialize, Deserialize)]
/// struct Library {
///     received: u64,
/// }
///
/// hypersig::routes! {
///     fn uploads(Router<()>);
///     upload VIDEO: Upload<Library> = stream "/video" max_bytes 512 * 1024 * 1024 => receive;
/// }
///
/// async fn receive((): (), mut upload: UploadStream<()>) -> Result<Patches<Library>, RequestError> {
///     let mut received = 0;
///     while let Some(mut file) = upload.next_file().await? {
///         while let Some(chunk) = file.chunk().await? {
///             received += chunk.len() as u64;
///         }
///     }
///     Ok(Library::received().patch(received).into())
/// }
///
/// let _app: Router = uploads(Router::new());
/// ```
#[cfg(feature = "axum")]
#[derive(Debug)]
pub struct UploadStream<A> {
    args: A,
    multipart: multer::Multipart<'static>,
}

#[cfg(feature = "axum")]
impl<A> UploadStream<A> {
    pub(crate) fn new(args: A, multipart: multer::Multipart<'static>) -> Self {
        Self { args, multipart }
    }

    /// The arguments the call passed.
    #[must_use]
    pub fn args(&self) -> &A {
        &self.args
    }

    /// The next file, in the order the visitor picked them, or `None` after
    /// the last one.
    ///
    /// The part of the previous file that was not read is skipped, and so is
    /// a part other than a file, as an `upload` line without `stream` skips it.
    ///
    /// # Errors
    ///
    /// Returns a [`RequestError`](crate::RequestError) when the body is
    /// malformed or longer than the upload's limit.
    pub async fn next_file(&mut self) -> Result<Option<StreamedFile<'_>>, crate::RequestError> {
        // A multer field owns its part, so a part skipped here holds no
        // borrow into the next turn of the loop.
        while let Some(field) = self
            .multipart
            .next_field()
            .await
            .map_err(crate::RequestError::from_multer)?
        {
            if field.name() == Some(FILES_PART) {
                return Ok(Some(StreamedFile {
                    field,
                    upload: PhantomData,
                }));
            }
        }
        Ok(None)
    }
}

/// One file of a streaming upload, read in chunks.
#[cfg(feature = "axum")]
#[derive(Debug)]
pub struct StreamedFile<'a> {
    field: multer::Field<'static>,
    /// Ties the file to the `&mut` borrow of the stream it came from, so the
    /// next file cannot be asked for while this one is held.
    upload: PhantomData<&'a mut ()>,
}

#[cfg(feature = "axum")]
impl StreamedFile<'_> {
    /// The file's name on the visitor's computer, as the browser sent it.
    ///
    /// The visitor controls it, so it is no safe path to write to.
    #[must_use]
    pub fn file_name(&self) -> Option<&str> {
        self.field.file_name()
    }

    /// The media type the browser sent, such as `image/png`. The browser
    /// guesses it from the file name, so check the bytes before trusting it.
    #[must_use]
    pub fn content_type(&self) -> Option<&str> {
        self.field.content_type().map(AsRef::as_ref)
    }

    /// The next chunk of the file's contents, or `None` after the last one.
    ///
    /// # Errors
    ///
    /// Returns a [`RequestError`](crate::RequestError) when the body is
    /// malformed or longer than the upload's limit.
    pub async fn chunk(&mut self) -> Result<Option<axum::body::Bytes>, crate::RequestError> {
        self.field
            .chunk()
            .await
            .map_err(crate::RequestError::from_multer)
    }
}

/// The name of the multipart part that holds the arguments, which the runtime
/// sends first.
#[cfg(feature = "axum")]
pub(crate) const ARGS_PART: &str = "args";

/// The name of the multipart parts that hold the files.
#[cfg(feature = "axum")]
pub(crate) const FILES_PART: &str = "files";
