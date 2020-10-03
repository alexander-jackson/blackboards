database:
	sqlite3 database/data.sqlite < database/schema.sql
	sqlite3 database/data.sqlite < database/sessions.sql
