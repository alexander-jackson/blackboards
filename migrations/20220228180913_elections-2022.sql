-- Drop all the data from the previous year's tables
TRUNCATE exec_positions, candidates, nominations, votes;

-- Add all the positions with unique IDs
-- All positions should begin closed, they can be toggled by the election admins
INSERT INTO exec_positions
(id, title, num_winners, open)
VALUES
	(135543, 'President', 1, false),
	(291946, 'Secretary & Welfare Officer', 1, false),
	(293061, 'Treasurer', 1, false),
	(394225, 'Social Secretary', 2, false),
	(413863, 'Publicity Officer', 1, false),
	(468901, 'Powerlifting Captain', 1, false),
	(508245, 'Weightlifting Captain', 1, false),
	(977896, 'Women''s & Campaigns Officer', 1, false);

-- Add all the candidates
-- These should also all begin as 'unelected' and will be elected when voting closes
INSERT INTO candidates
(warwick_id, name, elected)
VALUES
	(1904838, 'Todor Karadimov', false),
	(2147553, 'Maddelaena Grace Porter', false),
	(2038959, 'Poojan Chotalia', false),
	(2000244, 'Matthew Tam', false),
	(2102831, 'Simran Thakrar', false),
	(2109024, 'Ben Biesinger', false),
	(1835625, 'Tahira Choudhury', false),
	(1906960, 'Shubhdeep Sethi', false),
	(2136210, 'Simon Sun', false),
	(2004733, 'Blake Herdman', false),
	(2153016, 'Mason Mui', false),
	(2008089, 'Keith Hoan Zhan Rong', false),
	(2046197, 'Emily Ratcliffe', false);

-- Map all the candidates to the positions they applied for
INSERT INTO nominations
(position_id, warwick_id)
VALUES
	-- President
	(135543, 1904838),

	-- Secretary & Welfare Officer
	(291946, 2147553),

	-- Treasurer
	(293061, 2038959),

	-- Social Secretary
	(394225, 2000244),
	(394225, 2102831),
	(394225, 2109024),
	(394225, 1835625),
	(394225, 1906960),

	-- Publicity Officer
	(413863, 2136210),
	(413863, 2038959),
	(413863, 1835625),

	-- Powerlifting Captain
	(468901, 2004733),
	(468901, 2153016),

	-- Weightlifting Captain
	(508245, 2008089),

	-- Women's & Campaigns Officer
	(977896, 2046197);
