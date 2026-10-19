/**
 * Adds the atomic CSS rules an element patch brings.
 *
 * The page writes each bucket of rules as `<style data-hs-css="sort key">`, in
 * sort-key order, because rules of equal specificity apply in source order:
 * a breakpoint rule has to come after the base rule it overrides. A bucket
 * from a patch goes into the `<style>` with its key, or into a new one placed
 * before the first `<style>` whose key sorts after it. `crates/hypersig_style`
 * defines the keys so that comparing them as strings orders the buckets.
 */
export function addRules(
  buckets: readonly (readonly [key: string, css: string])[],
  head: HTMLHeadElement = document.head,
): void {
  for (const [key, css] of buckets) {
    const styles = [...head.querySelectorAll<HTMLStyleElement>("style[data-hs-css]")];
    const same = styles.find((style) => style.dataset.hsCss === key);
    if (same !== undefined) {
      // Rules repeated inside one bucket change nothing, so a bucket the page
      // already has in full is skipped and a partial overlap is appended.
      if (!(same.textContent ?? "").includes(css)) same.append(css);
      continue;
    }
    const style = document.createElement("style");
    style.dataset.hsCss = key;
    const nonce = pageNonce();
    if (nonce !== "") style.setAttribute("nonce", nonce);
    style.textContent = css;
    const after = styles.find((existing) => (existing.dataset.hsCss ?? "") > key);
    if (after !== undefined) after.before(style);
    else if (styles.length > 0) styles.at(-1)!.after(style);
    else head.prepend(style);
  }
}

/**
 * The nonce `Page::csp_nonce` put on the runtime's `<script>`, or `""`.
 *
 * A browser that enforces a Content-Security-Policy hides the attribute's
 * value and keeps it in the `nonce` property, so the property is read first.
 */
function pageNonce(): string {
  const script = document.querySelector<HTMLScriptElement>("script[nonce]");
  if (script === null) return "";
  return script.nonce || (script.getAttribute("nonce") ?? "");
}
