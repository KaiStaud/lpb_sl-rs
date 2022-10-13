CREATE TABLE IF NOT EXISTS nodes
(
    id          INTEGER PRIMARY KEY NOT NULL,
    alias TEXT                NOT NULL,
    vectors TEXT NOT NULL,
    rotations TEXT NOT NULL,
    following_node INTEGER NOT NULL,
    done        BOOLEAN             NOT NULL DEFAULT 0
);