CREATE TABLE sessions (
	id INTEGER PRIMARY KEY,
	title TEXT,
	start_time BIGINT,
	remaining INTEGER
);

CREATE TABLE registrations (
	session_id INTEGER,
	warwick_id INTEGER,
	name TEXT NOT NULL,
	PRIMARY KEY (session_id, warwick_id),
	CONSTRAINT fk_sessions
	FOREIGN KEY(session_id)
	REFERENCES sessions(id)
	ON DELETE CASCADE
);

CREATE TABLE attendances (
	session_id INTEGER,
	warwick_id INTEGER,
	PRIMARY KEY (session_id, warwick_id),
	CONSTRAINT fk_sessions
	FOREIGN KEY(session_id)
	REFERENCES sessions(id)
	ON DELETE CASCADE
);

CREATE TABLE auth_pairs (
	token TEXT PRIMARY KEY,
	secret TEXT
);

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

CREATE TABLE exec_positions (
	id INTEGER PRIMARY KEY,
	title TEXT NOT NULL,
	num_winners INTEGER NOT NULL,
	open BOOLEAN NOT NULL
);

CREATE TABLE candidates (
	warwick_id INTEGER PRIMARY KEY,
	name TEXT NOT NULL,
	elected BOOLEAN NOT NULL
);

CREATE TABLE nominations (
	position_id INTEGER NOT NULL,
	warwick_id INTEGER NOT NULL,
	PRIMARY KEY (position_id, warwick_id)
);

CREATE TABLE votes (
	warwick_id INTEGER NOT NULL,
	position_id INTEGER NOT NULL,
	candidate_id INTEGER NOT NULL,
	ranking INTEGER NOT NULL,
	PRIMARY KEY (warwick_id, position_id, candidate_id)
);
