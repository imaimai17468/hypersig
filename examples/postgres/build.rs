//! Rebuilds the example when a file the sqlx macros read changes.

fn main() {
    // `query_file!` reads `queries/`, `migrate!` reads `migrations/`, and an
    // offline build reads `.sqlx/`. A proc macro cannot tell cargo about a
    // file it reads, so without these lines an edited query keeps the old
    // build and its old check.
    println!("cargo::rerun-if-changed=queries");
    println!("cargo::rerun-if-changed=migrations");
    println!("cargo::rerun-if-changed=.sqlx");
}
