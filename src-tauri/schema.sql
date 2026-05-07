CREATE TABLE pomodoro_timer_state (
                id INTEGER PRIMARY KEY CHECK (id = 1),
                mode TEXT NOT NULL,
                status TEXT NOT NULL,
                remaining_seconds INTEGER NOT NULL,
                updated_at_ms INTEGER NOT NULL
            );

CREATE TABLE todos (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                title TEXT NOT NULL,
                completed INTEGER NOT NULL DEFAULT 0
            );

CREATE TABLE work_logs (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                body TEXT NOT NULL,
                created_at_ms INTEGER NOT NULL
            );
