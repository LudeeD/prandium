import sqlite3

con = sqlite3.connect('./public/index.db')

import os

def recipe_already_indexed(name):
    cur = con.cursor()
    cur.execute("SELECT * FROM recipes_info WHERE path=?", (name,))
    rows = cur.fetchall()
    for r in rows: print(r)
    return len(rows) > 0

def add_tag(name, recipe_id):
    print(name)
    cur = con.cursor()
    cur.execute("SELECT tag_id FROM tags WHERE name=?", (name,) )
    rows = cur.fetchall()
    id = 0
    if len(rows) == 0:  
        sql = '''INSERT INTO tags(name) VALUES(?)'''
        cur.execute(sql, (name,))
        con.commit()
        id = cur.lastrowid
    else:
        id = rows[0][0]
    
    sql = '''INSERT INTO recipes_tags(recipe_id, tag_id) VALUES(?,?)'''
    print(f"recipe id{recipe_id} id{id}")
    cur.execute(sql, (recipe_id,id))
    con.commit()
    
    return rows

def add_ingredient(name, quantity, recipe_id):
    print(name)
    cur = con.cursor()
    cur.execute("SELECT ingredient_id FROM ingredients WHERE name=?", (name,) )
    rows = cur.fetchall()
    id = 0
    if len(rows) == 0:  
        sql = '''INSERT INTO ingredients(name) VALUES(?)'''
        cur.execute(sql, (name,))
        con.commit()
        id = cur.lastrowid
    else:
        id = rows[0][0]
    
    sql = '''INSERT INTO recipes_ingredients(recipe_id, ingredient_id, quantity) VALUES(?,?,?)'''
    cur.execute(sql, (recipe_id,id,quantity,))
    con.commit()
    
    return rows

def add_new_recipe(title, path):
    cur = con.cursor()
    sql = ''' INSERT INTO recipes_info(title, path)
              VALUES(?,?) '''
    cur.execute(sql, (title, path))
    con.commit()
    id = cur.lastrowid
    print(id)
    return id

def add_new_tag(recipe_id, name):
    cur = con.cursor()
    sql = ''' INSERT INTO recipes_tags(name, recipe_id)
              VALUES(?,?) '''
    cur.execute(sql, (name, recipe_id))
    con.commit()
    return id

def add_new_ingredient(recipe_id, info):
    cur = con.cursor()
    sql = ''' INSERT INTO recipes_ingredients(info, recipe_id)
              VALUES(?,?) '''
    cur.execute(sql, (info, recipe_id))
    con.commit()
    return id



def parse_recipe(folder, filename):
    if recipe_already_indexed(filename): return 
    section = 0
    title = ''
    section_title = True
    tags = []
    ingredients = []
    path = os.path.join(folder, filename)
    with open(path) as f:
        recipe_id = -1;
        for index, line in enumerate(f):
            print("Line {}: {}".format(index, line.strip()))
            if line == '\n':
                section += 1
                section_title = True
                continue
            if section == 0:
                # Title Section
                title = line[2:].strip()
                print(f"Adding new recipe {title} from {filename}")
                recipe_id = add_new_recipe(title, filename)
            elif section == 1:
                # Tags Section
                if section_title: section_title = False; continue
                tag = ''.join(ch for ch in line if ch.isalnum())
                print(f"Tag => {tag}")
                #add_new_tag(recipe_id, tag)
                add_tag(tag, recipe_id)
            elif section == 2:
                # Ingredients Section
                if section_title: section_title = False; continue
                # 0 > "-"
                # 1 > ammount
                # 2.. name
                ingredient = line.split(' ');
                ammount = ingredient[1]
                name = ' '.join(ingredient[2:]).strip()
                add_ingredient(name, ammount, recipe_id)
            elif section == 3:
                break

    print(tags)
    print(ingredients)

    

def parse(path):
    for filename in os.listdir(path):
        if filename.endswith(".md"): 
             # print()
            parse_recipe(path, filename)
            continue
        else:
            continue



def main():
    parse('./recipes')

if __name__ == "__main__":
    main()
