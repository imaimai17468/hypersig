/** Waits for the tasks queued so far, after which the runtime has rendered the changes made before. */
export function tick(): Promise<void> {
  return new Promise<void>((resolve) => {
    setTimeout(resolve, 0);
  });
}

/** Waits past the next animation frame, after which the runtime has forgotten the elements a patch removed. */
export function nextFrame(): Promise<void> {
  return new Promise<void>((resolve) => {
    requestAnimationFrame(() => {
      setTimeout(resolve, 0);
    });
  });
}

export const $ = <E extends Element = HTMLElement>(selector: string) =>
  document.querySelector<E>(selector)!;

export const $$ = (selector: string) => [...document.querySelectorAll<HTMLElement>(selector)];

export function type(
  input: HTMLInputElement | HTMLSelectElement,
  value: string,
  event = "input",
): void {
  input.value = value;
  input.dispatchEvent(new Event(event, { bubbles: true }));
}

export function key(target: Element, name: string): void {
  target.dispatchEvent(
    new KeyboardEvent("keydown", { key: name, bubbles: true, cancelable: true }),
  );
}
