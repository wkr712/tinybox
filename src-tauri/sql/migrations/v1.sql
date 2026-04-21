-- Sticky Notes
CREATE TABLE IF NOT EXISTS notes (
    id          INTEGER PRIMARY KEY AUTOINCREMENT,
    title       TEXT NOT NULL DEFAULT '',
    content     TEXT NOT NULL DEFAULT '',
    color       TEXT NOT NULL DEFAULT '#1a1a2e',
    pinned      INTEGER NOT NULL DEFAULT 0,
    sort_order  INTEGER NOT NULL DEFAULT 0,
    created_at  TEXT NOT NULL DEFAULT (datetime('now', 'localtime')),
    updated_at  TEXT NOT NULL DEFAULT (datetime('now', 'localtime'))
);

-- Todo Items
CREATE TABLE IF NOT EXISTS todos (
    id          INTEGER PRIMARY KEY AUTOINCREMENT,
    text        TEXT NOT NULL,
    completed   INTEGER NOT NULL DEFAULT 0,
    priority    TEXT NOT NULL DEFAULT 'normal',
    due_date    TEXT,
    sort_order  INTEGER NOT NULL DEFAULT 0,
    created_at  TEXT NOT NULL DEFAULT (datetime('now', 'localtime')),
    updated_at  TEXT NOT NULL DEFAULT (datetime('now', 'localtime'))
);

-- Pomodoro Sessions
CREATE TABLE IF NOT EXISTS pomodoro_sessions (
    id          INTEGER PRIMARY KEY AUTOINCREMENT,
    duration    INTEGER NOT NULL DEFAULT 25,
    type        TEXT NOT NULL DEFAULT 'work',
    completed   INTEGER NOT NULL DEFAULT 0,
    started_at  TEXT,
    ended_at    TEXT,
    created_at  TEXT NOT NULL DEFAULT (datetime('now', 'localtime'))
);

-- Clipboard History
CREATE TABLE IF NOT EXISTS clipboard_history (
    id           INTEGER PRIMARY KEY AUTOINCREMENT,
    content      TEXT NOT NULL,
    content_type TEXT NOT NULL DEFAULT 'text',
    source_app   TEXT,
    is_favorite  INTEGER NOT NULL DEFAULT 0,
    created_at   TEXT NOT NULL DEFAULT (datetime('now', 'localtime'))
);

-- File Drop Zone
CREATE TABLE IF NOT EXISTS dropzone_files (
    id              INTEGER PRIMARY KEY AUTOINCREMENT,
    file_name       TEXT NOT NULL,
    file_path       TEXT NOT NULL,
    file_size       INTEGER NOT NULL DEFAULT 0,
    mime_type       TEXT,
    thumbnail_path  TEXT,
    tags            TEXT,
    created_at      TEXT NOT NULL DEFAULT (datetime('now', 'localtime'))
);

-- NetEase Cloud Music: Cached Playlists
CREATE TABLE IF NOT EXISTS ncm_playlists (
    id          INTEGER PRIMARY KEY,
    name        TEXT NOT NULL,
    cover_url   TEXT,
    track_count INTEGER NOT NULL DEFAULT 0,
    creator     TEXT,
    description TEXT,
    cached_at   TEXT NOT NULL DEFAULT (datetime('now', 'localtime'))
);

-- NetEase Cloud Music: Cached Tracks
CREATE TABLE IF NOT EXISTS ncm_tracks (
    id              INTEGER PRIMARY KEY,
    name            TEXT NOT NULL,
    artists         TEXT NOT NULL,
    album_name      TEXT,
    album_cover_url TEXT,
    duration_ms     INTEGER,
    lyric_data      TEXT,
    tlyric_data     TEXT,
    cached_at       TEXT NOT NULL DEFAULT (datetime('now', 'localtime'))
);

-- App Settings
CREATE TABLE IF NOT EXISTS settings (
    key         TEXT PRIMARY KEY,
    value       TEXT NOT NULL,
    updated_at  TEXT NOT NULL DEFAULT (datetime('now', 'localtime'))
);

-- Default settings
INSERT OR IGNORE INTO settings (key, value) VALUES
    ('sidebar_edge', 'left'),
    ('always_on_top', 'true'),
    ('clipboard_max_history', '100'),
    ('clipboard_monitor_enabled', 'true'),
    ('pomodoro_work_duration', '25'),
    ('pomodoro_break_duration', '5'),
    ('music_volume', '80'),
    ('hotkey_toggle_sidebar', 'Alt+Space');

-- Indexes
CREATE INDEX IF NOT EXISTS idx_notes_sort ON notes(sort_order);
CREATE INDEX IF NOT EXISTS idx_todos_completed ON todos(completed, sort_order);
CREATE INDEX IF NOT EXISTS idx_clipboard_created ON clipboard_history(created_at DESC);
CREATE INDEX IF NOT EXISTS idx_dropzone_created ON dropzone_files(created_at DESC);
