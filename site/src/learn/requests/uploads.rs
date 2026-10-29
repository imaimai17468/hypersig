//! The page on uploads: `Upload` and `FileInput`, the request they send,
//! the handler's `Uploaded`, the limits, and streaming large files.

use crate::Site;
use crate::doc::{Doc, Learn};
use crate::lang::L;
use crate::learn::snippet::{Snippet, show};
use crate::prose::{Part, heading, lead, note, para, parts, subheading, table_of, title};

/// The article.
pub fn page(site: Site) -> Part {
    parts([
        title(Doc::Learn(Learn::Uploads).title(site)),
        lead(site, Doc::Learn(Learn::Uploads).leads()),
        declaring(site),
        the_request(site),
        the_handler(site),
        limits(site),
        large_files(site),
        refused(site),
        unchecked(site),
        next(site),
    ])
}

/// `Upload`, `FileInput`, and the compile error for an input of another page.
fn declaring(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "Uploads",
                ja: "ファイルのアップロード",
            },
        ),
        para(
            site,
            L {
                en: "An [`Upload<S, A>`](api:hypersig/struct.Upload.html) is an action that also receives the files picked in a [`FileInput<S>`](api:hypersig/struct.FileInput.html). Declare it on an `upload` line of `routes!` with its handler, put `FileInput::input` on the page, and pass the same constant to `call` or `call_with`.",
                ja: "[`Upload<S, A>`](api:hypersig/struct.Upload.html) は、[`FileInput<S>`](api:hypersig/struct.FileInput.html) で選んだファイルも受け取るアクションです。`routes!` の `upload` の行にハンドラと一緒に宣言し、ページに `FileInput::input` を置き、同じ定数を `call` か `call_with` に渡します。",
            },
        ),
        show(Snippet::Uploads),
        table_of(
            site,
            L {
                en: &["Item", "What it is"],
                ja: &["項目", "内容"],
            },
            L {
                en: &[
                    &[
                        "`const fn FileInput::new(name: &'static str) -> FileInput<S>`",
                        "A file input named `name`. An empty name in a `const` is a compile error",
                    ],
                    &[
                        "`FileInput::input(self) -> El<S>`",
                        "The `<input type=\"file\">`. Add `multiple` or `accept` to it as to any element",
                    ],
                    &[
                        "`FileInput::clear(self) -> Stmt<S>`",
                        "Empties the input, such as after its files were sent",
                    ],
                    &[
                        "`Upload::call(&self, files: FileInput<S>) -> Call<S>`",
                        "A call that sends the files. It exists only on `Upload<S, ()>`",
                    ],
                    &[
                        "`Upload::call_with(&self, files: FileInput<S>, args: impl IntoExpr<S, A>) -> Call<S>`",
                        "A call that sends the files and `args`, evaluated in the browser when the call runs",
                    ],
                ],
                ja: &[
                    &[
                        "`const fn FileInput::new(name: &'static str) -> FileInput<S>`",
                        "`name` という名前のファイル入力。`const` で名前が空だとコンパイルエラーです",
                    ],
                    &[
                        "`FileInput::input(self) -> El<S>`",
                        "`<input type=\"file\">`。ほかの要素と同じように `multiple` や `accept` を付けられます",
                    ],
                    &[
                        "`FileInput::clear(self) -> Stmt<S>`",
                        "入力を空にします。ファイルを送ったあとなどに使います",
                    ],
                    &[
                        "`Upload::call(&self, files: FileInput<S>) -> Call<S>`",
                        "ファイルを送る呼び出し。`Upload<S, ()>` にだけあります",
                    ],
                    &[
                        "`Upload::call_with(&self, files: FileInput<S>, args: impl IntoExpr<S, A>) -> Call<S>`",
                        "ファイルと `args` を送る呼び出し。`args` は呼び出しのときにブラウザで評価されます",
                    ],
                ],
            },
        ),
        para(
            site,
            L {
                en: "The call is a [`Call<S>`](api:hypersig/struct.Call.html) like an action's, so `indicator`, `error`, and `then` work as they do there. `PICTURE.input()` is an `<input type=\"file\">`, and the page above adds `accept=\"image/png\"` to it.",
                ja: "呼び出しはアクションと同じ [`Call<S>`](api:hypersig/struct.Call.html) なので、`indicator`、`error`、`then` もアクションと同じように使えます。`PICTURE.input()` は `<input type=\"file\">` で、上のページはそれに `accept=\"image/png\"` を付けています。",
            },
        ),
        para(
            site,
            L {
                en: "Both constants carry the page's type `S`, so a file input of another page does not compile:",
                ja: "どちらの定数もページの型 `S` を持つので、ほかのページのファイル入力はコンパイルできません。",
            },
        ),
        show(Snippet::UploadWrongPage),
    ])
}

/// The multipart request an upload sends.
fn the_request(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "What an upload sends",
                ja: "アップロードが送るもの",
            },
        ),
        para(
            site,
            L {
                en: "An upload is a `POST` with a `multipart/form-data` body. Its first part holds the arguments, and each file picked in the input follows in the order the visitor picked them. An input with no file picked sends no file. A form on another site can send a `multipart/form-data` body too, so the runtime adds a header to an upload that such a form cannot set, and the router refuses a request without it.",
                ja: "アップロードは本文が `multipart/form-data` の `POST` です。最初の部分が引数を持ち、そのあとに入力欄で選んだファイルが、訪問者が選んだ順に続きます。ファイルを選んでいない入力欄はファイルを送りません。`multipart/form-data` の本文は別のサイトのフォームからも送れるので、ランタイムはアップロードに、そうしたフォームが付けられないヘッダーを付けます。ルーターはこのヘッダーのないリクエストを拒否します。",
            },
        ),
    ])
}

/// `Uploaded`, `UploadedFile`, and `FileName`.
fn the_handler(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "The handler",
                ja: "ハンドラ",
            },
        ),
        para(
            site,
            L {
                en: "The handler receives an [`Uploaded<A>`](api:hypersig/struct.Uploaded.html) with the arguments and each file's name, media type, and bytes. It answers as an action's handler does, so it can return a `Result` and take an extractor.",
                ja: "ハンドラは引数と、ファイルごとの名前、メディアタイプ、中身を持つ [`Uploaded<A>`](api:hypersig/struct.Uploaded.html) を受け取ります。応答はアクションのハンドラと同じなので、`Result` を返したり、エクストラクタを受け取ったりできます。",
            },
        ),
        table_of(
            site,
            L {
                en: &["Method", "Returns"],
                ja: &["メソッド", "戻り値"],
            },
            L {
                en: &[
                    &["`Uploaded::args`", "`&A`, the arguments the call passed"],
                    &[
                        "`Uploaded::files`",
                        "`&[UploadedFile]`, in the order the visitor picked them",
                    ],
                    &[
                        "`Uploaded::into_parts`",
                        "`(A, Vec<UploadedFile>)`, to own both",
                    ],
                    &[
                        "`UploadedFile::file_name`",
                        "`Option<FileName<'_>>`, the name on the visitor's computer",
                    ],
                    &[
                        "`UploadedFile::content_type`",
                        "`Option<&str>`, the media type the browser sent, such as `image/png`",
                    ],
                    &[
                        "`UploadedFile::bytes`, `into_bytes`",
                        "`&[u8]` or `Vec<u8>`, the file's contents",
                    ],
                ],
                ja: &[
                    &["`Uploaded::args`", "`&A`。呼び出しが渡した引数"],
                    &[
                        "`Uploaded::files`",
                        "`&[UploadedFile]`。訪問者が選んだ順です",
                    ],
                    &[
                        "`Uploaded::into_parts`",
                        "`(A, Vec<UploadedFile>)`。両方の所有権を得ます",
                    ],
                    &[
                        "`UploadedFile::file_name`",
                        "`Option<FileName<'_>>`。訪問者のコンピュータでの名前",
                    ],
                    &[
                        "`UploadedFile::content_type`",
                        "`Option<&str>`。ブラウザが送ったメディアタイプ。たとえば `image/png`",
                    ],
                    &[
                        "`UploadedFile::bytes`、`into_bytes`",
                        "`&[u8]` か `Vec<u8>`。ファイルの中身",
                    ],
                ],
            },
        ),
        para(
            site,
            L {
                en: "A visitor can send any text as the name, such as `../../etc/passwd`, so it reaches the handler as a [`FileName`](api:hypersig/struct.FileName.html), not as a `&str` that a path could be joined with by accident. `FileName::base_name` is the part after the last `/` or `\\`, or `None` where that is empty, `.`, `..`, or holds a control character, and `FileName::as_sent` is the text as sent. Store a file under a name the application chooses, and use the base name for showing it or reading its extension.",
                ja: "訪問者は `../../etc/passwd` のようにどんな文字列でも名前として送れるので、名前はうっかりパスにつなげられる `&str` ではなく、[`FileName`](api:hypersig/struct.FileName.html) としてハンドラに届きます。`FileName::base_name` は最後の `/` か `\\` より後ろの部分で、それが空、`.`、`..` のときや制御文字を含むときは `None` です。`FileName::as_sent` は送られたままの文字列です。ファイルはアプリケーションが決めた名前で保存し、ベース名は表示や拡張子の判定に使ってください。",
            },
        ),
        note(
            site,
            L {
                en: "The file name and the media type come from the visitor's browser, so check the bytes before trusting them. The `auth` example accepts a picture only when its first bytes are those of a PNG or a JPEG.",
                ja: "ファイル名とメディアタイプは訪問者のブラウザが決めるので、信用する前に中身を検査してください。`auth` の例は、先頭のバイトが PNG か JPEG のものである画像だけを受け付けます。",
            },
        ),
    ])
}

/// The size limit of an upload.
fn limits(site: Site) -> Part {
    parts([
        subheading(
            site,
            L {
                en: "The size limit",
                ja: "サイズの上限",
            },
        ),
        para(
            site,
            L {
                en: "A body longer than `Upload::DEFAULT_MAX_BYTES` (8 MiB), or the limit that `max_bytes` after the path sets, gets a 413, before it is read when its `Content-Length` says so. The limit counts the whole body, the arguments and the multipart framing included, and `Upload::limit` returns it.",
                ja: "`Upload::DEFAULT_MAX_BYTES`（8 MiB）か、パスのあとの `max_bytes` で決めた上限を超える本文には 413 を返します。`Content-Length` が上限を超えていれば、本文を読む前に返します。上限は引数とマルチパートの区切りを含めた本文全体に対するもので、`Upload::limit` で読めます。",
            },
        ),
    ])
}

/// Streaming uploads.
fn large_files(site: Site) -> Part {
    parts([
        subheading(
            site,
            L {
                en: "Large files",
                ja: "大きなファイル",
            },
        ),
        para(
            site,
            L {
                en: "An `upload` line reads every file into memory before its handler runs, so keep its limit near the largest file you accept. For larger files, write `stream` before the path. Its handler receives an [`UploadStream<A>`](api:hypersig/struct.UploadStream.html): the arguments, already read because the runtime sends them first, and `next_file`, which yields each file for the handler to read with `chunk` as its bytes arrive.",
                ja: "`upload` の行は、ハンドラが動く前にすべてのファイルをメモリに読み込みます。そのため、上限は受け付ける最大のファイルに近い値にしてください。それより大きなファイルには、パスの前に `stream` を書きます。そのハンドラは [`UploadStream<A>`](api:hypersig/struct.UploadStream.html) を受け取ります。引数はランタイムが先頭で送るので読み終えてあり、`next_file` がファイルを 1 つずつ返します。ハンドラはファイルの中身を、届いた分ずつ `chunk` で読みます。",
            },
        ),
        show(Snippet::UploadStream),
        para(
            site,
            L {
                en: "Once the body passes the limit, `next_file` or `chunk` returns a [`RequestError`](api:hypersig/struct.RequestError.html) with status 413, which the handler can return with `?`. `next_file` skips the part of the previous file that was not read. The file that `next_file` returns borrows the stream, so the next file cannot be asked for while the handler holds it.",
                ja: "本文が上限を超えると、`next_file` か `chunk` がステータス 413 の [`RequestError`](api:hypersig/struct.RequestError.html) を返すので、ハンドラは `?` でそのまま返せます。`next_file` は、前のファイルの読んでいない部分を読み飛ばします。`next_file` が返すファイルはストリームを借用するので、ハンドラがそれを持っているあいだは次のファイルを求められません。",
            },
        ),
    ])
}

/// The requests an upload refuses.
fn refused(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "Requests an upload refuses",
                ja: "アップロードが拒否するリクエスト",
            },
        ),
        para(
            site,
            L {
                en: "Both kinds of line refuse these requests with a `RequestError`, and skip parts other than the arguments and the files:",
                ja: "どちらの行も、次のリクエストを `RequestError` で拒否し、引数とファイル以外の部分は読み飛ばします。",
            },
        ),
        table_of(
            site,
            L {
                en: &["Status", "Request"],
                ja: &["ステータス", "リクエスト"],
            },
            L {
                en: &[
                    &[
                        "403 Forbidden",
                        "A `Sec-Fetch-Site` other than `same-origin`, or no header of an upload from the page",
                    ],
                    &[
                        "415 Unsupported Media Type",
                        "A body that is not `multipart/form-data`",
                    ],
                    &["413 Payload Too Large", "A body longer than the limit"],
                    &[
                        "400 Bad Request",
                        "A malformed body, a first part that is not the arguments, or arguments that do not deserialize into `A`",
                    ],
                ],
                ja: &[
                    &[
                        "403 Forbidden",
                        "`same-origin` 以外の `Sec-Fetch-Site`、またはページからのアップロードに付くヘッダーがない",
                    ],
                    &[
                        "415 Unsupported Media Type",
                        "本文が `multipart/form-data` でない",
                    ],
                    &["413 Payload Too Large", "本文が上限より長い"],
                    &[
                        "400 Bad Request",
                        "本文の形が崩れている、最初の部分が引数でない、または引数が `A` にデシリアライズできない",
                    ],
                ],
            },
        ),
        para(
            site,
            L {
                en: "A handler's extractor runs before the body is read, so an upload from a visitor who is not signed in is refused before its files arrive.",
                ja: "ハンドラのエクストラクタは本文を読む前に動くので、サインインしていない訪問者のアップロードは、ファイルが届く前に拒否されます。",
            },
        ),
    ])
}

/// What the types leave to the page and the handler.
fn unchecked(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "What the compiler does not check",
                ja: "コンパイラが検査しないこと",
            },
        ),
        para(
            site,
            L {
                en: "The compiler does not check that the input is on the page. When the call finds no input of its name, the browser's console shows `hypersig: no file input named \"picture\"` and the upload goes out without files. Inside a list item, the browser looks for the input in the same item first.",
                ja: "入力欄がページにあるかどうかは、コンパイラは検査しません。呼び出しがその名前の入力欄を見つけられないと、ブラウザのコンソールに `hypersig: no file input named \"picture\"` と出て、アップロードはファイルなしで送られます。リスト項目の中では、ブラウザはまず同じ項目の中から入力欄を探します。",
            },
        ),
        para(
            site,
            L {
                en: "Two file inputs with one name, where one lookup can reach both, make `Page::render` panic before the page is served. Each item of a list may carry the same name. File inputs and [node refs](learn/node-refs/) are looked up apart, so a file input and a node ref never share a name.",
                ja: "1 回の検索で両方に届く位置に同じ名前のファイル入力が 2 つあると、`Page::render` はページを配信する前にパニックします。リストの項目はそれぞれ同じ名前を持てます。ファイル入力と [ノード参照](learn/node-refs/) は別々に探されるので、ファイル入力とノード参照の名前がぶつかることはありません。",
            },
        ),
    ])
}

/// The links to the pages that come next.
fn next(site: Site) -> Part {
    para(
        site,
        L {
            en: "Next, [Storing data](learn/database/) keeps what a handler receives, and [Security](learn/security/) lists what the router refuses and why.",
            ja: "次の [データを保存する](learn/database/) では、ハンドラが受け取ったものを保存します。ルーターが拒否するものとその理由は [セキュリティ](learn/security/) にあります。",
        },
    )
}
