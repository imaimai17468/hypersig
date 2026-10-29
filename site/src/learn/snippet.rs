//! Short code samples, each a file under `site/snippets/` that `cargo test`
//! runs as a doctest, so a sample that stops compiling fails the tests.
//!
//! A line that starts with `# ` is compiled and left out of the page, as in
//! rustdoc. A sample shown as a change is the code after it: a line that ends
//! in `//+` was added, and a comment line that starts with `//-` holds a line
//! the change removed, so the doctest compiles what the change leaves.

use crate::code::{self, Change, Lang};
use crate::prose::Part;

macro_rules! snippets {
    ($($name:ident => $file:literal $(, $attr:literal)?;)*) => {
        /// One code sample.
        #[derive(Debug, Clone, Copy)]
        pub enum Snippet {
            $($name,)*
        }

        impl Snippet {
            fn source(self) -> &'static str {
                match self {
                    $(Self::$name => include_str!(concat!("../../snippets/", $file)),)*
                }
            }
        }

        #[cfg(doctest)]
        mod doctests {
            $(
                #[doc = concat!("```", $($attr,)? "\n", include_str!(concat!("../../snippets/", $file)), "```")]
                struct $name;
            )*
        }
    };
}

snippets! {
    Elements => "page/elements.rs";
    Foreign => "browser/foreign.rs";
    ElementsModule => "elements_module.rs";
    ElementsPage => "browser/elements_page.rs";
    Component => "page/component.rs";
    Bindings => "page/bindings.rs";
    SignalPaths => "state/signal_paths.rs";
    WrongField => "state/wrong_field.rs", "compile_fail,E0599";
    Nested => "state/nested.rs";
    Persist => "state/persist.rs";
    ComponentState => "state/component_state.rs";
    Operators => "state/operators.rs";
    WrongType => "wrong_type.rs", "compile_fail,E0277";
    WhyCounter => "why_counter.rs";
    Statements => "state/statements.rs";
    Events => "state/events.rs";
    Each => "state/each.rs";
    ListStatements => "state/list_statements.rs";
    Patches => "patches.rs";
    ListPatches => "list_patches.rs";
    TestingListPatches => "quality/testing_list_patches.rs";
    TestingSimulation => "quality/testing_simulation.rs";
    Forms => "requests/form_page.rs";
    Latest => "latest.rs";
    Parts => "page/parts.rs";
    Extractors => "requests/extractors.rs";
    HandlerErrors => "requests/handler_errors.rs";
    Logging => "requests/logging.rs";
    Uploads => "requests/uploads.rs";
    Sessions => "data/sessions.rs";
    TestingSessions => "quality/testing_sessions.rs";
    Database => "data/database.rs", "no_run";
    DynamoDb => "data/dynamodb.rs", "no_run";
    Api => "data/api.rs", "no_run";
    Links => "routing/links.rs";
    WrongRoute => "routing/wrong_route.rs", "compile_fail,E0080";
    ChildRoutes => "routing/child_routes.rs";
    Navigation => "routing/navigation.rs";
    History => "routing/history.rs";
    Head => "page/head.rs";
    SearchParams => "routing/search_params.rs";
    Styles => "page/styles.rs";
    Effects => "browser/effects.rs";
    StaticPage => "browser/static_page.rs";
    UiSetup => "ui_setup.rs";
    UiTheme => "ui_theme.rs";
    UiScheme => "ui_scheme.rs";
    ToastServer => "toast_server.rs";
    StyleBasics => "style_basics.rs";
    StyleValues => "style_values.rs";
    WrongValue => "wrong_value.rs", "compile_fail,E0277";
    GroupPeer => "style/conditions/group_peer.rs";
    Tokens => "style/tokens/tokens.rs";
    GlobalRules => "style/global/rules.rs";
    Patterns => "patterns.rs";
    Animation => "animation.rs";
    ViewTransitions => "view_transitions.rs";
    ViewTransitionStmt => "view_transition_stmt.rs";
    Unchecked => "unchecked.rs";
    I18nLocale => "i18n_locale.rs";
    I18nMissing => "i18n_missing.rs", "compile_fail,E0063";
    I18nPages => "i18n_pages.rs";
    I18nNegotiate => "i18n_negotiate.rs";
    // The changes the use cases page shows, each written as the code after
    // the change, so the doctest compiles what the page adds.
    UseOthersChanges => "use_cases/others_changes.rs";
    UseServerError => "use_cases/server_error.rs";
    UseLatest => "use_cases/latest.rs";
    UseAuction => "use_cases/auction.rs";
    UseDeployLog => "use_cases/deploy_log.rs";
    UseKitchen => "use_cases/kitchen.rs";
    UseSeats => "use_cases/seats.rs";
    UseUrlFilter => "use_cases/url_filter.rs";
    UseTranslations => "use_cases/translations.rs";
    // The code `cargo hypersig add server` prints, so the page and the CLI
    // show one text.
    Listen => "../../crates/hypersig_cli/steps/server.rs";
    SafeUrl => "browser/safe_url.rs";
    StaticFiles => "browser/static_files.rs";
    ErrorPage => "routing/error_page.rs";
    LoadMore => "load_more.rs";
    EditState => "state/edit_state.rs";
    // The upgrading page shows each release's breaking changes as the code
    // after them, so the doctest compiles what the release accepts.
    Upgrade05 => "upgrading/v0_5.rs";
    Upgrade06 => "upgrading/v0_6.rs";
    Upgrade07 => "upgrading/v0_7.rs";
    Upgrade08 => "upgrading/v0_8.rs";
    Upgrade09 => "upgrading/v0_9.rs";
    Upgrade010 => "upgrading/v0_10.rs";
    Upgrade011 => "upgrading/v0_11.rs";
    Upgrade012 => "upgrading/v0_12.rs";
    Upgrade013 => "upgrading/v0_13.rs";
    ErrorHandler => "quality/error_handler.rs", "compile_fail,E0277";
    ErrorId => "quality/error_id.rs", "compile_fail,E0080";
    ErrorComputed => "quality/error_computed.rs", "compile_fail,E0277";
    CallError => "quality/call_error.rs";
    // The samples of the Learn pages under Get started.
    KeptSignals => "start/kept_signals.rs";
    // The samples of the Learn pages under Building the page.
    OtherPage => "page/other_page.rs", "compile_fail,E0277";
    Children => "page/children.rs";
    Attrs => "page/attrs.rs";
    AttrScript => "page/attr_script.rs", "compile_fail,E0080";
    Ids => "page/ids.rs";
    Team => "page/team.rs";
    Document => "page/document.rs";
    Serve => "page/serve.rs";
    Title => "page/title.rs";
    TitleType => "page/title_type.rs", "compile_fail,E0277";
    OriginPath => "page/origin_path.rs", "compile_fail,E0080";
    Search => "page/search.rs";
    PostPage => "page/post_page.rs";
    BindingsRender => "page/bindings_render.rs";
    When => "page/when.rs";
    Classes => "page/classes.rs";
    ShowType => "page/show_type.rs", "compile_fail,E0277";
    PartOutside => "page/part_outside.rs", "compile_fail,E0277";
    PartInList => "page/part_in_list.rs", "compile_fail,E0308";
    StyleKeyword => "page/style_keyword.rs", "compile_fail,E0599";
    BindStyle => "page/bind_style.rs";
    // The samples of the Learn pages under Browser state.
    StateFirstRender => "state/first_render.rs";
    StateOtherPage => "state/other_page.rs", "compile_fail,E0277";
    StateSerdeAttribute => "state/serde_attribute.rs", "compile_fail";
    StateNestedWrongStruct => "state/nested_wrong_struct.rs", "compile_fail,E0308";
    StateStructValues => "state/struct_values.rs";
    StateOptions => "state/options.rs";
    StateIntegerDivision => "state/integer_division.rs", "compile_fail,E0369";
    StateStrings => "state/strings.rs";
    StateNumberFormat => "state/number_format.rs";
    StateLetIn => "state/let_in.rs";
    StateThenOrder => "state/then_order.rs";
    StateEnumByValue => "state/enum_by_value.rs", "compile_fail,E0277";
    StateLetInStmt => "state/let_in_stmt.rs";
    StateEventsClick => "state/events_click.rs";
    StateEventsText => "state/events_text.rs";
    StateEventsSubmit => "state/events_submit.rs";
    StateEventsFocus => "state/events_focus.rs";
    StateEventsPointer => "state/events_pointer.rs";
    StateEventsLoad => "state/events_load.rs";
    StateEventsDebounce => "state/events_debounce.rs";
    StateEventsTwice => "state/events_twice.rs";
    StateEventsKeys => "state/events_keys.rs";
    StateEventsPageKey => "state/events_page_key.rs";
    StateEventsWrongPage => "state/events_wrong_page.rs", "compile_fail,E0277";
    StateInputsBind => "state/inputs_bind.rs";
    StateInputsRadio => "state/inputs_radio.rs";
    StateInputsSelect => "state/inputs_select.rs";
    StateInputsBindEnum => "state/inputs_bind_enum.rs", "compile_fail,E0277";
    StateInputsExample => "state/inputs_example.rs";
    StateComputed => "state/computed.rs";
    StateComputedOrder => "state/computed_order.rs";
    StateComputedSet => "state/computed_set.rs", "compile_fail,E0599";
    StateComputedNotMarked => "state/computed_not_marked.rs", "compile_fail,E0277";
    StateComputedRead => "state/computed_read.rs";
    StateComputedNested => "state/computed_nested.rs";
    StatePersistComputed => "state/persist_computed.rs", "compile_fail,E0308";
    StateListRender => "state/list_render.rs";
    StateListExpressions => "state/list_expressions.rs";
    StatePushWrongType => "state/push_wrong_type.rs", "compile_fail,E0277";
    StateKeyOutside => "state/key_outside.rs", "compile_fail,E0599";
    StateItemFields => "state/item_fields.rs";
    StatePatchOnItem => "state/patch_on_item.rs", "compile_fail,E0599";
    StateItemBranch => "state/item_branch.rs";
    StateNestedLists => "state/nested_lists.rs";
    // The samples of the Learn pages under Talking to the server.
    TalkDeclare => "talking/declare.rs";
    TalkCall => "talking/call.rs";
    TalkWrongArgs => "talking/wrong_args.rs", "compile_fail,E0277";
    TalkRequest => "talking/request.rs";
    TalkReply => "talking/reply.rs";
    TalkWrongReply => "talking/wrong_reply.rs", "compile_fail,E0277";
    TalkCallStatus => "talking/call_status.rs";
    TalkWrongIndicator => "talking/wrong_indicator.rs", "compile_fail,E0308";
    TalkOutcomes => "talking/outcomes.rs";
    TalkOptimistic => "talking/optimistic.rs";
    TalkWrongRollback => "talking/wrong_rollback.rs", "compile_fail,E0277";
    TalkExclusive => "talking/exclusive.rs";
    TalkStreamTimeout => "talking/stream_timeout.rs", "compile_fail,E0599";
    TalkPatchPaths => "talking/patch_paths.rs";
    TalkWrongPatch => "talking/wrong_patch.rs", "compile_fail,E0308";
    TalkItemPatch => "talking/item_patch.rs", "compile_fail,E0599";
    TalkFragment => "talking/fragment.rs";
    TalkWrongFragment => "talking/wrong_fragment.rs", "compile_fail,E0271";
    TalkSince => "talking/since.rs";
    TalkStreamAsync => "talking/stream_async.rs", "compile_fail,E0277";
    TalkReconnectOnAction => "talking/reconnect_on_action.rs", "compile_fail,E0599";
    TalkInbox => "talking/inbox.rs";
    // The samples of the Learn pages under Pages and routing.
    RouteSegments => "routing/segments.rs";
    WrongSegment => "routing/wrong_segment.rs", "compile_fail,E0277";
    WrongHref => "routing/wrong_href.rs", "compile_fail,E0277";
    WrongChild => "routing/wrong_child.rs", "compile_fail,E0277";
    OrgPages => "routing/org_pages.rs";
    SearchFields => "routing/search_fields.rs";
    WrongSearch => "routing/wrong_search.rs", "compile_fail,E0277";
    MissingPage => "routing/missing_page.rs";
    WrongUnserved => "routing/wrong_unserved.rs", "compile_fail,E0277";
    BasePath => "routing/base_path.rs";
    WrongBase => "routing/wrong_base.rs", "compile_fail,E0080";
    // The samples of the Learn pages under Forms and requests.
    FormWrongField => "requests/form_wrong_field.rs", "compile_fail,E0308";
    FormComponent => "requests/form_component.rs";
    FormRules => "requests/form_rules.rs";
    FormRuleType => "requests/form_rule_type.rs", "compile_fail,E0277";
    FormHandler => "requests/form_handler.rs";
    Headers => "requests/headers.rs";
    ExtractorOrder => "requests/extractor_order.rs", "compile_fail,E0277";
    ErrorNotResponse => "requests/error_not_response.rs", "compile_fail,E0277";
    UploadWrongPage => "requests/upload_wrong_page.rs", "compile_fail,E0308";
    UploadStream => "requests/upload_stream.rs";
    // The samples of the Learn pages under Data and sign-in.
    WrongState => "data/wrong_state.rs", "compile_fail,E0277";
    ErrorWithoutResponse => "data/error_without_response.rs", "compile_fail,E0277";
    SqlxPool => "data/sqlx_pool.rs", "no_run";
    PgListen => "data/listen.rs", "no_run";
    PostgresBuild => "../../examples/postgres/build.rs";
    ApiRefusal => "data/api_refusal.rs", "no_run";
    WrongErrorSignal => "data/wrong_error_signal.rs", "compile_fail,E0308";
    Members => "data/members.rs";
    NavigateText => "data/navigate_text.rs", "compile_fail,E0308";
    RedirectText => "data/redirect_text.rs", "compile_fail,E0308";
    SessionExpiry => "data/session_expiry.rs";
    // The samples of the Learn pages under Browser features.
    NodeRefDeclare => "browser/node_ref.rs";
    NodeRefInFn => "browser/node_ref_in_fn.rs", "compile_fail,E0425";
    NodeRefOtherPage => "browser/node_ref_other_page.rs", "compile_fail,E0308";
    NodeRefList => "browser/node_ref_list.rs";
    NodeRefTwice => "browser/node_ref_twice.rs", "should_panic";
    DialogRefs => "browser/dialog_ref.rs";
    Timers => "browser/timers.rs";
    IntervalMillis => "browser/interval_millis.rs", "compile_fail,E0308";
    EffectWatch => "browser/effect.rs";
    Clock => "browser/clock.rs";
    RovingFocus => "browser/roving_focus.rs";
    ActiveDescendant => "browser/active_descendant.rs";
    ActiveDescendantType => "browser/active_descendant_type.rs", "compile_fail,E0308";
    SearchKeys => "browser/search_keys.rs";
    CustomEventPage => "browser/custom_event.rs";
    CustomEventType => "browser/custom_event_type.rs", "compile_fail,E0277";
    HrefString => "browser/href_string.rs", "compile_fail,E0277";
    SafeUrlScript => "browser/safe_url_script.rs", "compile_fail,E0080";
    AttrHref => "browser/attr_href.rs", "compile_fail,E0080";
    AttrValue => "browser/attr_value.rs";
    RenderUnder => "browser/render_under.rs";
    RuntimeSrcText => "browser/runtime_src_text.rs", "compile_fail,E0308";
    // The samples of the Learn pages under Quality and operations.
    Escaping => "quality/escaping.rs";
    AttrEventHandler => "quality/attr_event_handler.rs", "compile_fail,E0080";
    HrefFromString => "quality/href_from_string.rs", "compile_fail,E0277";
    Csp => "quality/csp.rs";
    TestingStream => "quality/testing_stream.rs";
    TestingWrongPage => "quality/testing_wrong_page.rs", "compile_fail,E0308";
    TestingListItems => "quality/testing_list_items.rs";
    SimulationWrongPage => "quality/simulation_wrong_page.rs", "compile_fail,E0277";
    DuplicateId => "quality/duplicate_id.rs", "should_panic";
    CallStatusType => "quality/call_status_type.rs", "compile_fail,E0308";
    // The samples of the Style pages under Get started and Values.
    // The samples of the Style pages under Conditions, Tokens and themes, and Global styles.
    PseudoClasses => "style/conditions/pseudo_classes.rs";
    PseudoClassTypo => "style/conditions/pseudo_class_typo.rs", "compile_fail,E0599";
    NestedConditions => "style/conditions/nested.rs";
    PseudoElements => "style/conditions/pseudo_elements.rs";
    PseudoElementCondition => "style/conditions/pseudo_element_condition.rs", "compile_fail,E0277";
    PseudoElementNested => "style/conditions/pseudo_element_nested.rs", "should_panic";
    MediaQueries => "style/conditions/media.rs";
    MediaOrder => "style/conditions/media_order.rs";
    MediaTypo => "style/conditions/media_typo.rs", "compile_fail,E0599";
    ContainerQueries => "style/conditions/containers.rs";
    ContainerWidth => "style/conditions/container_width.rs", "compile_fail,E0599";
    GroupCondition => "style/conditions/group_condition.rs", "compile_fail,E0308";
    CssWhen => "style/conditions/css_when.rs";
    CssMatch => "style/conditions/css_match.rs";
    ServerMatch => "style/conditions/server_match.rs";
    CssWhenType => "style/conditions/css_when_type.rs", "compile_fail,E0277";
    TokenType => "style/tokens/token_type.rs", "compile_fail,E0277";
    TokenValue => "style/tokens/token_value.rs", "compile_fail,E0277";
    TokenName => "style/tokens/token_name.rs", "should_panic";
    TokenScope => "style/tokens/token_scope.rs";
    BindToken => "style/tokens/bind_token.rs";
    Themes => "style/tokens/theme.rs";
    ThemeCondition => "style/tokens/theme_condition.rs", "compile_fail,E0308";
    DarkSystem => "style/tokens/dark_system.rs";
    DarkPicker => "style/tokens/dark_picker.rs";
    SelectorString => "style/global/selector_string.rs", "compile_fail,E0277";
    RuleGroup => "style/global/rule_group.rs", "should_panic";
    Fonts => "style/global/fonts.rs";
    FontSrc => "style/global/font_src.rs", "compile_fail,E0308";
    FontUrl => "style/global/font_url.rs", "compile_fail,E0080";
    // The samples of the Style pages under Layout, Animation, and Opting out of type checks.
}

/// The lines of `snippet` that the page shows.
fn shown(snippet: Snippet) -> impl Iterator<Item = &'static str> {
    snippet
        .source()
        .lines()
        .filter(|line| !(line.starts_with("# ") || *line == "#"))
}

/// `snippet` as a code block, without its hidden lines.
pub fn show(snippet: Snippet) -> Part {
    let shown: String = shown(snippet).flat_map(|line| [line, "\n"]).collect();
    code::block(Lang::Rust, None, &shown)
}

/// `snippet` as a code block headed by the file name `file`.
pub fn show_file(snippet: Snippet, file: &str) -> Part {
    let shown: String = shown(snippet).flat_map(|line| [line, "\n"]).collect();
    code::block(Lang::Rust, Some(file), &shown)
}

/// `snippet` as a change, with its added and removed lines marked.
pub fn show_change(snippet: Snippet) -> Part {
    code::diff(
        Lang::Rust,
        shown(snippet).map(|line| {
            if let Some(removed) = line.strip_prefix("//-") {
                (Change::Removed, removed)
            } else if let Some(added) = line.strip_suffix("//+") {
                (Change::Added, added.trim_end())
            } else {
                (Change::Kept, line)
            }
        }),
    )
}
