use hypersig_ui::{Toast, ToastVariant};

// A toast's id is set by `Toast::new` alone.
let saved = Toast {
    title: "Saved".into(),
    body: String::new(),
    tone: ToastVariant::Default,
};
