//! Form state, and validation rules that the browser and the server both evaluate.

use std::collections::BTreeMap;
use std::error::Error as StdError;
use std::fmt;
use std::marker::PhantomData;

use serde::{Deserialize, Serialize};
use serde_json::{Map, Value, json};

use crate::expr::{Expr, IntoExpr, IntoStmt, Stmt, if_else};
use crate::html::{self, El};
use crate::patch::SignalPatch;
use crate::signal::{Path, Sig, Signals};
use crate::ssr::{self, Ctx};

/// The values of a form, the values it started from, and what the visitor has done to it.
///
/// Put one in the page's signal struct as a field of type `Form<T>`, where
/// `T` is a `Signals` struct of the form's values that implements
/// [`Validate`]. The methods on `Sig<S, Form<T>>` then read and change it:
/// [`field`](Sig::field) binds a control, [`error`](Sig::error) gives the
/// message to show under it, [`form`](Sig::form) checks the rules before
/// calling an action, and [`reset`](Sig::reset) puts the starting values back.
///
/// A form is dirty while its values differ from the values it started
/// from. A [`Sig::patch`] of one field from the server changes the value
/// and leaves the starting values alone, so the form stays dirty. A handler
/// that has saved the values patches the whole form with
/// `Form::new(saved)`, which makes the saved values the new starting
/// values and clears what the visitor has done.
///
/// ```
/// use hypersig::html::{button, input};
/// use hypersig::{Action, Form, Page, Rules, Sig, Signals, Validate};
/// use serde::{Deserialize, Serialize};
///
/// #[derive(Signals, Serialize, Deserialize, Clone)]
/// struct Signup {
///     email: String,
/// }
///
/// impl Validate for Signup {
///     fn rules<S>(form: Sig<S, Self>, rules: &mut Rules<S, Self>) {
///         let email = form.at(Signup::email());
///         rules.check(Signup::email(), email.contains("@"), "Enter an email address.");
///     }
/// }
///
/// #[derive(Signals, Serialize, Deserialize)]
/// struct Join {
///     signup: Form<Signup>,
/// }
///
/// const JOIN: Action<Join, Signup> = Action::post("/join");
///
/// let signup = Join::signup();
/// let email = Signup::email();
/// let page = Page::new("Join", Join { signup: Form::new(Signup { email: String::new() }) })
///     .child(
///         signup
///             .form(JOIN.call_with(signup.value()))
///             .child(input().bind(signup.field(email)).on_blur(signup.touch(email)))
///             .child(button().attr("type", "submit").text("Join")),
///     );
/// assert!(page.render().contains("signup.touched.email"));
/// ```
#[derive(Clone, Serialize, Deserialize)]
pub struct Form<T> {
    value: T,
    initial: T,
    /// `true` under the key of each field the visitor has left, from [`key`].
    touched: Map<String, Value>,
    submitted: bool,
    /// The message the server reported under the key of each field, from [`key`].
    errors: Map<String, Value>,
}

impl<T: Clone> Form<T> {
    /// A form that starts from `value`, untouched and not yet submitted.
    #[must_use]
    pub fn new(value: T) -> Self {
        Self {
            initial: value.clone(),
            value,
            touched: Map::new(),
            submitted: false,
            errors: Map::new(),
        }
    }
}

impl<T: Validate> Form<T> {
    /// Checks `value` against the rules of `T` on the server.
    ///
    /// The rules are the ones the browser checks, evaluated by the same
    /// evaluator that writes the first HTML, so a value the browser accepted
    /// passes here too. Call it in the action handler, because a request
    /// can come from something other than the page.
    ///
    /// # Errors
    ///
    /// Returns the message of the first failing rule of each field.
    ///
    /// # Panics
    ///
    /// Panics if a rule reads [`Expr::now`] or [`Expr::random_below`], which
    /// only the browser can evaluate, or if `value` fails to serialize.
    ///
    /// ```
    /// use hypersig::{Form, Rules, Sig, Signals, Validate};
    /// use serde::{Deserialize, Serialize};
    ///
    /// #[derive(Signals, Serialize, Deserialize)]
    /// struct Signup {
    ///     email: String,
    /// }
    ///
    /// impl Validate for Signup {
    ///     fn rules<S>(form: Sig<S, Self>, rules: &mut Rules<S, Self>) {
    ///         let email = form.at(Signup::email());
    ///         rules.check(Signup::email(), email.contains("@"), "Enter an email address.");
    ///     }
    /// }
    ///
    /// let errors = Form::validate(&Signup { email: "ada".to_owned() }).unwrap_err();
    /// assert_eq!(errors.message(Signup::email()), Some("Enter an email address."));
    /// assert!(Form::validate(&Signup { email: "ada@example.com".to_owned() }).is_ok());
    /// ```
    pub fn validate(value: &T) -> Result<(), FieldErrors<T>> {
        let mut rules = Rules::<Submission, T>::new();
        T::rules(Sig::from_path(Path::field("value")), &mut rules);
        let root = json!({ "value": value });
        let ctx = Ctx {
            root: &root,
            scope: Vec::new(),
            copy: false,
            vars: Vec::new(),
            rules: None,
        };
        let mut errors = FieldErrors::new();
        for check in &rules.checks {
            if errors.messages.contains_key(&check.key) {
                continue;
            }
            let evaluate = |ast| {
                ssr::evaluate(ast, &ctx).unwrap_or_else(|| {
                    panic!(
                        "a rule for `{}` reads a value only the browser has",
                        check.key
                    )
                })
            };
            if !ssr::truthy(&evaluate(check.ok.as_json())) {
                let message = ssr::js_string(&evaluate(check.message.as_json()));
                errors.messages.insert(check.key.clone(), message);
            }
        }
        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
}

impl<T: fmt::Debug> fmt::Debug for Form<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Form")
            .field("value", &self.value)
            .field("initial", &self.initial)
            .field("touched", &self.touched)
            .field("submitted", &self.submitted)
            .field("errors", &self.errors)
            .finish()
    }
}

/// The page that [`Form::validate`] evaluates the rules on, whose one field `value` holds the submitted values.
enum Submission {}

/// A struct of form values with rules that say which values are wrong.
///
/// `rules` builds expressions from `form`, the values on whatever page
/// they live, so the browser checks them as the visitor types and
/// [`Form::validate`] checks the same expressions on the server. A rule
/// that reads [`Expr::now`] or [`Expr::random_below`] cannot be checked on
/// the server, which nothing checks before [`Form::validate`] panics.
///
/// The default `rules` adds none, for a form that only needs its dirty
/// state and [`reset`](Sig::reset).
pub trait Validate: Signals {
    /// Adds a check to `rules` for each field of `form` that can be wrong.
    fn rules<S>(form: Sig<S, Self>, rules: &mut Rules<S, Self>) {
        let _ = (form, rules);
    }
}

/// The checks of a form whose values are `T`, on the page whose signals are `S`.
///
/// [`Validate::rules`] receives one and adds to it with [`Rules::check`].
pub struct Rules<S, T> {
    checks: Vec<Check<S>>,
    marker: PhantomData<fn() -> T>,
}

struct Check<S> {
    key: String,
    ok: Expr<S, bool>,
    message: Expr<S, String>,
}

impl<S, T> Rules<S, T> {
    fn new() -> Self {
        Self {
            checks: Vec::new(),
            marker: PhantomData,
        }
    }

    /// Reports `message` for `field` while `ok` is false.
    ///
    /// A field with several checks shows the message of the first one that
    /// fails, in the order they were added.
    pub fn check<U>(
        &mut self,
        field: Sig<T, U>,
        ok: impl IntoExpr<S, bool>,
        message: impl IntoExpr<S, String>,
    ) -> &mut Self {
        self.checks.push(Check {
            key: key(field),
            ok: ok.into_expr(),
            message: message.into_expr(),
        });
        self
    }

    /// The message of the first failing check of the field under `key`, or `""`.
    fn message(&self, key: &str) -> Option<Expr<S, String>> {
        self.checks
            .iter()
            .rev()
            .filter(|check| check.key == key)
            .fold(None, |rest, check| {
                let rest = rest.unwrap_or_else(|| Expr::lit(&String::new()));
                Some(if_else(check.ok.clone(), rest, check.message.clone()))
            })
    }

    fn all_pass(&self) -> Expr<S, bool> {
        self.checks
            .iter()
            .map(|check| check.ok.clone())
            .reduce(Expr::and)
            .unwrap_or_else(|| Expr::lit(&true))
    }
}

impl<S, T> fmt::Debug for Rules<S, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_list()
            .entries(self.checks.iter().map(|check| &check.key))
            .finish()
    }
}

/// The key a field's touched flag and server message are stored under.
///
/// A dotted path would make `get` and `set` descend into nested objects, and
/// then a check on a struct field and one on a field inside it would write
/// to the same place. Field names are Rust identifiers, so `/` cannot occur
/// in one.
fn key<T, U>(field: Sig<T, U>) -> String {
    field.raw_path().dotted().replace('.', "/")
}

/// Messages for the fields of a form whose values are `T`, by field.
///
/// [`Form::validate`] returns one, and a handler can build one for what only
/// the server knows, such as an email address that is already taken.
/// [`Sig::reject`] sends it to the page.
///
/// ```
/// use hypersig::{FieldErrors, Signals};
/// use serde::{Deserialize, Serialize};
///
/// #[derive(Signals, Serialize, Deserialize)]
/// struct Signup {
///     email: String,
/// }
///
/// let errors = FieldErrors::new().add(Signup::email(), "That address is taken.");
/// assert_eq!(errors.to_string(), "email: That address is taken.");
/// ```
pub struct FieldErrors<T> {
    /// Messages under the key of each field, from [`key`].
    messages: BTreeMap<String, String>,
    marker: PhantomData<fn() -> T>,
}

impl<T> FieldErrors<T> {
    /// No messages.
    #[must_use]
    pub fn new() -> Self {
        Self {
            messages: BTreeMap::new(),
            marker: PhantomData,
        }
    }

    /// These messages, with `message` for `field` in place of any it had.
    #[must_use]
    pub fn add<U>(mut self, field: Sig<T, U>, message: impl Into<String>) -> Self {
        self.messages.insert(key(field), message.into());
        self
    }

    /// The message for `field`, if it has one.
    #[must_use]
    pub fn message<U>(&self, field: Sig<T, U>) -> Option<&str> {
        self.messages.get(&key(field)).map(String::as_str)
    }

    /// Whether no field has a message.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.messages.is_empty()
    }
}

impl<T> Default for FieldErrors<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> fmt::Debug for FieldErrors<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_map().entries(&self.messages).finish()
    }
}

impl<T> fmt::Display for FieldErrors<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (i, (key, message)) in self.messages.iter().enumerate() {
            if i > 0 {
                f.write_str("; ")?;
            }
            write!(f, "{}: {message}", key.replace('/', "."))?;
        }
        Ok(())
    }
}

impl<T> StdError for FieldErrors<T> {}

impl<S, T> Sig<S, Form<T>> {
    /// The values of the form.
    #[must_use]
    pub fn value(self) -> Sig<S, T> {
        self.at(part("value"))
    }

    /// The field `field` of the form's values, for binding a control to it.
    #[must_use]
    pub fn field<U>(self, field: Sig<T, U>) -> Sig<S, U> {
        self.value().at(field)
    }

    /// Whether the form's values differ from the values it started from.
    ///
    /// The runtime compares the JSON forms, as [`Expr::equals`] does.
    #[must_use]
    pub fn is_dirty(self) -> Expr<S, bool> {
        let initial: Sig<S, T> = self.at(part("initial"));
        Expr::from_json(json!([
            "ne",
            self.value().expr().into_json(),
            initial.expr().into_json()
        ]))
    }

    /// Whether `field` differs from the value it started from.
    #[must_use]
    pub fn is_field_dirty<U>(self, field: Sig<T, U>) -> Expr<S, bool> {
        let initial: Sig<S, T> = self.at(part("initial"));
        Expr::from_json(json!([
            "ne",
            self.field(field).expr().into_json(),
            initial.at(field).expr().into_json()
        ]))
    }

    /// Whether the visitor has left `field`, or has submitted the form while `field` has a rule.
    #[must_use]
    pub fn is_touched<U>(self, field: Sig<T, U>) -> Expr<S, bool> {
        let touched: Sig<S, Value> = self.at(part("touched"));
        Expr::from_json(json!([
            "or_else",
            ["get", touched.expr().into_json(), key(field)],
            ["lit", false]
        ]))
    }

    /// Whether the visitor has submitted the form since it started or was reset.
    #[must_use]
    pub fn is_submitted(self) -> Expr<S, bool> {
        self.at(part::<T, bool>("submitted")).expr()
    }

    /// A statement that marks `field` as left by the visitor.
    ///
    /// Run it on [`El::on_blur`](crate::html::El::on_blur) of the control,
    /// so that [`error`](Sig::error) waits until the visitor has finished
    /// with the field.
    #[must_use]
    pub fn touch<U>(self, field: Sig<T, U>) -> Stmt<S> {
        self.touch_key(&key(field))
    }

    fn touch_key(self, key: &str) -> Stmt<S> {
        let place = format!("{}.touched.{key}", self.path());
        Stmt::set(Value::from(place), Expr::<S, bool>::lit(&true))
    }

    /// A statement that puts back the starting values and forgets what the visitor has done.
    #[must_use]
    pub fn reset(self) -> Stmt<S> {
        let empty = || Expr::<S, Value>::from_json(json!(["lit", {}]));
        self.value()
            .set(self.at(part::<T, T>("initial")))
            .then(self.at(part::<T, Value>("touched")).set(empty()))
            .then(self.at(part::<T, bool>("submitted")).set(false))
            .then(self.at(part::<T, Value>("errors")).set(empty()))
    }

    /// A patch that shows `errors` under their fields until the next submit.
    #[must_use]
    pub fn reject(self, errors: &FieldErrors<T>) -> SignalPatch<S> {
        SignalPatch::single(
            format!("{}.errors", self.raw_path().dotted()),
            &errors.messages,
        )
    }
}

impl<S, T: Validate> Sig<S, Form<T>> {
    fn rules(self) -> Rules<S, T> {
        let mut rules = Rules::new();
        T::rules(self.value(), &mut rules);
        rules
    }

    /// Whether every rule of `T` passes for the form's values.
    #[must_use]
    pub fn is_valid(self) -> Expr<S, bool> {
        self.rules().all_pass()
    }

    /// The message to show under `field`, or `""`.
    ///
    /// The message of the first failing rule shows once the field is
    /// [touched](Sig::is_touched). Where no rule fails, a message the
    /// server sent with [`reject`](Sig::reject) shows until the next submit.
    #[must_use]
    pub fn error<U>(self, field: Sig<T, U>) -> Expr<S, String> {
        let key = key(field);
        let errors: Sig<S, Value> = self.at(part("errors"));
        let server = Expr::<S, String>::from_json(json!([
            "or_else",
            ["get", errors.expr().into_json(), key],
            ["lit", ""]
        ]));
        match self.rules().message(&key) {
            None => server,
            Some(client) => if_else(self.is_touched(field), client, "")
                .let_in(|shown| if_else(shown.clone().is_empty(), server, shown)),
        }
    }

    /// A `<form>` that, on submit, touches every field with a rule and runs `then` if every rule passes.
    ///
    /// Submitting also marks the form submitted and clears the messages
    /// from [`reject`](Sig::reject). `then` is usually the call that sends
    /// [`value`](Sig::value) to the server. The element carries
    /// `novalidate`, because the browser's own checks of `required` and
    /// `type="email"` would stop the submit event before the rules run.
    #[must_use]
    pub fn form(self, then: impl IntoStmt<S>) -> El<S> {
        let rules = self.rules();
        let mut keys: Vec<&str> = rules
            .checks
            .iter()
            .map(|check| check.key.as_str())
            .collect();
        keys.sort_unstable();
        keys.dedup();
        let submit = keys
            .into_iter()
            .fold(Stmt::none(), |stmt, key| stmt.then(self.touch_key(key)))
            .then(self.at(part::<T, bool>("submitted")).set(true))
            .then(
                self.at(part::<T, Value>("errors"))
                    .set(Expr::from_json(json!(["lit", {}]))),
            )
            .then(Stmt::when(rules.all_pass(), then));
        html::form().attr("novalidate", "").on_submit(submit)
    }
}

/// The field `name` of [`Form`], whose names the struct above serializes.
fn part<T, U>(name: &'static str) -> Sig<Form<T>, U> {
    Sig::from_path(Path::field(name))
}

#[cfg(test)]
mod tests {
    use serde::{Deserialize, Serialize};

    use super::{FieldErrors, Form, Rules, Validate};
    use crate::_private::sig;
    use crate::{Sig, Signals};

    #[derive(Serialize, Deserialize, Clone)]
    struct Signup {
        email: String,
        password: String,
        confirm: String,
    }

    impl Signals for Signup {}

    fn email() -> Sig<Signup, String> {
        sig("email")
    }

    fn password() -> Sig<Signup, String> {
        sig("password")
    }

    fn confirm() -> Sig<Signup, String> {
        sig("confirm")
    }

    impl Validate for Signup {
        fn rules<S>(form: Sig<S, Self>, rules: &mut Rules<S, Self>) {
            let password_value = form.at(password());
            rules
                .check(
                    email(),
                    !form.at(email()).is_empty(),
                    "Enter an email address.",
                )
                .check(
                    email(),
                    form.at(email()).contains("@"),
                    "That is not an email address.",
                )
                .check(
                    password(),
                    password_value.char_count().ge(8),
                    "Use 8 or more characters.",
                )
                .check(
                    confirm(),
                    form.at(confirm()).equals(password_value),
                    "The passwords differ.",
                );
        }
    }

    struct Page;

    fn signup() -> Sig<Page, Form<Signup>> {
        sig("signup")
    }

    fn signup_with(email: &str, password: &str, confirm: &str) -> Signup {
        Signup {
            email: email.to_owned(),
            password: password.to_owned(),
            confirm: confirm.to_owned(),
        }
    }

    #[test]
    fn validate_reports_the_first_failing_rule_of_each_field() {
        let errors = Form::validate(&signup_with("", "short", "other")).unwrap_err();
        assert_eq!(errors.message(email()), Some("Enter an email address."));
        assert_eq!(
            errors.message(password()),
            Some("Use 8 or more characters.")
        );
        assert_eq!(errors.message(confirm()), Some("The passwords differ."));
        Form::validate(&signup_with("a@b", "longenough", "longenough")).unwrap();
    }

    #[test]
    fn touch_and_reject_write_under_the_field_key() {
        assert_eq!(
            signup().touch(email()).to_json(),
            r#"["set","signup.touched.email",["lit",true]]"#
        );
        let patch = signup().reject(&FieldErrors::new().add(email(), "Taken."));
        assert_eq!(
            format!("{patch:?}"),
            r#"SignalPatch({"signup.errors": Object {"email": String("Taken.")}})"#
        );
    }

    #[test]
    fn an_error_without_rules_reads_only_the_server_message() {
        struct Plain;
        #[derive(Serialize, Deserialize)]
        struct Note {
            text: String,
        }
        impl Signals for Note {}
        impl Validate for Note {}
        fn note() -> Sig<Plain, Form<Note>> {
            sig("note")
        }
        let text: Sig<Note, String> = sig("text");
        assert_eq!(
            note().error(text).to_json(),
            r#"["or_else",["get",["sig","note.errors"],"text"],["lit",""]]"#
        );
        assert_eq!(note().is_valid().to_json(), r#"["lit",true]"#);
    }
}
