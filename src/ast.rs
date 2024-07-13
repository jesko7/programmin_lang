use crate::{lexer, parser::{Node, EvalVal, ValType}};
 

pub fn Add(node: Node) -> Option<EvalVal> {
    let num1 = node.token_values[0].eval();
    let num2 = node.token_values[2].eval();

    let output = num1.unwrap().num.unwrap() + num2.unwrap().num.unwrap();

    Some(
        EvalVal {  
            num: Some(output),  
            valtype: ValType::NUM,
            ..Default::default()
        }
    )
}

pub fn Minus(node: Node) -> Option<EvalVal> {
    let num1 = node.token_values[0].eval();
    let num2 = node.token_values[2].eval();

    let output = num1.unwrap().num.unwrap() - num2.unwrap().num.unwrap();

    Some(
        EvalVal {  
            num: Some(output),
            valtype: ValType::NUM,
            ..Default::default()
        }
    )
}

pub fn Mul(node: Node) -> Option<EvalVal> {
    let num1 = node.token_values[0].eval();
    let num2 = node.token_values[2].eval();

    let output = num1.unwrap().num.unwrap() * num2.unwrap().num.unwrap();

    Some(
        EvalVal {  
            num: Some(output),
            valtype: ValType::NUM,
            ..Default::default()
        }
    )
}

pub fn Div(node: Node) -> Option<EvalVal> {
    let num1 = node.token_values[0].eval();
    let num2 = node.token_values[2].eval();

    let output = num1.unwrap().num.unwrap() / num2.unwrap().num.unwrap();

    Some(
        EvalVal {  
            num: Some(output),
            valtype: ValType::NUM,
            ..Default::default()
        }
    )
}

pub fn Num(node: Node) -> Option<EvalVal> {
    let output = node.token_values[0].token.as_ref().unwrap().value().parse::<f32>().unwrap();

    Some(
        EvalVal {  
            num: Some(output),
            valtype: ValType::NUM,
            ..Default::default()
        }
    )
}



pub fn String(node: Node) -> Option<EvalVal> {
    let output = node.token_values[0].token.as_ref().unwrap().value();

    Some(
        EvalVal {  
            string: Some(output),
            valtype: ValType::STRING,
            ..Default::default()
        }
    )
}

