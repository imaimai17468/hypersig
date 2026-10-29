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
    StyleBasics => "style/start/basics.rs";
    StyleValues => "style/values/properties.rs";
    WrongValue => "style/values/wrong_value.rs", "compile_fail,E0277";
    GroupPeer => "style/conditions/group_peer.rs";
    Tokens => "style/tokens/tokens.rs";
    GlobalRules => "style/global/rules.rs";
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
    Upgrade014 => "upgrading/v0_14.rs";
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
    StyleOneRule => "style/start/one_rule.rs";
    StyleMerge => "style/start/merge.rs";
    StyleMoved => "style/start/moved.rs", "compile_fail,E0382";
    StyleShorthand => "style/start/shorthand.rs";
    StylePageStyle => "style/start/page_style.rs";
    StyleButtons => "style/start/buttons.rs";
    StyleMethodTypo => "style/values/method_typo.rs", "compile_fail,E0599";
    StyleNumbers => "style/values/numbers.rs";
    StyleNumberNan => "style/values/number_nan.rs", "should_panic";
    StyleShorthands => "style/values/shorthands.rs";
    StyleRawValues => "style/values/raw_values.rs";
    StyleNotice => "style/values/notice.rs";
    StyleLengths => "style/values/lengths.rs";
    StyleLengthBare => "style/values/length_bare.rs", "compile_fail,E0277";
    StyleLineHeightInteger => "style/values/line_height_integer.rs", "compile_fail,E0277";
    StyleBorderPct => "style/values/border_pct.rs", "compile_fail,E0277";
    StyleLengthF32 => "style/values/length_f32.rs";
    StyleLengthMath => "style/values/length_math.rs";
    StyleLengthMathMixed => "style/values/length_math_mixed.rs", "compile_fail,E0308";
    StyleArticle => "style/values/article.rs";
    StyleColors => "style/values/colors.rs";
    StyleColorString => "style/values/color_string.rs", "compile_fail,E0277";
    StyleColorLimits => "style/values/color_limits.rs";
    StyleBadge => "style/values/badge.rs";
    StyleAnglesTimes => "style/values/angles_times.rs";
    StyleTimeBare => "style/values/time_bare.rs", "compile_fail,E0277";
    StyleAngleTimeSwap => "style/values/angle_time_swap.rs", "compile_fail,E0277";
    StyleChevron => "style/values/chevron.rs";
    StyleUrls => "style/values/urls.rs";
    StyleUrlScript => "style/values/url_script.rs", "compile_fail,E0080";
    StyleUrlString => "style/values/url_string.rs", "compile_fail,E0308";
    StyleUrlBase => "style/values/url_base.rs";
    StyleLinksList => "style/values/links_list.rs";
    StyleKeywords => "style/values/keywords.rs";
    StyleKeywordMethod => "style/values/keyword_method.rs", "compile_fail,E0277";
    StyleTextButton => "style/values/text_button.rs";
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
    Stacks => "style/layout/stacks.rs";
    StackResponsive => "style/layout/stack_responsive.rs";
    StackGap => "style/layout/stack_gap.rs", "compile_fail,E0277";
    Grids => "style/layout/grids.rs";
    GridColumns => "style/layout/grid_columns.rs", "compile_fail,E0308";
    GridSignal => "style/layout/grid_signal.rs";
    Patterns => "style/layout/patterns.rs";
    CircleSize => "style/layout/circle_size.rs", "compile_fail,E0308";
    Keyframes => "style/animation/keyframes.rs";
    KeyframesName => "style/animation/keyframes_name.rs", "compile_fail,E0277";
    KeyframesSteps => "style/animation/keyframes_steps.rs";
    Transitions => "style/animation/transitions.rs";
    TransitionDuration => "style/animation/transition_duration.rs", "compile_fail,E0277";
    PageTransitions => "style/animation/page_transitions.rs";
    PageTransitionStmt => "style/animation/page_transition_stmt.rs";
    ViewTransitionName => "style/animation/view_transition_name.rs", "compile_fail,E0277";
    ReducedMotion => "style/animation/reduced_motion.rs";
    ReducedMotionVariant => "style/animation/reduced_motion_variant.rs", "compile_fail,E0599";
    Raw => "style/unchecked/raw.rs";
    RawPanic => "style/unchecked/raw_panic.rs", "should_panic";
    Unchecked => "style/unchecked/stylesheet.rs";
    StylesheetClass => "style/unchecked/stylesheet_class.rs", "compile_fail,E0080";
    LinkedStylesheet => "style/unchecked/linked_stylesheet.rs";
    StyleAttr => "style/unchecked/style_attr.rs", "compile_fail,E0080";
    // The samples of the Components pages under Get started, Buttons, Form fields, and Pickers.
    UiSetup => "components/start/setup.rs";
    UiPreview => "components/start/preview.rs";
    UiScheme => "components/start/scheme.rs";
    UiTokens => "components/start/tokens.rs";
    UiHandles => "components/start/handles.rs";
    UiTexts => "components/start/texts.rs";
    UiButton => "components/buttons/button.rs";
    UiIconButton => "components/buttons/icon_button.rs";
    UiLinkButton => "components/buttons/link_button.rs";
    UiToggle => "components/buttons/toggle.rs";
    UiToggleGroup => "components/buttons/toggle_group.rs";
    UiMultiToggleGroup => "components/buttons/multi_toggle_group.rs";
    UiTextField => "components/forms/text_field.rs";
    UiNumberField => "components/forms/number_field.rs";
    UiNumberBounds => "components/forms/number_bounds.rs", "should_panic";
    UiDateField => "components/forms/date_field.rs";
    UiCheckbox => "components/forms/checkbox.rs";
    UiSwitch => "components/forms/switch.rs";
    UiRadioGroup => "components/forms/radio_group.rs";
    UiSelect => "components/forms/select.rs";
    UiSlider => "components/forms/slider.rs";
    UiFormFields => "components/forms/form_fields.rs";
    UiListbox => "components/pickers/listbox.rs";
    UiCombobox => "components/pickers/combobox.rs";
    UiGlobalAsTheme =>"components/start/global_as_theme.rs", "compile_fail,E0308";
    UiThemeAsStyle => "components/start/theme_as_style.rs", "compile_fail,E0308";
    UiTokenWrongValue => "components/start/token_wrong_value.rs", "compile_fail,E0277";
    UiComboboxSameRef => "components/start/combobox_same_ref.rs", "compile_fail,E0080";
    UiHandleTwice => "components/start/handle_twice.rs", "should_panic";
    UiTextMissingField => "components/start/text_missing_field.rs", "compile_fail,E0063";
    UiVariantString => "components/buttons/variant_string.rs", "compile_fail,E0308";
    UiIconWithoutLabel => "components/buttons/icon_without_label.rs", "compile_fail,E0061";
    UiLinkHrefString => "components/buttons/link_href_string.rs", "compile_fail,E0277";
    UiToggleComputed => "components/buttons/toggle_computed.rs", "compile_fail,E0277";
    UiGroupWrongValue => "components/buttons/group_wrong_value.rs", "compile_fail,E0308";
    UiMultiWrongSignal => "components/buttons/multi_wrong_signal.rs", "compile_fail,E0308";
    UiTextFieldNumber => "components/forms/text_field_number.rs", "compile_fail,E0308";
    UiNumberFieldText => "components/forms/number_field_text.rs", "compile_fail,E0277";
    UiDateFieldText => "components/forms/date_field_text.rs", "compile_fail,E0277";
    UiCheckboxComputed => "components/forms/checkbox_computed.rs", "compile_fail,E0277";
    UiSwitchText => "components/forms/switch_text.rs", "compile_fail,E0308";
    UiRadioWrongOption => "components/forms/radio_wrong_option.rs", "compile_fail,E0271";
    UiSelectOtherEnum => "components/forms/select_other_enum.rs", "compile_fail,E0271";
    UiSliderFloat => "components/forms/slider_float.rs", "compile_fail,E0308";
    UiInFormOtherStruct => "components/forms/in_form_other_struct.rs", "compile_fail,E0308";
    UiListboxNoSegment => "components/pickers/listbox_no_segment.rs", "compile_fail,E0277";
    UiComboboxStringState => "components/pickers/combobox_string_state.rs", "compile_fail,E0308";
    // The samples of the Components pages under Layout, Navigation, and Data.
    // The samples of the Components pages under Overlays and Display.
    UiDialog => "components/overlays/dialog.rs";
    UiDialogNodeRef => "components/overlays/dialog_node_ref.rs", "compile_fail,E0308";
    UiDialogText => "components/overlays/dialog_text.rs", "compile_fail,E0061";
    UiAlertDialog => "components/overlays/alert_dialog.rs";
    UiAlertDialogText => "components/overlays/alert_dialog_text.rs", "compile_fail,E0061";
    UiSheet => "components/overlays/sheet.rs";
    UiSheetSide => "components/overlays/sheet_side.rs", "compile_fail,E0308";
    UiMenu => "components/overlays/menu.rs";
    UiMenuDisabled => "components/overlays/menu_disabled.rs", "compile_fail,E0277";
    UiTooltip => "components/overlays/tooltip.rs";
    UiTooltipLabel => "components/overlays/tooltip_label.rs", "compile_fail,E0061";
    UiTooltipRich => "components/overlays/tooltip_rich.rs", "compile_fail,E0277";
    UiPopover => "components/overlays/popover.rs";
    UiPopoverDialogRef => "components/overlays/popover_dialog_ref.rs", "compile_fail,E0308";
    UiToast => "components/overlays/toast.rs";
    UiToastList => "components/overlays/toast_list.rs", "compile_fail,E0308";
    UiToastDelay => "components/overlays/toast_delay.rs", "compile_fail,E0308";
    UiToastId => "components/overlays/toast_id.rs", "compile_fail,E0639";
    UiServerToasts => "components/overlays/server_toasts.rs";
    UiServerToastText => "components/overlays/server_toast_text.rs", "compile_fail,E0308";
    UiAlert => "components/display/alert.rs";
    UiAlertVariant => "components/display/alert_variant.rs", "compile_fail,E0061";
    UiBadge => "components/display/badge.rs";
    UiBadgeVariant => "components/display/badge_variant.rs", "compile_fail,E0308";
    UiProgress => "components/display/progress.rs";
    UiProgressFraction => "components/display/progress_fraction.rs", "compile_fail,E0277";
    UiSkeleton => "components/display/skeleton.rs";
    UiSpinner => "components/display/spinner.rs";
    UiSpinnerLabel => "components/display/spinner_label.rs", "compile_fail,E0061";
    UiAvatar => "components/display/avatar.rs";
    UiAvatarUrl => "components/display/avatar_url.rs", "compile_fail,E0277";
    UiKbd => "components/display/kbd.rs";
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
