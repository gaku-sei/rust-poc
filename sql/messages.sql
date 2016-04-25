CREATE TABLE messages (
    id      INTEGER PRIMARY KEY,
    -- author_id  INTEGER,
    room_id INTEGER,
    content TEXT NOT null,
    FOREIGN KEY(room_id) REFERENCES rooms(id)
    -- FOREIGN KEY(user_id) REFERENCES users(id)
);
