export default defineEventHandler((event) => {
  removeTodo(Number(getRouterParam(event, "id")));
  setResponseStatus(event, 204);
  return null;
});
