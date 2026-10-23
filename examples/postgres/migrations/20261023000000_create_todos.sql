CREATE TABLE todos (
    id BIGINT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    title TEXT NOT NULL,
    done BOOLEAN NOT NULL DEFAULT FALSE
);

-- Each statement that changes the table notifies `todos_changed`, which every
-- server process listens on. PostgreSQL delivers the notification when the
-- transaction commits, so a listener reads the list after the change.
CREATE FUNCTION notify_todos_changed() RETURNS trigger AS $$
BEGIN
    PERFORM pg_notify('todos_changed', '');
    RETURN NULL;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER todos_changed
    AFTER INSERT OR UPDATE OR DELETE ON todos
    FOR EACH STATEMENT EXECUTE FUNCTION notify_todos_changed();
