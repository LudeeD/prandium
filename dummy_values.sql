SQLite version 3.33.0 2020-08-14 13:23:32
Enter ".help" for usage hints.
sqlite> INSERT into tags VALUES "meal";
Error: near ""meal"": syntax error
sqlite> INSERT into tags VALUES ("meal"); 
Error: table tags has 2 columns but 1 values were supplied
sqlite> INSERT INTO tags (name) VALUES ("meal"); 
sqlite> INSERT INTO tags (name) VALUES ("snack"); 
sqlite> INSERT INTO ingredients (name) VALUES ("chouriço"); 
sqlite> INSERT INTO ingredients (name) VALUES ("aveia");    
sqlite> INSERT INTO recipes_info (title, path) VALUES ("Favas com chouriço", "./favas.md"); 
sqlite> INSERT INTO recipes_info (title, path) VALUES ("Aveia com aveia", "./aveia.md");    
sqlite> INSERT INTO recipes_ingredients (recipe_id, ingredient_id) VALUES (3);       
Error: 1 values for 2 columns
sqlite> SELECT * from recipes_info;
1|Favas com chouriço|./favas.md
2|Aveia com aveia|./aveia.md
sqlite> INSERT INTO recipes_ingredients (recipe_id, ingredient_id) VALUES (1,1); 
Error: NOT NULL constraint failed: recipes_ingredients.quantity
sqlite> INSERT INTO recipes_ingredients (recipe_id, ingredient_id) VALUES (1,1,10);  
Error: 3 values for 2 columns
sqlite> INSERT INTO recipes_ingredients (recipe_id, ingredient_id, quantity) VALUES (1,1,10); 
sqlite> INSERT INTO recipes_ingredients (recipe_id, ingredient_id, quantity) VALUES (2,2,10);  
sqlite> INSERT INTO recipes_tags (recipe_id, tag_id, quantity) VALUES (1,1,10);               
sqlite> INSERT INTO recipes_tags (recipe_id, tag_id, quantity) VALUES (2,2,10);