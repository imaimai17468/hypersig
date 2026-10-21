use hypersig::SafeUrl;

const LINK: SafeUrl = SafeUrl::new("javascript:alert(1)");

fn main() {
    let _ = LINK;
}
