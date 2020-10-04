DROP TABLE IF EXISTS sessions;
CREATE TABLE sessions (
	id INTEGER PRIMARY KEY,
	title TEXT,
	start_time INTEGER,
	remaining INTEGER
);

DROP TABLE IF EXISTS requests;
CREATE TABLE requests (
	session_id INTEGER,
	warwick_id INTEGER PRIMARY KEY,
	name TEXT NOT NULL,
	identifier INTEGER NOT NULL
);

DROP TABLE IF EXISTS registrations;
CREATE TABLE registrations (
	session_id INTEGER,
	warwick_id INTEGER PRIMARY KEY,
	name TEXT NOT NULL
);
