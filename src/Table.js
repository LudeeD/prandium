import React, { useState, useEffect } from "react";

import Card from "react-bootstrap/Card";
import Row from "react-bootstrap/Row";
import Col from "react-bootstrap/Col";
import { Container } from "react-bootstrap";

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
export default function Table({ values }) {
  console.log(values)
  return (

    values.map((value, index) => (
    <Container>
      <Row className="justify-content-md-center">
        <Col></Col>
        <Col>
          <Card>
            <Card.Body>{value[0]} <a style={{float: "right"}}href={"https://github.com/LudeeD/prandium/blob/master/recipes/"+value[1]}>link</a></Card.Body>
          </Card>
        </Col>
        <Col></Col>
      </Row>
   </Container>
    ))
  );
}