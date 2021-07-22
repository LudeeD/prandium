import React, { useState, useEffect, useRef } from "react";

import Row from "react-bootstrap/Row";
import Col from "react-bootstrap/Col";
import InputGroup from "react-bootstrap/InputGroup";
import FormControl from "react-bootstrap/FormControl";
import Button from "react-bootstrap/Button";

/**
 * Renders a single value of the array returned by db.exec(...) as a table
 * @param {import("sql.js").QueryExecResult} props
 */
export default function Input({ updatecb }) {

    const [input, setInput] = useState({name: "", ingredients: [], tags: [], number: 10});

    const update_name = (new_name) => {
        let item = { ...input, name: new_name }
        setInput(item)
    }

    const update_ingredients = (ingredients_string) => {
        let ing = ingredients_string.split(';')
        let arr = ing.map(x => x.trim())
        let item = { ...input, ingredients: arr }
        setInput(item)
    }

    const update_tags = (tags_string) => {
        let tags = tags_string.split(';')
        let arr = tags.map(x => x.trim())
        let item = { ...input, tags: arr }
        setInput(item)
    }

    const update_number = (new_number) => {
        if (new_number.length == 0) {new_number = 10} 
        let item = { ...input, number: new_number }
        setInput(item)
    }

    const numberInput = useRef(null);
    const get_random_btn = () => {
        numberInput.current.value = 1
        update_number(1)
        updatecb(input)
    }

    const get_week_btn = () => {
        numberInput.current.value = 14
        update_number(14)
        
    }

    useEffect(() => {updatecb(input)}, [input])


    return (
        <>
        <Row>
            <InputGroup className="mb-3">
                <Button style={{backgroundColor: "#0B4F6C", borderColor: "#0B4F6C"}} onClick={() => updatecb(input)} >Get Recipes</Button>
                <FormControl
                    id="name"
                    aria-label="name"
                    placeholder="name e.g: pasta" 
                    onChange={e => update_name(e.target.value)}
                    />
                <FormControl
                    id="ingredients"
                    aria-label="ingredients"
                    placeholder="ingredients e.g: tomato"
                    onChange={e => update_ingredients(e.target.value)}
                    />
                <FormControl
                    id="tag"
                    aria-label="tag"
                    placeholder="tags e.g: vegetarian; full-meal"
                    onChange={e => update_tags(e.target.value)}
                    />
                <FormControl
                    id="number"
                    aria-label="number"
                    placeholder="number e.g: 10"
                    onChange={e => update_number(e.target.value)}
                    ref={numberInput}
                    />
            </InputGroup>
        </Row>
        <Row>
            <Col sm={2}> <Button className="w-100" style={{backgroundColor: "#0B4F6C", borderColor: "#0B4F6C"}} onClick={() => get_random_btn()} >Random</Button> </Col>
            <Col sm={2}> <Button className="w-100" style={{backgroundColor: "#0B4F6C", borderColor: "#0B4F6C"}} onClick={() => get_week_btn()} >Week</Button> </Col>
        </Row>
        </>
    );
}