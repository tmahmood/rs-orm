CREATE TABLE IF NOT EXISTS surveys (
	id SERIAL PRIMARY KEY,
	title character varying NOT NULL unique,
	duration integer NOT NULL,
	active boolean,
	start_date bigint
);

CREATE TABLE IF NOT EXISTS questionsets (
	id SERIAL PRIMARY KEY,
	title VARCHAR NOT NULL
);

CREATE TABLE IF NOT EXISTS questions (
	id SERIAL PRIMARY KEY,
	title VARCHAR NOT NULL
);

CREATE TABLE IF NOT EXISTS optionsets (
	id SERIAL PRIMARY KEY,
	label VARCHAR NOT NULL
);

CREATE TABLE IF NOT EXISTS options (
	id SERIAL PRIMARY KEY,
	label VARCHAR NOT NULL unique
);

CREATE TABLE IF NOT EXISTS questionset_questions (
	id SERIAL PRIMARY KEY,
	questionset_id integer references questionsets on delete cascade not null,
	question_id integer references questions on delete cascade not null,
	unique(question_id, questionset_id)
);

CREATE TABLE IF NOT EXISTS optionset_options (
	id SERIAL PRIMARY KEY,
	optionset_id integer references optionsets on delete cascade not null,
	option_id integer references options on delete cascade not null,
	unique(optionset_id, option_id)
);

CREATE TABLE IF NOT EXISTS question_optionsets (
	id SERIAL PRIMARY KEY,
	question_id integer references questions on delete cascade not null,
	optionset_id integer references optionsets on delete cascade not null,
	unique(question_id, optionset_id)
);

