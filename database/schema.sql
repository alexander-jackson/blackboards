DROP TABLE IF EXISTS sessions;
CREATE TABLE sessions (
	id INTEGER PRIMARY KEY,
	title TEXT,
	start_time INTEGER,
	remaining INTEGER
);

/* DROP TABLE IF EXISTS verified_emails; */
CREATE TABLE verified_emails (
	warwick_id INTEGER PRIMARY KEY,
	name TEXT
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
	warwick_id INTEGER,
	name TEXT NOT NULL,
	PRIMARY KEY (session_id, warwick_id)
);
