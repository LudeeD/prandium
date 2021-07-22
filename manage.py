import sqlite3

con = sqlite3.connect('./public/index.db')

def setup_db():
    cur = con.cursor()
    # Create table
    cur.execute('''CREATE TABLE IF NOT EXISTS recipes_info
                   (recipe_id INTEGER PRIMARY KEY,
                   title TEXT NOT NULL,
                   path TEXT NOT NULL)''')
    con.commit()

    cur.execute('''CREATE TABLE IF NOT EXISTS ingredients
                   (ingredient_id INTEGER PRIMARY KEY, name TEXT NOT NULL)''')

    cur.execute('''CREATE TABLE IF NOT EXISTS tags
                   (tag_id INTEGER PRIMARY KEY, name TEXT NOT NULL)''')

    cur.execute('''CREATE TABLE IF NOT EXISTS recipes_ingredients
                   (recipes_ingredients_id INTEGER PRIMARY KEY,
                    recipe_id INTEGER NOT NULL, 
                    ingredient_id INTEGER NOT NULL, quantity TEXT NOT NULL, 
                    FOREIGN KEY(ingredient_id) REFERENCES ingredients(ingredient_id),
                    FOREIGN KEY(recipe_id) REFERENCES recipes_info(recipe_id))''')

    cur.execute('''CREATE TABLE IF NOT EXISTS recipes_tags
                   (recipes_tags INTEGER PRIMARY KEY,
                    recipe_id INTEGER NOT NULL, 
                    tag_id INTEGER NOT NULL,
                    FOREIGN KEY(tag_id) REFERENCES tags(tag_id),
                    FOREIGN KEY(recipe_id) REFERENCES recipes_info(recipe_id))''')

    # Save (commit) the changes
    con.commit()

def list_entries():
    cur = con.cursor()
    for x in cur.execute('''SELECT * from recipes_info;'''):
        print(x)
    for x in cur.execute('''SELECT * from recipes_tags;'''):
        print(x)
    for x in cur.execute('''SELECT * from recipes_ingredients;'''):
        print(x)

def reset():
    cur = con.cursor()
    # delete all rows from table
    cur.execute('DELETE FROM recipes_info;',);
    print('We have deleted', cur.rowcount, 'records from the table.')
    cur.execute('DELETE FROM recipes_tags;',);
    print('We have deleted', cur.rowcount, 'records from the table.')
    cur.execute('DELETE FROM recipes_ingredients;',);
    print('We have deleted', cur.rowcount, 'records from the table.')
    cur.execute('DELETE FROM ingredients;',);
    print('We have deleted', cur.rowcount, 'records from the table.')
    cur.execute('DELETE FROM tags;',);
    print('We have deleted', cur.rowcount, 'records from the table.')
    #commit the changes to db			
    con.commit()

import sys
def main():

    op = sys.argv[1]

    if op == 'setup':
        print("Setup DB...")
        setup_db()
    elif op == 'list':
        list_entries()
    elif op == 'delete':
        reset()






if __name__ == "__main__":
    main()
