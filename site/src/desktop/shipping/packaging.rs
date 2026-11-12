//! The page on building packages with `cargo packager`.

use crate::Site;
use crate::code::{self, Lang};
use crate::doc::{DesktopPage, Doc};
use crate::lang::L;
use crate::prose::{Part, bullets, heading, lead, note, para, parts, table_of};

/// The commands that install cargo-packager and package examples/notes.
const PACKAGE: &str = "cargo install cargo-packager --version 0.11.8 --locked
cargo packager -p example-notes --release";

/// The packaging table of examples/notes/Cargo.toml, without its `version`.
const TABLE: &str = r#"[package.metadata.packager]
product-name = "hypersig notes"
identifier = "dev.hypersig.example-notes"
before-packaging-command = "cargo build --release -p example-notes"
binaries = [{ path = "example-notes", main = true }]
icons = ["icons/icon.png"]"#;

/// The commands that make a key pair and sign the packages with it.
const SIGN: &str = "cargo packager signer generate --path ~/.keys/notes.key
CARGO_PACKAGER_SIGN_PRIVATE_KEY=~/.keys/notes.key \\
CARGO_PACKAGER_SIGN_PRIVATE_KEY_PASSWORD='the password' \\
cargo packager -p example-notes --release";

pub(crate) fn page(site: Site, head: Part) -> Part {
    parts(vec![
        head,
        lead(site, Doc::Desktop(DesktopPage::Packaging).leads()),
        heading(
            site,
            L {
                en: "Install cargo-packager",
                ja: "cargo-packager をインストールする",
            },
        ),
        para(
            site,
            L {
                en: "hypersig_desktop builds no packages itself. [cargo-packager](https://github.com/crabnebula-dev/cargo-packager) reads a table in the application's `Cargo.toml`, builds the release binary, and wraps it in the formats each system installs from. The repository's CI packages `examples/notes` with version 0.11.8 on macOS, Windows, and Linux, so install that version:",
                ja: "hypersig_desktop 自体はパッケージを作りません。[cargo-packager](https://github.com/crabnebula-dev/cargo-packager) がアプリの `Cargo.toml` の設定を読み、リリースビルドのバイナリを作り、それを各 OS のインストール形式にまとめます。リポジトリの CI はバージョン 0.11.8 で `examples/notes` を macOS、Windows、Linux 向けにパッケージにしています。そのため、同じバージョンをインストールします。",
            },
        ),
        code::block(Lang::Shell, None, PACKAGE),
        para(
            site,
            L {
                en: "The second command, run from the workspace's root, packages the crate `-p` names. `--release` packages the binary under `target/release`, and the packages are written there too.",
                ja: "2 つめのコマンドをワークスペースのルートで実行すると、`-p` で指定したクレートをパッケージにします。`--release` を付けると `target/release` にあるバイナリをパッケージにし、できたパッケージも同じディレクトリに書き出します。",
            },
        ),
        heading(
            site,
            L {
                en: "The packaging table",
                ja: "パッケージの設定",
            },
        ),
        para(
            site,
            L {
                en: "cargo-packager reads `[package.metadata.packager]` from the `Cargo.toml` of the crate it packages. This is the table of `examples/notes`, without the `version` line it also has:",
                ja: "cargo-packager はパッケージにするクレートの `Cargo.toml` から `[package.metadata.packager]` を読みます。次は `examples/notes` の設定で、`version` の行だけを除いています。",
            },
        ),
        code::block(Lang::Toml, Some("examples/notes/Cargo.toml"), TABLE),
        table_of(
            site,
            L {
                en: &["Key", "What it sets"][..],
                ja: &["キー", "設定する内容"][..],
            },
            L {
                en: &[
                    &[
                        "`product-name`",
                        "The application's name, which the packages take, as in `hypersig notes.app`.",
                    ][..],
                    &[
                        "`identifier`",
                        "The reverse domain name of the package. Give it the identifier `App::new` names.",
                    ],
                    &[
                        "`before-packaging-command`",
                        "The command run first, which builds the release binary.",
                    ],
                    &[
                        "`binaries`",
                        "The binaries to package, by file name under `target/release`. `main = true` marks the one the package starts.",
                    ],
                    &[
                        "`icons`",
                        "The icon files, relative to the directory of this `Cargo.toml`.",
                    ],
                    &[
                        "`version`",
                        "The version of the package. Without it, cargo-packager takes the crate's `version`.",
                    ],
                ][..],
                ja: &[
                    &[
                        "`product-name`",
                        "アプリの名前です。パッケージの名前にもなり、`hypersig notes.app` のようになります。",
                    ][..],
                    &[
                        "`identifier`",
                        "パッケージの逆ドメイン形式の識別子です。`App::new` に渡した識別子と同じにします。",
                    ],
                    &[
                        "`before-packaging-command`",
                        "最初に実行するコマンドで、リリースビルドのバイナリを作ります。",
                    ],
                    &[
                        "`binaries`",
                        "パッケージに入れるバイナリを、`target/release` の中のファイル名で指定します。`main = true` は、パッケージが起動するバイナリを示します。",
                    ],
                    &[
                        "`icons`",
                        "アイコンのファイルです。この `Cargo.toml` があるディレクトリからの相対パスで書きます。",
                    ],
                    &[
                        "`version`",
                        "パッケージのバージョンです。省略すると、cargo-packager はクレートの `version` を使います。",
                    ],
                ][..],
            },
        ),
        para(
            site,
            L {
                en: "The identifier decides where the web view keeps its data. On macOS, WKWebView keeps each application's cookies and `localStorage` by the bundle's identifier, and on Windows and Linux they are kept under the directory [`App::data_dir`](api:hypersig_desktop/struct.App.html#method.data_dir) returns, which `App::new` names. With the two identifiers equal, the packaged application and its data are found under one name on every system. CI checks that the `.app` of `examples/notes` names `dev.hypersig.example-notes` as its bundle identifier.",
                ja: "識別子は、Web ビューがデータを置く場所を決めます。macOS の WKWebView はアプリの Cookie と `localStorage` をバンドルの識別子ごとに保存します。Windows と Linux では、`App::new` に渡した識別子で決まる [`App::data_dir`](api:hypersig_desktop/struct.App.html#method.data_dir) のディレクトリに保存します。2 つの識別子を同じにしておけば、パッケージにしたアプリとそのデータは、どの OS でも 1 つの名前で見つかります。CI は、`examples/notes` の `.app` がバンドルの識別子として `dev.hypersig.example-notes` を持つことを確かめています。",
            },
        ),
        para(
            site,
            L {
                en: "Leave `version` out, or keep it equal to the crate's `version`. [Updates](desktop/updates/) compares `env!(\"CARGO_PKG_VERSION\")`, the crate's version, with the one the update endpoint announces, so a package built under another version is offered an update it already is, or misses one. Nothing checks that the two agree.",
                ja: "`version` は省略するか、クレートの `version` と同じにします。[更新の配布](desktop/updates/) ではクレートのバージョンである `env!(\"CARGO_PKG_VERSION\")` と、更新のエンドポイントが知らせるバージョンを比べます。別のバージョンで作ったパッケージには、すでに入っているバージョンが更新として届いたり、届くはずの更新が届かなかったりします。2 つが一致しているかどうかは検査されません。",
            },
        ),
        heading(
            site,
            L {
                en: "The icon",
                ja: "アイコン",
            },
        ),
        para(
            site,
            L {
                en: "The AppImage needs a square icon: without one, `cargo packager` stops on Linux with `Could not find a square icon to use as AppImage icon`. `examples/notes/icons/icon.png` is a 256 by 256 PNG, which also becomes the `.app`'s icon on macOS. `icons` takes glob patterns, so `icons/*.png` lists every PNG in that directory.",
                ja: "AppImage には正方形のアイコンが必要です。ないと、Linux の `cargo packager` は `Could not find a square icon to use as AppImage icon` と表示して止まります。`examples/notes/icons/icon.png` は 256 × 256 の PNG で、macOS では `.app` のアイコンにもなります。`icons` には glob のパターンを書けるので、`icons/*.png` と書けばそのディレクトリの PNG をすべて指定できます。",
            },
        ),
        heading(
            site,
            L {
                en: "Keys of the macOS Info.plist",
                ja: "macOS の Info.plist のキー",
            },
        ),
        para(
            site,
            L {
                en: "cargo-packager writes the `.app`'s `Info.plist` and adds every key of the file `[package.metadata.packager.macos]` names as `info-plist-path`, read from the crate's directory. A page that uses [the microphone or the camera](desktop/devices/) needs `NSMicrophoneUsageDescription` or `NSCameraUsageDescription` there, and `examples/meter` adds the first:",
                ja: "cargo-packager は `.app` の `Info.plist` を書き出し、`[package.metadata.packager.macos]` の `info-plist-path` で指定したファイルのキーをすべて加えます。このファイルはクレートのディレクトリから読みます。[マイクやカメラ](desktop/devices/)を使うページには、ここに `NSMicrophoneUsageDescription` か `NSCameraUsageDescription` が必要です。`examples/meter` は前者を加えています。",
            },
        ),
        code::block(
            Lang::Toml,
            Some("examples/meter/Cargo.toml"),
            "[package.metadata.packager.macos]\ninfo-plist-path = \"Info.plist\"",
        ),
        para(
            site,
            L {
                en: "CI packages the meter on macOS and checks that its `.app` has the key.",
                ja: "CI は macOS で meter の例のパッケージを作り、その `.app` にキーがあることを確かめています。",
            },
        ),
        heading(
            site,
            L {
                en: "What each system gets",
                ja: "OS ごとにできるパッケージ",
            },
        ),
        para(
            site,
            L {
                en: "Without `--formats`, cargo-packager builds the formats of the system it runs on, and CI runs it this way on each system:",
                ja: "`--formats` を指定しなければ、cargo-packager は実行した OS の形式でパッケージを作ります。CI は各 OS でこの方法で実行しています。",
            },
        ),
        table_of(
            site,
            L {
                en: &["System", "Packages"][..],
                ja: &["OS", "パッケージ"][..],
            },
            L {
                en: &[
                    &["macOS", "an `.app` bundle and a `.dmg` disk image"][..],
                    &["Windows", "an NSIS installer (`.exe`)"],
                    &["Linux", "a `.deb`, an AppImage, and a pacman package"],
                ][..],
                ja: &[
                    &["macOS", "`.app` バンドルと `.dmg` ディスクイメージ"][..],
                    &["Windows", "NSIS のインストーラー（`.exe`）"],
                    &["Linux", "`.deb`、AppImage、pacman のパッケージ"],
                ][..],
            },
        ),
        para(
            site,
            L {
                en: "A package holds a binary for the system and architecture it was built on, so build each system's packages on that system. The binary is built with the features `before-packaging-command` names; the `smoke` feature of [Smoke runs](desktop/smoke-runs/) stays out of a release.",
                ja: "パッケージには、作った OS とアーキテクチャのバイナリが入ります。そのため、各 OS のパッケージはその OS で作ります。バイナリは `before-packaging-command` で指定した feature でビルドされます。[CI でのスモークテスト](desktop/smoke-runs/) の `smoke` feature は、リリースのビルドには入れません。",
            },
        ),
        heading(
            site,
            L {
                en: "Code signing",
                ja: "コード署名",
            },
        ),
        para(
            site,
            L {
                en: "Without a code signature, macOS refuses to open the downloaded `.app`, and Windows warns before it runs the installer. cargo-packager signs while it packages. These are the settings and variables its 0.11.8 source reads:",
                ja: "コード署名がなければ、macOS はダウンロードした `.app` を開かず、Windows はインストーラーを実行する前に警告を出します。cargo-packager はパッケージを作りながら署名します。次は、0.11.8 のソースが読む設定と環境変数です。",
            },
        ),
        bullets(
            site,
            &[
                L {
                    en: "macOS: `signing-identity` in `[package.metadata.packager.macos]`, such as `Developer ID Application: NAME (TEAM_ID)`, turns signing on. `APPLE_CERTIFICATE`, the base64 of the `.p12` file, and `APPLE_CERTIFICATE_PASSWORD` import the certificate into a keychain of its own, as CI needs. The binary is signed with the hardened runtime (`--options runtime`), and `entitlements` names the file of entitlements it is signed with.",
                    ja: "macOS: `[package.metadata.packager.macos]` の `signing-identity`（`Developer ID Application: NAME (TEAM_ID)` のような値）を指定すると署名します。`APPLE_CERTIFICATE`（`.p12` ファイルを base64 にしたもの）と `APPLE_CERTIFICATE_PASSWORD` を渡すと、CI で必要なように、証明書を専用のキーチェーンに読み込みます。バイナリは Hardened Runtime（`--options runtime`）で署名され、`entitlements` で指定したファイルのエンタイトルメントが付きます。",
                },
                L {
                    en: "macOS notarization follows signing when `APPLE_ID`, `APPLE_PASSWORD` (an app-specific password), and `APPLE_TEAM_ID` are set, or `APPLE_API_KEY`, `APPLE_API_ISSUER`, and `APPLE_API_KEY_PATH`, or `APPLE_KEYCHAIN_PROFILE`. It sends the `.app` to `notarytool` and staples the ticket. Without any of them, it warns and skips notarization.",
                    ja: "macOS の公証は `APPLE_ID`、`APPLE_PASSWORD`（App 用パスワード）、`APPLE_TEAM_ID` の組か、`APPLE_API_KEY`、`APPLE_API_ISSUER`、`APPLE_API_KEY_PATH` の組か、`APPLE_KEYCHAIN_PROFILE` があるときに、署名に続けて行われます。`.app` を `notarytool` に送り、チケットをステープルします。どれもなければ、警告を出して公証を飛ばします。",
                },
                L {
                    en: "The hardened runtime blocks the microphone and the camera unless the entitlements hold `com.apple.security.device.audio-input` or `com.apple.security.device.camera` as `true`. `examples/meter` names `Entitlements.plist`, which holds the first, for an application that [allows the microphone](desktop/devices/).",
                    ja: "Hardened Runtime では、エンタイトルメントに `com.apple.security.device.audio-input` か `com.apple.security.device.camera` を `true` で入れないと、マイクやカメラを使えません。`examples/meter` は、[マイクを許可した](desktop/devices/)アプリのために、前者を入れた `Entitlements.plist` を指定しています。",
                },
                L {
                    en: "Windows: `certificate-thumbprint` and `digest-algorithm` (SHA-256 is recommended) in `[package.metadata.packager.windows]` sign with `signtool.exe` from a certificate in the Windows store, with `timestamp-url` for the timestamp. `sign-command`, a command with `%1` for the file, signs some other way, such as with `osslsigncode`.",
                    ja: "Windows: `[package.metadata.packager.windows]` の `certificate-thumbprint` と `digest-algorithm`（SHA-256 を推奨）を指定すると、Windows の証明書ストアの証明書で `signtool.exe` が署名します。タイムスタンプには `timestamp-url` を使います。`sign-command`（ファイルの場所に `%1` を書いたコマンド）を指定すると、`osslsigncode` のような別の方法で署名できます。",
                },
            ],
        ),
        note(
            site,
            L {
                en: "No run in this repository has signed or notarized a package, since it needs the owner's Apple Developer account and Windows certificate. [Issue #226](https://github.com/imaimai17468/hypersig/issues/226) tracks it.",
                ja: "持ち主の Apple Developer アカウントと Windows の証明書が必要なので、このリポジトリでは、まだどの実行でもパッケージに署名したり公証したりしていません。[issue #226](https://github.com/imaimai17468/hypersig/issues/226) で扱っています。",
            },
        ),
        heading(
            site,
            L {
                en: "Signing the packages for updates",
                ja: "更新のためにパッケージに署名する",
            },
        ),
        para(
            site,
            L {
                en: "[An `Updater`](desktop/updates/) installs only a package whose signature its public key verifies. `cargo packager signer generate` makes the key pair: with `--path`, it writes the private key to that file and the public key to the same path with `.pub` added. With `CARGO_PACKAGER_SIGN_PRIVATE_KEY` set to the private key, or to the path of its file, and `CARGO_PACKAGER_SIGN_PRIVATE_KEY_PASSWORD` to its password, `cargo packager` signs each package it builds:",
                ja: "[`Updater`](desktop/updates/) がインストールするのは、公開鍵で署名を確かめられたパッケージだけです。鍵のペアは `cargo packager signer generate` で作ります。`--path` を付けると、そのファイルに秘密鍵を、同じパスの末尾に `.pub` を付けたファイルに公開鍵を書き出します。`CARGO_PACKAGER_SIGN_PRIVATE_KEY` に秘密鍵そのものかそのファイルのパスを、`CARGO_PACKAGER_SIGN_PRIVATE_KEY_PASSWORD` にそのパスワードを設定すると、`cargo packager` は作った各パッケージに署名します。",
            },
        ),
        code::block(Lang::Shell, None, SIGN),
        para(
            site,
            L {
                en: "In cargo-packager 0.11.8's source, signing writes the signature beside each package as a file with `.sig` added to its name, and first packs the `.app` directory into an `.app.tar.gz`, which it signs in place of the directory. The repository's CI packages without a key, so it does not run this step.",
                ja: "cargo-packager 0.11.8 のソースでは、署名は各パッケージの隣に、名前の末尾に `.sig` を付けたファイルとして書き出されます。`.app` はディレクトリなので、先に `.app.tar.gz` にまとめ、そちらに署名します。リポジトリの CI は鍵を設定せずにパッケージを作るので、この手順は CI では実行していません。",
            },
        ),
        para(
            site,
            L {
                en: "Keep the private key and its password out of the repository. Anyone who holds them can sign a package that every installed copy of the application accepts as an update.",
                ja: "秘密鍵とそのパスワードはリポジトリに入れないでください。それを持つ人は誰でも、インストール済みのアプリが更新として受け入れるパッケージに署名できます。",
            },
        ),
        heading(
            site,
            L {
                en: "Next",
                ja: "次に読むページ",
            },
        ),
        para(
            site,
            L {
                en: "[Updates](desktop/updates/) finds a signed package from the running application and installs it. [The application](desktop/app/) covers the identifier `App::new` takes.",
                ja: "[更新の配布](desktop/updates/) では署名したパッケージを動いているアプリから見つけてインストールします。`App::new` に渡す識別子は [アプリの設定](desktop/app/) で説明しています。",
            },
        ),
    ])
}
