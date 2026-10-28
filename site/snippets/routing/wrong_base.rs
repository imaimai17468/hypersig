use hypersig::Base;

// error[E0080]: evaluation panicked: a base path starts with `/`, does not
// end with `/`, and is not `/` alone
const APP: Base = Base::new("/app/");
# let _ = APP;
