-- Your SQL goes here

CREATE TABLE jusrisdictions (
	id INTEGER PRIMARY KEY,
	name VARCHAR NOT NULL,
	source VARCHAR NOT NULL,
	public_link VARCHAR NOT NULL
);


INSERT INTO jusrisdictions(id, name, source, public_link) VALUES (1, 'Clackamas County', "https://www.clackamas.us/coronavirus", "https://www.clackamas.us/coronavirus" ), (2, 'Multnomah County', "multco.us/covid-vaccine", "multco.us/covid-vaccine");