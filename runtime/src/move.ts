/**
 * Puts `element` before `before` in `parent`, as `insertBefore` does.
 *
 * `insertBefore` takes an element in the document out before it puts it back,
 * which blurs a focused element inside it and drops the selection of a text
 * field. `moveBefore` keeps both, and where the browser lacks it the focused
 * element is focused again with its selection afterwards.
 */
export function move(parent: Node, element: Element, before: Node | null): void {
  if (
    parent instanceof Element &&
    "moveBefore" in parent &&
    parent.isConnected &&
    element.isConnected
  ) {
    parent.moveBefore(element, before);
    return;
  }
  const focused = document.activeElement;
  if (!(focused instanceof HTMLElement) || !element.contains(focused)) {
    parent.insertBefore(element, before);
    return;
  }
  const field =
    focused instanceof HTMLInputElement || focused instanceof HTMLTextAreaElement ? focused : null;
  // An input whose type has no text selection, such as a checkbox, reads `null`.
  const start = field?.selectionStart ?? null;
  const end = field?.selectionEnd ?? null;
  const direction = field?.selectionDirection ?? undefined;
  parent.insertBefore(element, before);
  if (document.activeElement === focused) return;
  focused.focus({ preventScroll: true });
  if (field !== null && start !== null && end !== null)
    field.setSelectionRange(start, end, direction);
}
