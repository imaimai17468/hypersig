//! The attributes of HTML, SVG, and ARIA that an [`Attr`] can name.
//!
//! The names come from the npm packages `html-element-attributes` 3.5.0,
//! `svg-element-attributes` 2.2.0, and `aria-attributes` 2.0.1, which
//! `tests/generate.rs` reads from `data/`. The attributes [`Attr`] refuses
//! have no constant here. A name outside these lists, such as `data-lat`
//! or an attribute of a custom element, is built with
//! [`attr!`](crate::attr!).
//!
//! Written by `tests/generate.rs`. Run it with
//! `HYPERSIG_GENERATE=overwrite` rather than editing this file.

use super::Attr;

/// `abbr`, on `<td>`, `<th>`.
pub const ABBR: Attr = Attr::new("abbr");

/// `about`, on every SVG element.
pub const ABOUT: Attr = Attr::new("about");

/// `accent-height`, on `<font-face>`.
pub const ACCENT_HEIGHT: Attr = Attr::new("accent-height");

/// `accept`, on `<form>`, `<input>`.
pub const ACCEPT: Attr = Attr::new("accept");

/// `accept-charset`, on `<form>`.
pub const ACCEPT_CHARSET: Attr = Attr::new("accept-charset");

/// `accesskey`, on every HTML element.
pub const ACCESSKEY: Attr = Attr::new("accesskey");

/// `accumulate`, on `<animate>`, `<animateColor>`, `<animateMotion>`, `<animateTransform>`.
pub const ACCUMULATE: Attr = Attr::new("accumulate");

/// `additive`, on `<animate>`, `<animateColor>`, `<animateMotion>`, `<animateTransform>`.
pub const ADDITIVE: Attr = Attr::new("additive");

/// `align`, on 25 HTML elements.
pub const ALIGN: Attr = Attr::new("align");

/// `alignment-baseline`, on every SVG element.
pub const ALIGNMENT_BASELINE: Attr = Attr::new("alignment-baseline");

/// `alink`, on `<body>`.
pub const ALINK: Attr = Attr::new("alink");

/// `allow`, on `<iframe>`.
pub const ALLOW: Attr = Attr::new("allow");

/// `allowfullscreen`, on `<iframe>`.
pub const ALLOWFULLSCREEN: Attr = Attr::new("allowfullscreen");

/// `allowpaymentrequest`, on `<iframe>`.
pub const ALLOWPAYMENTREQUEST: Attr = Attr::new("allowpaymentrequest");

/// `allowusermedia`, on `<iframe>`.
pub const ALLOWUSERMEDIA: Attr = Attr::new("allowusermedia");

/// `alpha`, on `<input>`.
pub const ALPHA: Attr = Attr::new("alpha");

/// `alphabetic`, on `<font-face>`.
pub const ALPHABETIC: Attr = Attr::new("alphabetic");

/// `alt`, on `<applet>`, `<area>`, `<img>`, `<input>`.
pub const ALT: Attr = Attr::new("alt");

/// `amplitude`, on `<feFuncA>`, `<feFuncB>`, `<feFuncG>`, `<feFuncR>`.
pub const AMPLITUDE: Attr = Attr::new("amplitude");

/// `arabic-form`, on `<glyph>`.
pub const ARABIC_FORM: Attr = Attr::new("arabic-form");

/// `archive`, on `<applet>`, `<object>`.
pub const ARCHIVE: Attr = Attr::new("archive");

/// `aria-atomic`, on every element, as ARIA defines.
pub const ARIA_ATOMIC: Attr = Attr::new("aria-atomic");

/// `aria-autocomplete`, on every element, as ARIA defines.
pub const ARIA_AUTOCOMPLETE: Attr = Attr::new("aria-autocomplete");

/// `aria-busy`, on every element, as ARIA defines.
pub const ARIA_BUSY: Attr = Attr::new("aria-busy");

/// `aria-checked`, on every element, as ARIA defines.
pub const ARIA_CHECKED: Attr = Attr::new("aria-checked");

/// `aria-colcount`, on every element, as ARIA defines.
pub const ARIA_COLCOUNT: Attr = Attr::new("aria-colcount");

/// `aria-colindex`, on every element, as ARIA defines.
pub const ARIA_COLINDEX: Attr = Attr::new("aria-colindex");

/// `aria-colspan`, on every element, as ARIA defines.
pub const ARIA_COLSPAN: Attr = Attr::new("aria-colspan");

/// `aria-current`, on every element, as ARIA defines.
pub const ARIA_CURRENT: Attr = Attr::new("aria-current");

/// `aria-disabled`, on every element, as ARIA defines.
pub const ARIA_DISABLED: Attr = Attr::new("aria-disabled");

/// `aria-dropeffect`, on every element, as ARIA defines.
pub const ARIA_DROPEFFECT: Attr = Attr::new("aria-dropeffect");

/// `aria-expanded`, on every element, as ARIA defines.
pub const ARIA_EXPANDED: Attr = Attr::new("aria-expanded");

/// `aria-grabbed`, on every element, as ARIA defines.
pub const ARIA_GRABBED: Attr = Attr::new("aria-grabbed");

/// `aria-haspopup`, on every element, as ARIA defines.
pub const ARIA_HASPOPUP: Attr = Attr::new("aria-haspopup");

/// `aria-hidden`, on every element, as ARIA defines.
pub const ARIA_HIDDEN: Attr = Attr::new("aria-hidden");

/// `aria-invalid`, on every element, as ARIA defines.
pub const ARIA_INVALID: Attr = Attr::new("aria-invalid");

/// `aria-keyshortcuts`, on every element, as ARIA defines.
pub const ARIA_KEYSHORTCUTS: Attr = Attr::new("aria-keyshortcuts");

/// `aria-label`, on every element, as ARIA defines.
pub const ARIA_LABEL: Attr = Attr::new("aria-label");

/// `aria-level`, on every element, as ARIA defines.
pub const ARIA_LEVEL: Attr = Attr::new("aria-level");

/// `aria-live`, on every element, as ARIA defines.
pub const ARIA_LIVE: Attr = Attr::new("aria-live");

/// `aria-modal`, on every element, as ARIA defines.
pub const ARIA_MODAL: Attr = Attr::new("aria-modal");

/// `aria-multiline`, on every element, as ARIA defines.
pub const ARIA_MULTILINE: Attr = Attr::new("aria-multiline");

/// `aria-multiselectable`, on every element, as ARIA defines.
pub const ARIA_MULTISELECTABLE: Attr = Attr::new("aria-multiselectable");

/// `aria-orientation`, on every element, as ARIA defines.
pub const ARIA_ORIENTATION: Attr = Attr::new("aria-orientation");

/// `aria-placeholder`, on every element, as ARIA defines.
pub const ARIA_PLACEHOLDER: Attr = Attr::new("aria-placeholder");

/// `aria-posinset`, on every element, as ARIA defines.
pub const ARIA_POSINSET: Attr = Attr::new("aria-posinset");

/// `aria-pressed`, on every element, as ARIA defines.
pub const ARIA_PRESSED: Attr = Attr::new("aria-pressed");

/// `aria-readonly`, on every element, as ARIA defines.
pub const ARIA_READONLY: Attr = Attr::new("aria-readonly");

/// `aria-relevant`, on every element, as ARIA defines.
pub const ARIA_RELEVANT: Attr = Attr::new("aria-relevant");

/// `aria-required`, on every element, as ARIA defines.
pub const ARIA_REQUIRED: Attr = Attr::new("aria-required");

/// `aria-roledescription`, on every element, as ARIA defines.
pub const ARIA_ROLEDESCRIPTION: Attr = Attr::new("aria-roledescription");

/// `aria-rowcount`, on every element, as ARIA defines.
pub const ARIA_ROWCOUNT: Attr = Attr::new("aria-rowcount");

/// `aria-rowindex`, on every element, as ARIA defines.
pub const ARIA_ROWINDEX: Attr = Attr::new("aria-rowindex");

/// `aria-rowspan`, on every element, as ARIA defines.
pub const ARIA_ROWSPAN: Attr = Attr::new("aria-rowspan");

/// `aria-selected`, on every element, as ARIA defines.
pub const ARIA_SELECTED: Attr = Attr::new("aria-selected");

/// `aria-setsize`, on every element, as ARIA defines.
pub const ARIA_SETSIZE: Attr = Attr::new("aria-setsize");

/// `aria-sort`, on every element, as ARIA defines.
pub const ARIA_SORT: Attr = Attr::new("aria-sort");

/// `aria-valuemax`, on every element, as ARIA defines.
pub const ARIA_VALUEMAX: Attr = Attr::new("aria-valuemax");

/// `aria-valuemin`, on every element, as ARIA defines.
pub const ARIA_VALUEMIN: Attr = Attr::new("aria-valuemin");

/// `aria-valuenow`, on every element, as ARIA defines.
pub const ARIA_VALUENOW: Attr = Attr::new("aria-valuenow");

/// `aria-valuetext`, on every element, as ARIA defines.
pub const ARIA_VALUETEXT: Attr = Attr::new("aria-valuetext");

/// `as`, on `<link>`.
pub const AS: Attr = Attr::new("as");

/// `ascent`, on `<font-face>`.
pub const ASCENT: Attr = Attr::new("ascent");

/// `async`, on `<script>`.
pub const ASYNC: Attr = Attr::new("async");

/// `attributeType`, on `<animate>`, `<animateColor>`, `<animateTransform>`, `<set>`.
pub const ATTRIBUTE_TYPE: Attr = Attr::new("attributeType");

/// `autocapitalize`, on every HTML element.
pub const AUTOCAPITALIZE: Attr = Attr::new("autocapitalize");

/// `autocomplete`, on `<form>`, `<input>`, `<select>`, `<textarea>`.
pub const AUTOCOMPLETE: Attr = Attr::new("autocomplete");

/// `autocorrect`, on every HTML element.
pub const AUTOCORRECT: Attr = Attr::new("autocorrect");

/// `autofocus`, on every HTML element.
pub const AUTOFOCUS: Attr = Attr::new("autofocus");

/// `autoplay`, on `<audio>`, `<video>`.
pub const AUTOPLAY: Attr = Attr::new("autoplay");

/// `axis`, on `<td>`, `<th>`.
pub const AXIS: Attr = Attr::new("axis");

/// `azimuth`, on `<feDistantLight>`.
pub const AZIMUTH: Attr = Attr::new("azimuth");

/// `background`, on `<body>`.
pub const BACKGROUND: Attr = Attr::new("background");

/// `bandwidth`, on `<prefetch>`.
pub const BANDWIDTH: Attr = Attr::new("bandwidth");

/// `baseline-shift`, on every SVG element.
pub const BASELINE_SHIFT: Attr = Attr::new("baseline-shift");

/// `baseFrequency`, on `<feTurbulence>`.
pub const BASE_FREQUENCY: Attr = Attr::new("baseFrequency");

/// `baseProfile`, on `<svg>`.
pub const BASE_PROFILE: Attr = Attr::new("baseProfile");

/// `bbox`, on `<font-face>`.
pub const BBOX: Attr = Attr::new("bbox");

/// `begin`, on 9 SVG elements.
pub const BEGIN: Attr = Attr::new("begin");

/// `bgcolor`, on `<body>`, `<table>`, `<td>`, `<th>`, `<tr>`.
pub const BGCOLOR: Attr = Attr::new("bgcolor");

/// `bias`, on `<feConvolveMatrix>`.
pub const BIAS: Attr = Attr::new("bias");

/// `blocking`, on `<link>`, `<script>`, `<style>`.
pub const BLOCKING: Attr = Attr::new("blocking");

/// `border`, on `<img>`, `<object>`, `<table>`.
pub const BORDER: Attr = Attr::new("border");

/// `by`, on `<animate>`, `<animateColor>`, `<animateMotion>`, `<animateTransform>`.
pub const BY: Attr = Attr::new("by");

/// `calcMode`, on `<animate>`, `<animateColor>`, `<animateMotion>`, `<animateTransform>`.
pub const CALC_MODE: Attr = Attr::new("calcMode");

/// `cap-height`, on `<font-face>`.
pub const CAP_HEIGHT: Attr = Attr::new("cap-height");

/// `cellpadding`, on `<table>`.
pub const CELLPADDING: Attr = Attr::new("cellpadding");

/// `cellspacing`, on `<table>`.
pub const CELLSPACING: Attr = Attr::new("cellspacing");

/// `char`, on 8 HTML elements.
pub const CHAR: Attr = Attr::new("char");

/// `charoff`, on 8 HTML elements.
pub const CHAROFF: Attr = Attr::new("charoff");

/// `charset`, on `<a>`, `<link>`, `<meta>`, `<script>`.
pub const CHARSET: Attr = Attr::new("charset");

/// `checked`, on `<input>`.
pub const CHECKED: Attr = Attr::new("checked");

/// `cite`, on `<blockquote>`, `<del>`, `<ins>`, `<q>`.
pub const CITE: Attr = Attr::new("cite");

/// `classid`, on `<object>`.
pub const CLASSID: Attr = Attr::new("classid");

/// `clear`, on `<br>`.
pub const CLEAR: Attr = Attr::new("clear");

/// `clip`, on 51 SVG elements.
pub const CLIP: Attr = Attr::new("clip");

/// `clip-path`, on every SVG element.
pub const CLIP_PATH: Attr = Attr::new("clip-path");

/// `clipPathUnits`, on `<clipPath>`.
pub const CLIP_PATH_UNITS: Attr = Attr::new("clipPathUnits");

/// `clip-rule`, on every SVG element.
pub const CLIP_RULE: Attr = Attr::new("clip-rule");

/// `closedby`, on `<dialog>`.
pub const CLOSEDBY: Attr = Attr::new("closedby");

/// `code`, on `<applet>`.
pub const CODE: Attr = Attr::new("code");

/// `codebase`, on `<applet>`, `<object>`.
pub const CODEBASE: Attr = Attr::new("codebase");

/// `codetype`, on `<object>`.
pub const CODETYPE: Attr = Attr::new("codetype");

/// `color`, on `<basefont>`, `<font>`, `<link>`; every SVG element.
pub const COLOR: Attr = Attr::new("color");

/// `colorspace`, on `<input>`.
pub const COLORSPACE: Attr = Attr::new("colorspace");

/// `color-interpolation`, on every SVG element.
pub const COLOR_INTERPOLATION: Attr = Attr::new("color-interpolation");

/// `color-interpolation-filters`, on every SVG element.
pub const COLOR_INTERPOLATION_FILTERS: Attr = Attr::new("color-interpolation-filters");

/// `color-profile`, on 51 SVG elements.
pub const COLOR_PROFILE: Attr = Attr::new("color-profile");

/// `color-rendering`, on every SVG element.
pub const COLOR_RENDERING: Attr = Attr::new("color-rendering");

/// `cols`, on `<frameset>`, `<textarea>`.
pub const COLS: Attr = Attr::new("cols");

/// `colspan`, on `<td>`, `<th>`.
pub const COLSPAN: Attr = Attr::new("colspan");

/// `command`, on `<button>`.
pub const COMMAND: Attr = Attr::new("command");

/// `compact`, on `<dir>`, `<dl>`, `<menu>`, `<ol>`, `<ul>`.
pub const COMPACT: Attr = Attr::new("compact");

/// `content`, on `<meta>`; every SVG element.
pub const CONTENT: Attr = Attr::new("content");

/// `contenteditable`, on every HTML element.
pub const CONTENTEDITABLE: Attr = Attr::new("contenteditable");

/// `contentScriptType`, on `<svg>`.
pub const CONTENT_SCRIPT_TYPE: Attr = Attr::new("contentScriptType");

/// `contentStyleType`, on `<svg>`.
pub const CONTENT_STYLE_TYPE: Attr = Attr::new("contentStyleType");

/// `controls`, on `<audio>`, `<video>`.
pub const CONTROLS: Attr = Attr::new("controls");

/// `coords`, on `<a>`, `<area>`.
pub const COORDS: Attr = Attr::new("coords");

/// `crossorigin`, on `<audio>`, `<img>`, `<link>`, `<script>`, `<video>`; `<feImage>`, `<image>`, `<script>`.
pub const CROSSORIGIN: Attr = Attr::new("crossorigin");

/// `cursor`, on every SVG element.
pub const CURSOR: Attr = Attr::new("cursor");

/// `cx`, on `<circle>`, `<ellipse>`, `<radialGradient>`.
pub const CX: Attr = Attr::new("cx");

/// `cy`, on `<circle>`, `<ellipse>`, `<radialGradient>`.
pub const CY: Attr = Attr::new("cy");

/// `d`, on `<glyph>`, `<missing-glyph>`, `<path>`.
pub const D: Attr = Attr::new("d");

/// `datatype`, on every SVG element.
pub const DATATYPE: Attr = Attr::new("datatype");

/// `datetime`, on `<del>`, `<ins>`, `<time>`.
pub const DATETIME: Attr = Attr::new("datetime");

/// `declare`, on `<object>`.
pub const DECLARE: Attr = Attr::new("declare");

/// `decoding`, on `<img>`.
pub const DECODING: Attr = Attr::new("decoding");

/// `default`, on `<track>`.
pub const DEFAULT: Attr = Attr::new("default");

/// `defaultAction`, on `<listener>`.
pub const DEFAULT_ACTION: Attr = Attr::new("defaultAction");

/// `defer`, on `<script>`.
pub const DEFER: Attr = Attr::new("defer");

/// `descent`, on `<font-face>`.
pub const DESCENT: Attr = Attr::new("descent");

/// `diffuseConstant`, on `<feDiffuseLighting>`.
pub const DIFFUSE_CONSTANT: Attr = Attr::new("diffuseConstant");

/// `dir`, on every HTML element.
pub const DIR: Attr = Attr::new("dir");

/// `direction`, on every SVG element.
pub const DIRECTION: Attr = Attr::new("direction");

/// `dirname`, on `<input>`, `<textarea>`.
pub const DIRNAME: Attr = Attr::new("dirname");

/// `disabled`, on 8 HTML elements.
pub const DISABLED: Attr = Attr::new("disabled");

/// `display`, on every SVG element.
pub const DISPLAY: Attr = Attr::new("display");

/// `divisor`, on `<feConvolveMatrix>`.
pub const DIVISOR: Attr = Attr::new("divisor");

/// `dominant-baseline`, on every SVG element.
pub const DOMINANT_BASELINE: Attr = Attr::new("dominant-baseline");

/// `download`, on `<a>`, `<area>`; `<a>`.
pub const DOWNLOAD: Attr = Attr::new("download");

/// `draggable`, on every HTML element.
pub const DRAGGABLE: Attr = Attr::new("draggable");

/// `dur`, on 8 SVG elements.
pub const DUR: Attr = Attr::new("dur");

/// `dx`, on 7 SVG elements.
pub const DX: Attr = Attr::new("dx");

/// `dy`, on 7 SVG elements.
pub const DY: Attr = Attr::new("dy");

/// `edgeMode`, on `<feConvolveMatrix>`, `<feGaussianBlur>`.
pub const EDGE_MODE: Attr = Attr::new("edgeMode");

/// `editable`, on `<text>`, `<textArea>`.
pub const EDITABLE: Attr = Attr::new("editable");

/// `elevation`, on `<feDistantLight>`.
pub const ELEVATION: Attr = Attr::new("elevation");

/// `enable-background`, on 51 SVG elements.
pub const ENABLE_BACKGROUND: Attr = Attr::new("enable-background");

/// `enctype`, on `<form>`.
pub const ENCTYPE: Attr = Attr::new("enctype");

/// `end`, on 8 SVG elements.
pub const END: Attr = Attr::new("end");

/// `enterkeyhint`, on every HTML element.
pub const ENTERKEYHINT: Attr = Attr::new("enterkeyhint");

/// `event`, on `<listener>`.
pub const EVENT: Attr = Attr::new("event");

/// `exponent`, on `<feFuncA>`, `<feFuncB>`, `<feFuncG>`, `<feFuncR>`.
pub const EXPONENT: Attr = Attr::new("exponent");

/// `exportparts`, on every HTML element.
pub const EXPORTPARTS: Attr = Attr::new("exportparts");

/// `externalResourcesRequired`, on 45 SVG elements.
pub const EXTERNAL_RESOURCES_REQUIRED: Attr = Attr::new("externalResourcesRequired");

/// `face`, on `<basefont>`, `<font>`.
pub const FACE: Attr = Attr::new("face");

/// `fetchpriority`, on `<img>`, `<link>`, `<script>`.
pub const FETCHPRIORITY: Attr = Attr::new("fetchpriority");

/// `fill`, on every SVG element.
pub const FILL: Attr = Attr::new("fill");

/// `fill-opacity`, on every SVG element.
pub const FILL_OPACITY: Attr = Attr::new("fill-opacity");

/// `fill-rule`, on every SVG element.
pub const FILL_RULE: Attr = Attr::new("fill-rule");

/// `filter`, on every SVG element.
pub const FILTER: Attr = Attr::new("filter");

/// `filterRes`, on `<filter>`.
pub const FILTER_RES: Attr = Attr::new("filterRes");

/// `filterUnits`, on `<filter>`.
pub const FILTER_UNITS: Attr = Attr::new("filterUnits");

/// `flood-color`, on every SVG element.
pub const FLOOD_COLOR: Attr = Attr::new("flood-color");

/// `flood-opacity`, on every SVG element.
pub const FLOOD_OPACITY: Attr = Attr::new("flood-opacity");

/// `focusable`, on 19 SVG elements.
pub const FOCUSABLE: Attr = Attr::new("focusable");

/// `focusHighlight`, on 19 SVG elements.
pub const FOCUS_HIGHLIGHT: Attr = Attr::new("focusHighlight");

/// `font-family`, on every SVG element.
pub const FONT_FAMILY: Attr = Attr::new("font-family");

/// `font-size`, on every SVG element.
pub const FONT_SIZE: Attr = Attr::new("font-size");

/// `font-size-adjust`, on every SVG element.
pub const FONT_SIZE_ADJUST: Attr = Attr::new("font-size-adjust");

/// `font-stretch`, on every SVG element.
pub const FONT_STRETCH: Attr = Attr::new("font-stretch");

/// `font-style`, on every SVG element.
pub const FONT_STYLE: Attr = Attr::new("font-style");

/// `font-variant`, on every SVG element.
pub const FONT_VARIANT: Attr = Attr::new("font-variant");

/// `font-weight`, on every SVG element.
pub const FONT_WEIGHT: Attr = Attr::new("font-weight");

/// `format`, on `<altGlyph>`, `<glyphRef>`.
pub const FORMAT: Attr = Attr::new("format");

/// `formenctype`, on `<button>`, `<input>`.
pub const FORMENCTYPE: Attr = Attr::new("formenctype");

/// `formmethod`, on `<button>`, `<input>`.
pub const FORMMETHOD: Attr = Attr::new("formmethod");

/// `formnovalidate`, on `<button>`, `<input>`.
pub const FORMNOVALIDATE: Attr = Attr::new("formnovalidate");

/// `formtarget`, on `<button>`, `<input>`.
pub const FORMTARGET: Attr = Attr::new("formtarget");

/// `fr`, on `<radialGradient>`.
pub const FR: Attr = Attr::new("fr");

/// `frame`, on `<table>`.
pub const FRAME: Attr = Attr::new("frame");

/// `frameborder`, on `<frame>`, `<iframe>`.
pub const FRAMEBORDER: Attr = Attr::new("frameborder");

/// `from`, on `<animate>`, `<animateColor>`, `<animateMotion>`, `<animateTransform>`.
pub const FROM: Attr = Attr::new("from");

/// `fx`, on `<radialGradient>`.
pub const FX: Attr = Attr::new("fx");

/// `fy`, on `<radialGradient>`.
pub const FY: Attr = Attr::new("fy");

/// `g1`, on `<hkern>`, `<vkern>`.
pub const G1: Attr = Attr::new("g1");

/// `g2`, on `<hkern>`, `<vkern>`.
pub const G2: Attr = Attr::new("g2");

/// `glyph-name`, on `<glyph>`.
pub const GLYPH_NAME: Attr = Attr::new("glyph-name");

/// `glyph-orientation-horizontal`, on every SVG element.
pub const GLYPH_ORIENTATION_HORIZONTAL: Attr = Attr::new("glyph-orientation-horizontal");

/// `glyph-orientation-vertical`, on every SVG element.
pub const GLYPH_ORIENTATION_VERTICAL: Attr = Attr::new("glyph-orientation-vertical");

/// `glyphRef`, on `<altGlyph>`, `<glyphRef>`.
pub const GLYPH_REF: Attr = Attr::new("glyphRef");

/// `gradientTransform`, on `<linearGradient>`, `<radialGradient>`.
pub const GRADIENT_TRANSFORM: Attr = Attr::new("gradientTransform");

/// `gradientUnits`, on `<linearGradient>`, `<radialGradient>`.
pub const GRADIENT_UNITS: Attr = Attr::new("gradientUnits");

/// `handler`, on `<listener>`.
pub const HANDLER: Attr = Attr::new("handler");

/// `hanging`, on `<font-face>`.
pub const HANGING: Attr = Attr::new("hanging");

/// `height`, on 11 HTML elements; 29 SVG elements.
pub const HEIGHT: Attr = Attr::new("height");

/// `hidden`, on every HTML element.
pub const HIDDEN: Attr = Attr::new("hidden");

/// `high`, on `<meter>`.
pub const HIGH: Attr = Attr::new("high");

/// `horiz-adv-x`, on `<font>`, `<glyph>`, `<missing-glyph>`.
pub const HORIZ_ADV_X: Attr = Attr::new("horiz-adv-x");

/// `horiz-origin-x`, on `<font>`.
pub const HORIZ_ORIGIN_X: Attr = Attr::new("horiz-origin-x");

/// `horiz-origin-y`, on `<font>`.
pub const HORIZ_ORIGIN_Y: Attr = Attr::new("horiz-origin-y");

/// `hreflang`, on `<a>`, `<area>`, `<link>`; `<a>`.
pub const HREFLANG: Attr = Attr::new("hreflang");

/// `hspace`, on `<applet>`, `<img>`, `<object>`.
pub const HSPACE: Attr = Attr::new("hspace");

/// `ideographic`, on `<font-face>`.
pub const IDEOGRAPHIC: Attr = Attr::new("ideographic");

/// `imagesizes`, on `<link>`.
pub const IMAGESIZES: Attr = Attr::new("imagesizes");

/// `imagesrcset`, on `<link>`.
pub const IMAGESRCSET: Attr = Attr::new("imagesrcset");

/// `image-rendering`, on every SVG element.
pub const IMAGE_RENDERING: Attr = Attr::new("image-rendering");

/// `in`, on 14 SVG elements.
pub const IN: Attr = Attr::new("in");

/// `in2`, on `<feBlend>`, `<feComposite>`, `<feDisplacementMap>`.
pub const IN2: Attr = Attr::new("in2");

/// `inert`, on every HTML element.
pub const INERT: Attr = Attr::new("inert");

/// `initialVisibility`, on `<animation>`, `<video>`.
pub const INITIAL_VISIBILITY: Attr = Attr::new("initialVisibility");

/// `inputmode`, on every HTML element.
pub const INPUTMODE: Attr = Attr::new("inputmode");

/// `integrity`, on `<link>`, `<script>`.
pub const INTEGRITY: Attr = Attr::new("integrity");

/// `intercept`, on `<feFuncA>`, `<feFuncB>`, `<feFuncG>`, `<feFuncR>`.
pub const INTERCEPT: Attr = Attr::new("intercept");

/// `is`, on every HTML element.
pub const IS: Attr = Attr::new("is");

/// `ismap`, on `<img>`, `<input>`.
pub const ISMAP: Attr = Attr::new("ismap");

/// `itemid`, on every HTML element.
pub const ITEMID: Attr = Attr::new("itemid");

/// `itemprop`, on every HTML element.
pub const ITEMPROP: Attr = Attr::new("itemprop");

/// `itemscope`, on every HTML element.
pub const ITEMSCOPE: Attr = Attr::new("itemscope");

/// `itemtype`, on every HTML element.
pub const ITEMTYPE: Attr = Attr::new("itemtype");

/// `k`, on `<hkern>`, `<vkern>`.
pub const K: Attr = Attr::new("k");

/// `k1`, on `<feComposite>`.
pub const K1: Attr = Attr::new("k1");

/// `k2`, on `<feComposite>`.
pub const K2: Attr = Attr::new("k2");

/// `k3`, on `<feComposite>`.
pub const K3: Attr = Attr::new("k3");

/// `k4`, on `<feComposite>`.
pub const K4: Attr = Attr::new("k4");

/// `kernelMatrix`, on `<feConvolveMatrix>`.
pub const KERNEL_MATRIX: Attr = Attr::new("kernelMatrix");

/// `kernelUnitLength`, on `<feConvolveMatrix>`, `<feDiffuseLighting>`, `<feSpecularLighting>`.
pub const KERNEL_UNIT_LENGTH: Attr = Attr::new("kernelUnitLength");

/// `kerning`, on 51 SVG elements.
pub const KERNING: Attr = Attr::new("kerning");

/// `keyPoints`, on `<animateMotion>`.
pub const KEY_POINTS: Attr = Attr::new("keyPoints");

/// `keySplines`, on `<animate>`, `<animateColor>`, `<animateMotion>`, `<animateTransform>`.
pub const KEY_SPLINES: Attr = Attr::new("keySplines");

/// `keyTimes`, on `<animate>`, `<animateColor>`, `<animateMotion>`, `<animateTransform>`.
pub const KEY_TIMES: Attr = Attr::new("keyTimes");

/// `kind`, on `<track>`.
pub const KIND: Attr = Attr::new("kind");

/// `label`, on `<optgroup>`, `<option>`, `<track>`.
pub const LABEL: Attr = Attr::new("label");

/// `lang`, on every HTML element; every SVG element.
pub const LANG: Attr = Attr::new("lang");

/// `language`, on `<script>`.
pub const LANGUAGE: Attr = Attr::new("language");

/// `lengthAdjust`, on `<text>`, `<textPath>`, `<tref>`, `<tspan>`.
pub const LENGTH_ADJUST: Attr = Attr::new("lengthAdjust");

/// `letter-spacing`, on every SVG element.
pub const LETTER_SPACING: Attr = Attr::new("letter-spacing");

/// `lighting-color`, on every SVG element.
pub const LIGHTING_COLOR: Attr = Attr::new("lighting-color");

/// `limitingConeAngle`, on `<feSpotLight>`.
pub const LIMITING_CONE_ANGLE: Attr = Attr::new("limitingConeAngle");

/// `link`, on `<body>`.
pub const LINK: Attr = Attr::new("link");

/// `loading`, on `<iframe>`, `<img>`.
pub const LOADING: Attr = Attr::new("loading");

/// `local`, on `<color-profile>`.
pub const LOCAL: Attr = Attr::new("local");

/// `longdesc`, on `<frame>`, `<iframe>`, `<img>`.
pub const LONGDESC: Attr = Attr::new("longdesc");

/// `loop`, on `<audio>`, `<video>`.
pub const LOOP: Attr = Attr::new("loop");

/// `low`, on `<meter>`.
pub const LOW: Attr = Attr::new("low");

/// `manifest`, on `<html>`.
pub const MANIFEST: Attr = Attr::new("manifest");

/// `marginheight`, on `<frame>`, `<iframe>`.
pub const MARGINHEIGHT: Attr = Attr::new("marginheight");

/// `marginwidth`, on `<frame>`, `<iframe>`.
pub const MARGINWIDTH: Attr = Attr::new("marginwidth");

/// `marker-end`, on every SVG element.
pub const MARKER_END: Attr = Attr::new("marker-end");

/// `markerHeight`, on `<marker>`.
pub const MARKER_HEIGHT: Attr = Attr::new("markerHeight");

/// `marker-mid`, on every SVG element.
pub const MARKER_MID: Attr = Attr::new("marker-mid");

/// `marker-start`, on every SVG element.
pub const MARKER_START: Attr = Attr::new("marker-start");

/// `markerUnits`, on `<marker>`.
pub const MARKER_UNITS: Attr = Attr::new("markerUnits");

/// `markerWidth`, on `<marker>`.
pub const MARKER_WIDTH: Attr = Attr::new("markerWidth");

/// `mask`, on every SVG element.
pub const MASK: Attr = Attr::new("mask");

/// `maskContentUnits`, on `<mask>`.
pub const MASK_CONTENT_UNITS: Attr = Attr::new("maskContentUnits");

/// `maskUnits`, on `<mask>`.
pub const MASK_UNITS: Attr = Attr::new("maskUnits");

/// `mathematical`, on `<font-face>`.
pub const MATHEMATICAL: Attr = Attr::new("mathematical");

/// `max`, on `<input>`, `<meter>`, `<progress>`; 8 SVG elements.
pub const MAX: Attr = Attr::new("max");

/// `maxlength`, on `<input>`, `<textarea>`.
pub const MAXLENGTH: Attr = Attr::new("maxlength");

/// `media`, on `<link>`, `<meta>`, `<source>`, `<style>`; `<style>`.
pub const MEDIA: Attr = Attr::new("media");

/// `mediaCharacterEncoding`, on `<prefetch>`.
pub const MEDIA_CHARACTER_ENCODING: Attr = Attr::new("mediaCharacterEncoding");

/// `mediaContentEncodings`, on `<prefetch>`.
pub const MEDIA_CONTENT_ENCODINGS: Attr = Attr::new("mediaContentEncodings");

/// `mediaSize`, on `<prefetch>`.
pub const MEDIA_SIZE: Attr = Attr::new("mediaSize");

/// `mediaTime`, on `<prefetch>`.
pub const MEDIA_TIME: Attr = Attr::new("mediaTime");

/// `method`, on `<form>`; `<textPath>`.
pub const METHOD: Attr = Attr::new("method");

/// `min`, on `<input>`, `<meter>`; 8 SVG elements.
pub const MIN: Attr = Attr::new("min");

/// `minlength`, on `<input>`, `<textarea>`.
pub const MINLENGTH: Attr = Attr::new("minlength");

/// `mode`, on `<feBlend>`.
pub const MODE: Attr = Attr::new("mode");

/// `multiple`, on `<input>`, `<select>`.
pub const MULTIPLE: Attr = Attr::new("multiple");

/// `muted`, on `<audio>`, `<video>`.
pub const MUTED: Attr = Attr::new("muted");

/// `name`, on 18 HTML elements; `<color-profile>`, `<font-face-name>`.
pub const NAME: Attr = Attr::new("name");

/// `nav-down`, on 19 SVG elements.
pub const NAV_DOWN: Attr = Attr::new("nav-down");

/// `nav-down-left`, on 19 SVG elements.
pub const NAV_DOWN_LEFT: Attr = Attr::new("nav-down-left");

/// `nav-down-right`, on 19 SVG elements.
pub const NAV_DOWN_RIGHT: Attr = Attr::new("nav-down-right");

/// `nav-left`, on 19 SVG elements.
pub const NAV_LEFT: Attr = Attr::new("nav-left");

/// `nav-next`, on 19 SVG elements.
pub const NAV_NEXT: Attr = Attr::new("nav-next");

/// `nav-prev`, on 19 SVG elements.
pub const NAV_PREV: Attr = Attr::new("nav-prev");

/// `nav-right`, on 19 SVG elements.
pub const NAV_RIGHT: Attr = Attr::new("nav-right");

/// `nav-up`, on 19 SVG elements.
pub const NAV_UP: Attr = Attr::new("nav-up");

/// `nav-up-left`, on 19 SVG elements.
pub const NAV_UP_LEFT: Attr = Attr::new("nav-up-left");

/// `nav-up-right`, on 19 SVG elements.
pub const NAV_UP_RIGHT: Attr = Attr::new("nav-up-right");

/// `nohref`, on `<area>`.
pub const NOHREF: Attr = Attr::new("nohref");

/// `nomodule`, on `<script>`.
pub const NOMODULE: Attr = Attr::new("nomodule");

/// `nonce`, on every HTML element.
pub const NONCE: Attr = Attr::new("nonce");

/// `noresize`, on `<frame>`.
pub const NORESIZE: Attr = Attr::new("noresize");

/// `noshade`, on `<hr>`.
pub const NOSHADE: Attr = Attr::new("noshade");

/// `novalidate`, on `<form>`.
pub const NOVALIDATE: Attr = Attr::new("novalidate");

/// `nowrap`, on `<td>`, `<th>`.
pub const NOWRAP: Attr = Attr::new("nowrap");

/// `numOctaves`, on `<feTurbulence>`.
pub const NUM_OCTAVES: Attr = Attr::new("numOctaves");

/// `object`, on `<applet>`.
pub const OBJECT: Attr = Attr::new("object");

/// `observer`, on `<listener>`.
pub const OBSERVER: Attr = Attr::new("observer");

/// `offset`, on `<feFuncA>`, `<feFuncB>`, `<feFuncG>`, `<feFuncR>`, `<stop>`.
pub const OFFSET: Attr = Attr::new("offset");

/// `opacity`, on every SVG element.
pub const OPACITY: Attr = Attr::new("opacity");

/// `open`, on `<details>`, `<dialog>`.
pub const OPEN: Attr = Attr::new("open");

/// `operator`, on `<feComposite>`, `<feMorphology>`.
pub const OPERATOR: Attr = Attr::new("operator");

/// `optimum`, on `<meter>`.
pub const OPTIMUM: Attr = Attr::new("optimum");

/// `order`, on `<feConvolveMatrix>`.
pub const ORDER: Attr = Attr::new("order");

/// `orient`, on `<marker>`.
pub const ORIENT: Attr = Attr::new("orient");

/// `orientation`, on `<glyph>`.
pub const ORIENTATION: Attr = Attr::new("orientation");

/// `origin`, on `<animateMotion>`.
pub const ORIGIN: Attr = Attr::new("origin");

/// `overflow`, on every SVG element.
pub const OVERFLOW: Attr = Attr::new("overflow");

/// `overlay`, on `<video>`.
pub const OVERLAY: Attr = Attr::new("overlay");

/// `overline-position`, on `<font-face>`.
pub const OVERLINE_POSITION: Attr = Attr::new("overline-position");

/// `overline-thickness`, on `<font-face>`.
pub const OVERLINE_THICKNESS: Attr = Attr::new("overline-thickness");

/// `paint-order`, on every SVG element.
pub const PAINT_ORDER: Attr = Attr::new("paint-order");

/// `panose-1`, on `<font-face>`.
pub const PANOSE_1: Attr = Attr::new("panose-1");

/// `part`, on every HTML element.
pub const PART: Attr = Attr::new("part");

/// `path`, on `<animateMotion>`, `<textPath>`.
pub const PATH: Attr = Attr::new("path");

/// `pathLength`, on 7 SVG elements.
pub const PATH_LENGTH: Attr = Attr::new("pathLength");

/// `pattern`, on `<input>`.
pub const PATTERN: Attr = Attr::new("pattern");

/// `patternContentUnits`, on `<pattern>`.
pub const PATTERN_CONTENT_UNITS: Attr = Attr::new("patternContentUnits");

/// `patternTransform`, on `<pattern>`.
pub const PATTERN_TRANSFORM: Attr = Attr::new("patternTransform");

/// `patternUnits`, on `<pattern>`.
pub const PATTERN_UNITS: Attr = Attr::new("patternUnits");

/// `phase`, on `<listener>`.
pub const PHASE: Attr = Attr::new("phase");

/// `ping`, on `<a>`, `<area>`; `<a>`.
pub const PING: Attr = Attr::new("ping");

/// `placeholder`, on `<input>`, `<textarea>`.
pub const PLACEHOLDER: Attr = Attr::new("placeholder");

/// `playbackorder`, on `<svg>`.
pub const PLAYBACKORDER: Attr = Attr::new("playbackorder");

/// `playbackOrder`, on `<svg>`.
pub const PLAYBACK_ORDER: Attr = Attr::new("playbackOrder");

/// `playsinline`, on `<video>`.
pub const PLAYSINLINE: Attr = Attr::new("playsinline");

/// `pointer-events`, on every SVG element.
pub const POINTER_EVENTS: Attr = Attr::new("pointer-events");

/// `points`, on `<polygon>`, `<polyline>`.
pub const POINTS: Attr = Attr::new("points");

/// `pointsAtX`, on `<feSpotLight>`.
pub const POINTS_AT_X: Attr = Attr::new("pointsAtX");

/// `pointsAtY`, on `<feSpotLight>`.
pub const POINTS_AT_Y: Attr = Attr::new("pointsAtY");

/// `pointsAtZ`, on `<feSpotLight>`.
pub const POINTS_AT_Z: Attr = Attr::new("pointsAtZ");

/// `popover`, on every HTML element.
pub const POPOVER: Attr = Attr::new("popover");

/// `popovertargetaction`, on `<button>`, `<input>`.
pub const POPOVERTARGETACTION: Attr = Attr::new("popovertargetaction");

/// `poster`, on `<video>`.
pub const POSTER: Attr = Attr::new("poster");

/// `preload`, on `<audio>`, `<video>`.
pub const PRELOAD: Attr = Attr::new("preload");

/// `preserveAlpha`, on `<feConvolveMatrix>`.
pub const PRESERVE_ALPHA: Attr = Attr::new("preserveAlpha");

/// `preserveAspectRatio`, on 10 SVG elements.
pub const PRESERVE_ASPECT_RATIO: Attr = Attr::new("preserveAspectRatio");

/// `primitiveUnits`, on `<filter>`.
pub const PRIMITIVE_UNITS: Attr = Attr::new("primitiveUnits");

/// `profile`, on `<head>`.
pub const PROFILE: Attr = Attr::new("profile");

/// `prompt`, on `<isindex>`.
pub const PROMPT: Attr = Attr::new("prompt");

/// `propagate`, on `<listener>`.
pub const PROPAGATE: Attr = Attr::new("propagate");

/// `property`, on every SVG element.
pub const PROPERTY: Attr = Attr::new("property");

/// `r`, on `<circle>`, `<radialGradient>`.
pub const R: Attr = Attr::new("r");

/// `radius`, on `<feMorphology>`.
pub const RADIUS: Attr = Attr::new("radius");

/// `readonly`, on `<input>`, `<textarea>`.
pub const READONLY: Attr = Attr::new("readonly");

/// `referrerpolicy`, on `<a>`, `<area>`, `<iframe>`, `<img>`, `<link>`, `<script>`; `<a>`.
pub const REFERRERPOLICY: Attr = Attr::new("referrerpolicy");

/// `refX`, on `<marker>`, `<symbol>`.
pub const REF_X: Attr = Attr::new("refX");

/// `refY`, on `<marker>`, `<symbol>`.
pub const REF_Y: Attr = Attr::new("refY");

/// `rel`, on `<a>`, `<area>`, `<link>`; every SVG element.
pub const REL: Attr = Attr::new("rel");

/// `rendering-intent`, on `<color-profile>`.
pub const RENDERING_INTENT: Attr = Attr::new("rendering-intent");

/// `repeatCount`, on 8 SVG elements.
pub const REPEAT_COUNT: Attr = Attr::new("repeatCount");

/// `repeatDur`, on 8 SVG elements.
pub const REPEAT_DUR: Attr = Attr::new("repeatDur");

/// `required`, on `<input>`, `<select>`, `<textarea>`.
pub const REQUIRED: Attr = Attr::new("required");

/// `requiredExtensions`, on 41 SVG elements.
pub const REQUIRED_EXTENSIONS: Attr = Attr::new("requiredExtensions");

/// `requiredFeatures`, on 38 SVG elements.
pub const REQUIRED_FEATURES: Attr = Attr::new("requiredFeatures");

/// `requiredFonts`, on 29 SVG elements.
pub const REQUIRED_FONTS: Attr = Attr::new("requiredFonts");

/// `requiredFormats`, on 29 SVG elements.
pub const REQUIRED_FORMATS: Attr = Attr::new("requiredFormats");

/// `resource`, on every SVG element.
pub const RESOURCE: Attr = Attr::new("resource");

/// `restart`, on 8 SVG elements.
pub const RESTART: Attr = Attr::new("restart");

/// `result`, on 17 SVG elements.
pub const RESULT: Attr = Attr::new("result");

/// `rev`, on `<a>`, `<link>`; every SVG element.
pub const REV: Attr = Attr::new("rev");

/// `reversed`, on `<ol>`.
pub const REVERSED: Attr = Attr::new("reversed");

/// `rotate`, on `<altGlyph>`, `<animateMotion>`, `<text>`, `<tref>`, `<tspan>`.
pub const ROTATE: Attr = Attr::new("rotate");

/// `rows`, on `<frameset>`, `<textarea>`.
pub const ROWS: Attr = Attr::new("rows");

/// `rowspan`, on `<td>`, `<th>`.
pub const ROWSPAN: Attr = Attr::new("rowspan");

/// `rules`, on `<table>`.
pub const RULES: Attr = Attr::new("rules");

/// `rx`, on `<ellipse>`, `<rect>`.
pub const RX: Attr = Attr::new("rx");

/// `ry`, on `<ellipse>`, `<rect>`.
pub const RY: Attr = Attr::new("ry");

/// `sandbox`, on `<iframe>`.
pub const SANDBOX: Attr = Attr::new("sandbox");

/// `scale`, on `<feDisplacementMap>`.
pub const SCALE: Attr = Attr::new("scale");

/// `scheme`, on `<meta>`.
pub const SCHEME: Attr = Attr::new("scheme");

/// `scope`, on `<td>`, `<th>`.
pub const SCOPE: Attr = Attr::new("scope");

/// `scrolling`, on `<frame>`, `<iframe>`.
pub const SCROLLING: Attr = Attr::new("scrolling");

/// `seed`, on `<feTurbulence>`.
pub const SEED: Attr = Attr::new("seed");

/// `selected`, on `<option>`.
pub const SELECTED: Attr = Attr::new("selected");

/// `shadowrootclonable`, on `<template>`.
pub const SHADOWROOTCLONABLE: Attr = Attr::new("shadowrootclonable");

/// `shadowrootcustomelementregistry`, on `<template>`.
pub const SHADOWROOTCUSTOMELEMENTREGISTRY: Attr = Attr::new("shadowrootcustomelementregistry");

/// `shadowrootdelegatesfocus`, on `<template>`.
pub const SHADOWROOTDELEGATESFOCUS: Attr = Attr::new("shadowrootdelegatesfocus");

/// `shadowrootmode`, on `<template>`.
pub const SHADOWROOTMODE: Attr = Attr::new("shadowrootmode");

/// `shadowrootserializable`, on `<template>`.
pub const SHADOWROOTSERIALIZABLE: Attr = Attr::new("shadowrootserializable");

/// `shape`, on `<a>`, `<area>`.
pub const SHAPE: Attr = Attr::new("shape");

/// `shape-rendering`, on every SVG element.
pub const SHAPE_RENDERING: Attr = Attr::new("shape-rendering");

/// `side`, on `<textPath>`.
pub const SIDE: Attr = Attr::new("side");

/// `size`, on `<basefont>`, `<font>`, `<hr>`, `<input>`, `<select>`.
pub const SIZE: Attr = Attr::new("size");

/// `sizes`, on `<img>`, `<link>`, `<source>`.
pub const SIZES: Attr = Attr::new("sizes");

/// `slope`, on `<feFuncA>`, `<feFuncB>`, `<feFuncG>`, `<feFuncR>`, `<font-face>`.
pub const SLOPE: Attr = Attr::new("slope");

/// `slot`, on every HTML element.
pub const SLOT: Attr = Attr::new("slot");

/// `snapshotTime`, on `<svg>`.
pub const SNAPSHOT_TIME: Attr = Attr::new("snapshotTime");

/// `spacing`, on `<textPath>`.
pub const SPACING: Attr = Attr::new("spacing");

/// `span`, on `<col>`, `<colgroup>`.
pub const SPAN: Attr = Attr::new("span");

/// `specularConstant`, on `<feSpecularLighting>`.
pub const SPECULAR_CONSTANT: Attr = Attr::new("specularConstant");

/// `specularExponent`, on `<feSpecularLighting>`, `<feSpotLight>`.
pub const SPECULAR_EXPONENT: Attr = Attr::new("specularExponent");

/// `spellcheck`, on every HTML element.
pub const SPELLCHECK: Attr = Attr::new("spellcheck");

/// `spreadMethod`, on `<linearGradient>`, `<radialGradient>`.
pub const SPREAD_METHOD: Attr = Attr::new("spreadMethod");

/// `srclang`, on `<track>`.
pub const SRCLANG: Attr = Attr::new("srclang");

/// `srcset`, on `<img>`, `<source>`.
pub const SRCSET: Attr = Attr::new("srcset");

/// `standby`, on `<object>`.
pub const STANDBY: Attr = Attr::new("standby");

/// `start`, on `<ol>`.
pub const START: Attr = Attr::new("start");

/// `startOffset`, on `<textPath>`.
pub const START_OFFSET: Attr = Attr::new("startOffset");

/// `stdDeviation`, on `<feDropShadow>`, `<feGaussianBlur>`.
pub const STD_DEVIATION: Attr = Attr::new("stdDeviation");

/// `stemh`, on `<font-face>`.
pub const STEMH: Attr = Attr::new("stemh");

/// `stemv`, on `<font-face>`.
pub const STEMV: Attr = Attr::new("stemv");

/// `step`, on `<input>`.
pub const STEP: Attr = Attr::new("step");

/// `stitchTiles`, on `<feTurbulence>`.
pub const STITCH_TILES: Attr = Attr::new("stitchTiles");

/// `stop-color`, on every SVG element.
pub const STOP_COLOR: Attr = Attr::new("stop-color");

/// `stop-opacity`, on every SVG element.
pub const STOP_OPACITY: Attr = Attr::new("stop-opacity");

/// `strikethrough-position`, on `<font-face>`.
pub const STRIKETHROUGH_POSITION: Attr = Attr::new("strikethrough-position");

/// `strikethrough-thickness`, on `<font-face>`.
pub const STRIKETHROUGH_THICKNESS: Attr = Attr::new("strikethrough-thickness");

/// `string`, on `<font-face-format>`.
pub const STRING: Attr = Attr::new("string");

/// `stroke`, on every SVG element.
pub const STROKE: Attr = Attr::new("stroke");

/// `stroke-dasharray`, on every SVG element.
pub const STROKE_DASHARRAY: Attr = Attr::new("stroke-dasharray");

/// `stroke-dashoffset`, on every SVG element.
pub const STROKE_DASHOFFSET: Attr = Attr::new("stroke-dashoffset");

/// `stroke-linecap`, on every SVG element.
pub const STROKE_LINECAP: Attr = Attr::new("stroke-linecap");

/// `stroke-linejoin`, on every SVG element.
pub const STROKE_LINEJOIN: Attr = Attr::new("stroke-linejoin");

/// `stroke-miterlimit`, on every SVG element.
pub const STROKE_MITERLIMIT: Attr = Attr::new("stroke-miterlimit");

/// `stroke-opacity`, on every SVG element.
pub const STROKE_OPACITY: Attr = Attr::new("stroke-opacity");

/// `stroke-width`, on every SVG element.
pub const STROKE_WIDTH: Attr = Attr::new("stroke-width");

/// `style`, on every HTML element; every SVG element.
pub const STYLE: Attr = Attr::new("style");

/// `summary`, on `<table>`.
pub const SUMMARY: Attr = Attr::new("summary");

/// `surfaceScale`, on `<feDiffuseLighting>`, `<feSpecularLighting>`.
pub const SURFACE_SCALE: Attr = Attr::new("surfaceScale");

/// `syncBehavior`, on `<animation>`, `<audio>`, `<video>`.
pub const SYNC_BEHAVIOR: Attr = Attr::new("syncBehavior");

/// `syncBehaviorDefault`, on `<svg>`.
pub const SYNC_BEHAVIOR_DEFAULT: Attr = Attr::new("syncBehaviorDefault");

/// `syncMaster`, on `<animation>`, `<audio>`, `<video>`.
pub const SYNC_MASTER: Attr = Attr::new("syncMaster");

/// `syncTolerance`, on `<animation>`, `<audio>`, `<video>`.
pub const SYNC_TOLERANCE: Attr = Attr::new("syncTolerance");

/// `syncToleranceDefault`, on `<svg>`.
pub const SYNC_TOLERANCE_DEFAULT: Attr = Attr::new("syncToleranceDefault");

/// `systemLanguage`, on 41 SVG elements.
pub const SYSTEM_LANGUAGE: Attr = Attr::new("systemLanguage");

/// `tabindex`, on every HTML element; every SVG element.
pub const TABINDEX: Attr = Attr::new("tabindex");

/// `tableValues`, on `<feFuncA>`, `<feFuncB>`, `<feFuncG>`, `<feFuncR>`.
pub const TABLE_VALUES: Attr = Attr::new("tableValues");

/// `target`, on `<a>`, `<area>`, `<base>`, `<form>`, `<link>`; `<a>`, `<listener>`.
pub const TARGET: Attr = Attr::new("target");

/// `targetX`, on `<feConvolveMatrix>`.
pub const TARGET_X: Attr = Attr::new("targetX");

/// `targetY`, on `<feConvolveMatrix>`.
pub const TARGET_Y: Attr = Attr::new("targetY");

/// `text`, on `<body>`.
pub const TEXT: Attr = Attr::new("text");

/// `text-anchor`, on every SVG element.
pub const TEXT_ANCHOR: Attr = Attr::new("text-anchor");

/// `text-decoration`, on every SVG element.
pub const TEXT_DECORATION: Attr = Attr::new("text-decoration");

/// `textLength`, on `<text>`, `<textPath>`, `<tref>`, `<tspan>`.
pub const TEXT_LENGTH: Attr = Attr::new("textLength");

/// `text-overflow`, on every SVG element.
pub const TEXT_OVERFLOW: Attr = Attr::new("text-overflow");

/// `text-rendering`, on every SVG element.
pub const TEXT_RENDERING: Attr = Attr::new("text-rendering");

/// `timelinebegin`, on `<svg>`.
pub const TIMELINEBEGIN: Attr = Attr::new("timelinebegin");

/// `timelineBegin`, on `<svg>`.
pub const TIMELINE_BEGIN: Attr = Attr::new("timelineBegin");

/// `title`, on every HTML element; `<style>`.
pub const TITLE: Attr = Attr::new("title");

/// `to`, on `<animate>`, `<animateColor>`, `<animateMotion>`, `<animateTransform>`, `<set>`.
pub const TO: Attr = Attr::new("to");

/// `transform`, on every SVG element.
pub const TRANSFORM: Attr = Attr::new("transform");

/// `transformBehavior`, on `<video>`.
pub const TRANSFORM_BEHAVIOR: Attr = Attr::new("transformBehavior");

/// `translate`, on every HTML element.
pub const TRANSLATE: Attr = Attr::new("translate");

/// `type`, on 14 HTML elements; 14 SVG elements.
pub const TYPE: Attr = Attr::new("type");

/// `typemustmatch`, on `<object>`.
pub const TYPEMUSTMATCH: Attr = Attr::new("typemustmatch");

/// `typeof`, on every SVG element.
pub const TYPEOF: Attr = Attr::new("typeof");

/// `u1`, on `<hkern>`, `<vkern>`.
pub const U1: Attr = Attr::new("u1");

/// `u2`, on `<hkern>`, `<vkern>`.
pub const U2: Attr = Attr::new("u2");

/// `underline-position`, on `<font-face>`.
pub const UNDERLINE_POSITION: Attr = Attr::new("underline-position");

/// `underline-thickness`, on `<font-face>`.
pub const UNDERLINE_THICKNESS: Attr = Attr::new("underline-thickness");

/// `unicode`, on `<glyph>`.
pub const UNICODE: Attr = Attr::new("unicode");

/// `unicode-bidi`, on every SVG element.
pub const UNICODE_BIDI: Attr = Attr::new("unicode-bidi");

/// `unicode-range`, on `<font-face>`.
pub const UNICODE_RANGE: Attr = Attr::new("unicode-range");

/// `units-per-em`, on `<font-face>`.
pub const UNITS_PER_EM: Attr = Attr::new("units-per-em");

/// `usemap`, on `<img>`, `<input>`, `<object>`.
pub const USEMAP: Attr = Attr::new("usemap");

/// `valign`, on 8 HTML elements.
pub const VALIGN: Attr = Attr::new("valign");

/// `value`, on 8 HTML elements.
pub const VALUE: Attr = Attr::new("value");

/// `values`, on `<animate>`, `<animateColor>`, `<animateMotion>`, `<animateTransform>`, `<feColorMatrix>`.
pub const VALUES: Attr = Attr::new("values");

/// `valuetype`, on `<param>`.
pub const VALUETYPE: Attr = Attr::new("valuetype");

/// `vector-effect`, on every SVG element.
pub const VECTOR_EFFECT: Attr = Attr::new("vector-effect");

/// `version`, on `<html>`; `<svg>`.
pub const VERSION: Attr = Attr::new("version");

/// `vert-adv-y`, on `<font>`, `<glyph>`, `<missing-glyph>`.
pub const VERT_ADV_Y: Attr = Attr::new("vert-adv-y");

/// `vert-origin-x`, on `<font>`, `<glyph>`, `<missing-glyph>`.
pub const VERT_ORIGIN_X: Attr = Attr::new("vert-origin-x");

/// `vert-origin-y`, on `<font>`, `<glyph>`, `<missing-glyph>`.
pub const VERT_ORIGIN_Y: Attr = Attr::new("vert-origin-y");

/// `viewBox`, on `<marker>`, `<pattern>`, `<svg>`, `<symbol>`, `<view>`.
pub const VIEW_BOX: Attr = Attr::new("viewBox");

/// `viewTarget`, on `<view>`.
pub const VIEW_TARGET: Attr = Attr::new("viewTarget");

/// `visibility`, on every SVG element.
pub const VISIBILITY: Attr = Attr::new("visibility");

/// `vlink`, on `<body>`.
pub const VLINK: Attr = Attr::new("vlink");

/// `vspace`, on `<applet>`, `<img>`, `<object>`.
pub const VSPACE: Attr = Attr::new("vspace");

/// `v-alphabetic`, on `<font-face>`.
pub const V_ALPHABETIC: Attr = Attr::new("v-alphabetic");

/// `v-hanging`, on `<font-face>`.
pub const V_HANGING: Attr = Attr::new("v-hanging");

/// `v-ideographic`, on `<font-face>`.
pub const V_IDEOGRAPHIC: Attr = Attr::new("v-ideographic");

/// `v-mathematical`, on `<font-face>`.
pub const V_MATHEMATICAL: Attr = Attr::new("v-mathematical");

/// `white-space`, on every SVG element.
pub const WHITE_SPACE: Attr = Attr::new("white-space");

/// `width`, on 16 HTML elements; 29 SVG elements.
pub const WIDTH: Attr = Attr::new("width");

/// `widths`, on `<font-face>`.
pub const WIDTHS: Attr = Attr::new("widths");

/// `word-spacing`, on every SVG element.
pub const WORD_SPACING: Attr = Attr::new("word-spacing");

/// `wrap`, on `<textarea>`.
pub const WRAP: Attr = Attr::new("wrap");

/// `writingsuggestions`, on every HTML element.
pub const WRITINGSUGGESTIONS: Attr = Attr::new("writingsuggestions");

/// `writing-mode`, on every SVG element.
pub const WRITING_MODE: Attr = Attr::new("writing-mode");

/// `x`, on 37 SVG elements.
pub const X: Attr = Attr::new("x");

/// `x1`, on `<line>`, `<linearGradient>`.
pub const X1: Attr = Attr::new("x1");

/// `x2`, on `<line>`, `<linearGradient>`.
pub const X2: Attr = Attr::new("x2");

/// `xChannelSelector`, on `<feDisplacementMap>`.
pub const X_CHANNEL_SELECTOR: Attr = Attr::new("xChannelSelector");

/// `x-height`, on `<font-face>`.
pub const X_HEIGHT: Attr = Attr::new("x-height");

/// `y`, on 37 SVG elements.
pub const Y: Attr = Attr::new("y");

/// `y1`, on `<line>`, `<linearGradient>`.
pub const Y1: Attr = Attr::new("y1");

/// `y2`, on `<line>`, `<linearGradient>`.
pub const Y2: Attr = Attr::new("y2");

/// `yChannelSelector`, on `<feDisplacementMap>`.
pub const Y_CHANNEL_SELECTOR: Attr = Attr::new("yChannelSelector");

/// `z`, on `<fePointLight>`, `<feSpotLight>`.
pub const Z: Attr = Attr::new("z");

/// `zoomAndPan`, on `<svg>`, `<view>`.
pub const ZOOM_AND_PAN: Attr = Attr::new("zoomAndPan");
