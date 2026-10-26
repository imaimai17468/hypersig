use hypersig::html::Id;

// error[E0080]: evaluation panicked: an id holds no whitespace
const EMAIL: Id = Id::new("e mail");
# let _ = EMAIL;
