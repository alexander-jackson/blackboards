DROP TABLE IF EXISTS sessions;
CREATE TABLE sessions (
	id INTEGER PRIMARY KEY,
	title TEXT,
	start_time INTEGER,
	remaining INTEGER
);

DROP TABLE IF EXISTS registrations;
CREATE TABLE registrations (
	session_id INTEGER,
	warwick_id INTEGER,
	name TEXT NOT NULL,
	PRIMARY KEY (session_id, warwick_id)
);

DROP TABLE IF EXISTS attendances;
CREATE TABLE attendances (
	session_id INTEGER,
	warwick_id INTEGER,
	PRIMARY KEY (session_id, warwick_id)
);

DROP TABLE IF EXISTS auth_pairs;
CREATE TABLE auth_pairs (
	token TEXT PRIMARY KEY,
	secret TEXT
);
