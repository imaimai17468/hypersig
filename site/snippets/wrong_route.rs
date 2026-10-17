use hypersig::Route;

// error[E0080]: evaluation panicked: the route's `{name}` segments differ in
// number from the values of its parameter type
const MEMBER: Route<i32> = Route::new("/members");
