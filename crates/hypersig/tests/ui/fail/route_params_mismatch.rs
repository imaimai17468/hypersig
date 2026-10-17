use hypersig::Route;

const MEMBER: Route<i32> = Route::new("/members");

fn main() {
    let _ = MEMBER.path();
}
