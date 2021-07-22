import React, { useState, useEffect } from "react";
import "./styles.css";
import initSqlJs from "sql.js";
import 'bootstrap/dist/css/bootstrap.css';

// Required to let webpack 4 know it needs to copy the wasm file to our assets
import sqlWasm from "!!file-loader?name=sql-wasm-[contenthash].wasm!sql.js/dist/sql-wasm.wasm";

export default function App() {
  const [db, setDb] = useState(null);
  const [error, setError] = useState(null);

  useEffect(async () => {
    // sql.js needs to fetch its wasm file, so we cannot immediately instantiate the database
    // without any configuration, initSqlJs will fetch the wasm files directly from the same path as the js
    // see ../craco.config.js
    try {
      const sqlPromise = initSqlJs({ locateFile: () => sqlWasm });
      const dataPromise = fetch("/prandium/index.db").then(res => res.arrayBuffer());
      const [SQL, buf] = await Promise.all([sqlPromise, dataPromise])
      const db = new SQL.Database(new Uint8Array(buf));
      setDb(db);
    } catch (err) {
      setError(err);
    }
  }, []);

  if (error) return <pre>{error.toString()}</pre>;
  else if (!db) return <pre>Loading...</pre>;
  else return <MainRecipes db={db} />;
}

import Container from "react-bootstrap/Container";
import Button from "react-bootstrap/Button";
import Row from "react-bootstrap/Row";
import Col from "react-bootstrap/Col";

import Table from './Table.js'
import Input from './Input.js'

function MainRecipes({ db }) {
  const [error, setError] = useState(null);
  const [results, setResults] = useState([]);

  function get_recipes(where) {
    try {
      let query = `SELECT DISTINCT
        recipes_info.title, recipes_info.path
        FROM recipes_info
        INNER JOIN recipes_ingredients ON recipes_ingredients.recipe_id = recipes_info.recipe_id
        INNER JOIN ingredients ON ingredients.ingredient_id = recipes_ingredients.ingredient_id
        INNER JOIN recipes_tags ON recipes_tags.recipe_id = recipes_info.recipe_id
        INNER JOIN tags ON tags.tag_id = recipes_tags.tag_id
        `
      if (where) {
        query += where
      }
      console.log(query)
      let results = db.exec(query)
      console.log(results)
      setResults(results[0]['values'])
      setError(null)
    } catch (err) {
      setError(err)
      setResults([])
    }
  }

  const inputUpdated = ( inputs ) => {

    let query = ""

    if (inputs.ingredients[0] != "" && inputs.ingredients.length > 0) {
        query += "WHERE "
        query += inputs.ingredients.map(x => " ingredients.name = '"+x+"'").join(" OR ")
    }

    if (inputs.tags[0] != "" && inputs.tags.length > 0) {
        if (query.length > 0) {query += " AND "}
        else {query += "WHERE "}
        query += inputs.tags.map(x => " tags.name = '"+x+"'").join(" OR ")
        console.log(query)
    }

    query+= "ORDER BY RANDOM() " + "LIMIT " + inputs.number

    console.log(inputs.name)
    console.log(inputs.ingredients)
    console.log(inputs.tags)
    console.log(inputs.number)

    get_recipes(query)

  }

  return (
    <Container >
      <br></br>
      <Col><h1>Recipes for everyone 🍲</h1></Col>
      <pre>Recipe planner, generator, finder... everything recipe related. </pre> 
      <Input updatecb={inputUpdated}></Input>
      <pre className="error">{(error || "").toString()}</pre>
      
      <Table values={results}></Table>

    </Container>
  );
}

/**
 * A simple SQL read-eval-print-loop
 * @param {{db: import("sql.js").Database}} props
 */
function SQLRepl({ db }) {
  const [error, setError] = useState(null);
  const [results, setResults] = useState([]);

  function exec(sql) {
    try {
      // The sql is executed synchronously on the UI thread.
      // You may want to use a web worker here instead
      setResults(db.exec(sql)); // an array of objects is returned
      setError(null);
    } catch (err) {
      // exec throws an error when the SQL statement is invalid
      setError(err);
      setResults([]);
    }
  }

  return (
    <div className="App" >
      <h1>React SQL interpreter</h1>

      <textarea
        onChange={(e) => exec(e.target.value)}
        placeholder="Enter some SQL. No inspiration ? Try “select sqlite_version()”"
      ></textarea>

      <pre className="error">{(error || "").toString()}</pre>

      <pre>
        {
          // results contains one object per select statement in the query
          results.map(({ columns, values }, i) => (
            <ResultsTable key={i} columns={columns} values={values} />
          ))
        }
      </pre>
    </div>
  );
}

/**
 * Renders a single value of the array returned by db.exec(...) as a table
 * @param {import("sql.js").QueryExecResult} props
 */
function ResultsTable({ columns, values }) {
  return (
    <table>
      <thead>
        <tr>
          {columns.map((columnName, i) => (
            <td key={i}>{columnName}</td>
          ))}
        </tr>
      </thead>

      <tbody>
        {
          // values is an array of arrays representing the results of the query
          values.map((row, i) => (
            <tr key={i}>
              {row.map((value, i) => (
                <td key={i}>{value}</td>
              ))}
            </tr>
          ))
        }
      </tbody>
    </table>
  );
}
