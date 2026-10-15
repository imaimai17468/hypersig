export default defineEventHandler(async (event) => {
  const id = Number(getRouterParam(event, "id"));
  const { done } = await readBody<{ done: boolean }>(event);
  setTodoDone(id, done === true);
  setResponseStatus(event, 204);
  return null;
});
