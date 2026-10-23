SELECT id AS "id: TodoId", title, done
FROM todos
WHERE id = $1
