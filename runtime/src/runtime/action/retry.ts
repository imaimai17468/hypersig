/** The longest wait before a call sent with `reconnect` is sent again, doubling after each failure up to the last. */
export const FIRST_RETRY_MS = 1000;
export const LAST_RETRY_MS = 30_000;

/**
 * Waits between half of `ms` and `ms`, so pages that lost one server at once
 * do not all ask it again at the same moment. The wait ends early when the
 * browser comes back online or the page becomes visible, where a retry has
 * the best chance to work.
 */
export function backoff(ms: number): Promise<void> {
  return new Promise((done) => {
    const finish = (): void => {
      clearTimeout(timer);
      removeEventListener("online", finish);
      document.removeEventListener("visibilitychange", shown);
      done();
    };
    const shown = (): void => {
      if (document.visibilityState === "visible") finish();
    };
    const timer = setTimeout(finish, ms / 2 + (Math.random() * ms) / 2);
    addEventListener("online", finish);
    document.addEventListener("visibilitychange", shown);
  });
}
