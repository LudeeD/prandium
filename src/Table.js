import React, { useState, useEffect } from "react";

import Card from "react-bootstrap/Card";

//export default function Table({ results, demo }) {
//    return (
//        {
//          // values is an array of arrays representing the results of the query
//          demo.map((i) => (
//            <tr key={i}>
//              {row.map((value, i) => (
//                <td key={i}>{value}</td>
//              ))}
//            </tr>
//          ))
//        }
//        [1,2,3].map((v) => {
//                <Card>
//                    <Card.Body>This is some text within a card body. {v}</Card.Body>
//                </Card>
//        })
//    )
//}
//
//    const demo = [1, 2, 3]

/**
 * Renders a single value of the array returned by db.exec(...) as a table
 * @param {import("sql.js").QueryExecResult} props
 */
export default function Table({ columns, values }) {
  return (
      values.map( (value, index) => (
        <Card>
            <Card.Body>{index} - > {value}</Card.Body>
        </Card>
      ))
 );
}