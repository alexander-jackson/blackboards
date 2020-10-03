database: clean
	sqlite3 database/data.sqlite < database/sessions.sql

clean:
	sqlite3 database/data.sqlite < database/schema.sql
