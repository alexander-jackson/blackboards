-- Removes the need to track the number of remaining sessions

-- Create a new column for the number of spaces
ALTER TABLE sessions
ADD COLUMN spaces INTEGER;

-- Populate its values based on the current registrations
UPDATE sessions
SET spaces = remaining + (
	SELECT COUNT(*)
	FROM registrations
	WHERE registrations.session_id = id
);

-- Update the column to be non-null
ALTER TABLE sessions
ALTER COLUMN spaces SET NOT NULL;

-- Delete the remaining column
ALTER TABLE sessions
DROP COLUMN remaining;
