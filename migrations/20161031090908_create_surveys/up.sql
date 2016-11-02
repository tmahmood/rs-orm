
CREATE TABLE IF NOT EXISTS surveys (
	id SERIAL PRIMARY KEY,
	title VARCHAR NOT NULL,
	duration INTEGER,
	active BOOLEAN NOT NULL DEFAULT 'f',
	start_date VARCHAR NOT NULL
);

create table if not exists phones (
	id serial primary key,
	phone varchar not null,
	UNIQUE(phone)
);

CREATE TABLE IF NOT EXISTS texts (
	id SERIAL PRIMARY KEY,
	status VARCHAR NOT NULL UNIQUE
);

create table if not exists phone_status (
	id serial primary key,
	phone_id INTEGER references phones,
	survey_id INTEGER references surveys,
	status_id INTEGER references status_texts,
	UNIQUE(phone_id, survey_id)
);

insert into texts (status) values ('Invalid'), ('Busy'), ('Received and Helping');

