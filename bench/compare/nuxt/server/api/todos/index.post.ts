export default defineEventHandler(async (event) => {
  const { id, title } = await readBody<{ id: number; title: string }>(event);
  const trimmed = String(title ?? "").trim();
  if (!Number.isInteger(id) || trimmed === "") throw createError({ statusCode: 400 });
  addTodo({ id, title: trimmed, done: false });
  setResponseStatus(event, 204);
  return null;
});
