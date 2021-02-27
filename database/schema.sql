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

DROP TABLE IF EXISTS personal_bests;
CREATE TABLE personal_bests (
	warwick_id INTEGER PRIMARY KEY,
	name TEXT NOT NULL,
	squat REAL,
	bench REAL,
	deadlift REAL,
	snatch REAL,
	clean_and_jerk REAL,
	show_pl BOOLEAN,
	show_wl BOOLEAN
);

DROP TABLE IF EXISTS taskmaster_entries;
CREATE TABLE taskmaster_entries (
	name TEXT PRIMARY KEY,
	score INTEGER NOT NULL
);

DROP TABLE IF EXISTS exec_positions;
CREATE TABLE exec_positions (
	id INTEGER PRIMARY KEY,
	title TEXT NOT NULL
);

DROP TABLE IF EXISTS nominations;
CREATE TABLE nominations (
	position_id INTEGER NOT NULL,
	warwick_id INTEGER NOT NULL,
	PRIMARY KEY (position_id, warwick_id)
);
