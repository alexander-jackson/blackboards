-- Add all the RON candidates
INSERT INTO candidates (warwick_id, name, elected)
VALUES
	(1700001, 'RON', false),
	(1700002, 'RON', false),
	(1700003, 'RON', false),
	(1700004, 'RON', false),
	(1700005, 'RON', false),
	(1700006, 'RON', false),
	(1700007, 'RON', false),
	(1700008, 'RON', false),
	(1700009, 'RON', false);

-- Nominate them for their positions
INSERT INTO nominations (position_id, warwick_id)
VALUES
	-- President
	(135543, 1700001),

	-- Secretary & Welfare Officer
	(291946, 1700002),

	-- Treasurer
	(293061, 1700003),

	-- Social Secretary
	(394225, 1700004),
	(394225, 1700005),

	-- Publicity Officer
	(413863, 1700006),

	-- Powerlifting Captain
	(468901, 1700007),

	-- Weightlifting Captain
	(508245, 1700008),

	-- Women's & Campaigns Officer
	(977896, 1700009);
