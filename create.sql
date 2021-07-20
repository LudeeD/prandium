CREATE TABLE recipes_info (
	recipe_id       INTEGER PRIMARY KEY,
	title           TEXT NOT NULL,
	diet            TEXT NOT NULL,
);

CREATE TABLE recipes_instructions (
	instruction_id  INTEGER PRIMARY KEY,
    FOREIGN KEY(recipe_id) REFERENCES recipes_info(recipe_id)
	info            TEXT NOT NULL,
);

CREATE TABLE recipes_ingredients (
	instruction_id  INTEGER PRIMARY KEY,
    FOREIGN KEY(recipe_id) REFERENCES recipes_info(recipe_id)
	info            TEXT NOT NULL,
	amount          TEXT NOT NULL,
);

INSERT INTO recipes_info (title, diet)
VALUES('Luís', 'Silva', 'mail@mail.com', '+123456789');