export default defineEventHandler((event) => {
  resetTodos();
  setResponseStatus(event, 204);
  return null;
});
