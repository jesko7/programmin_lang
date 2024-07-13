use std::fmt::Debug;
use crate::ast;
use crate::lexer::Token;
//use crate::ast::*
//rule.class_name, next_token_values, next_token_names, index, line


#[derive(Clone)]
pub struct Node {
    pub class_name: String,
    pub token_values: Vec<Node>,
    pub token_names: Vec<String>,
    pub index: usize,
    pub line: String,
    pub token: Option<Token>
}


#[derive(Debug, Clone)]
pub enum ValType {
    NULL,
    NUM,
    STRING,
}
#[derive(Debug, Clone)]
pub struct EvalVal {
    pub num: Option<f32>,
    pub string: Option<String>,
    pub valtype: ValType
}

impl Default for EvalVal {
    fn default() -> Self {
        EvalVal { num: None, string: None, valtype: ValType::NULL}
    }
}



impl Node {
    pub fn eval(&self) -> Option<EvalVal> {
        match self.class_name.as_str() {
            "Num" => ast::Num(self.clone()),
            "String" => ast::String(self.clone()),
            "Div" => ast::Div(self.clone()),
            "Mul" => ast::Mul(self.clone()),
            "Add" => ast::Add(self.clone()),
            "Minus" => ast::Minus(self.clone()),
            other => panic!("node eval not implemented: {}", other)
        }
    } 
}

impl Debug for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.class_name.as_str() {
            "Add" => write!(f, "Add({:?} + {:?})", self.token_values[0], self.token_values[2]),
            "Mul" => write!(f, "Mul({:?} * {:?})", self.token_values[0], self.token_values[2]),
            "Div" => write!(f, "Div({:?} / {:?})", self.token_values[0], self.token_values[2]),
            "Minus" => write!(f, "Minus({:?} - {:?})", self.token_values[0], self.token_values[2]),
            "TOKEN" => write!(f, "{}", self.token.as_ref().unwrap().value()),
            "Num" => write!(f, "{:?}", self.token_values[0].token.as_ref().unwrap()),
            "String" => write!(f, "{:?}", self.token_values[0].token.as_ref().unwrap()),
            "Name" => write!(f, "{:?}", self.token_values[0].token.as_ref().unwrap()),
            other => todo!("{other}")
        }
        
    }
}


pub struct Rule
{
    pub pattern: Vec<String>,
    pub class_name: String,
    pub name: String
}

pub struct Parser
{
    pub rules: Vec<Vec<Rule>>,
    pub precedence: usize
}

impl Parser
{
    pub fn new() -> Parser {
        Parser {
            rules: vec![vec![]],
            precedence: 0
        }
    }

    pub fn add_rule(&mut self, pattern: Vec<&str>, name: &str, class_name: &str)
    {
        let mut pattern2 = vec![];

        for value in pattern {
            pattern2.push(value.to_string());
        }

        let name2 = name.to_string();

        let class_name2 = class_name.to_string();

        let rule = Rule {
            pattern: pattern2,
            class_name: class_name2,
            name: name2
        };
        self.rules[self.precedence].push(rule);
    }

    pub fn new_precedence(&mut self) {
        self.rules.push(vec![]);
        self.precedence += 1;
    }

    fn parse_rules(&self, mut names: Vec<String>, mut values: Vec<Node>, line: &str, rules: &Vec<Rule<>>) -> (Vec<Node>, Vec<String>){

        
        
        for rule in rules {
            let mut index = 0;
            
            if names.len() < rule.pattern.len() {
                continue;
            }
            //println!("names: {:?}, rule: {:?}",  names, rule.pattern);
            
            //go throug all Tokens/Nodes
            while index <= names.len() - rule.pattern.len() {
                //println!("index: {}", index);
                //println!("names: {:?}, {:?}", future_names, names);
                //get:? the next tokens
                //println!("{:?}, {}, {}", names, index, index + rule.pattern.len());
                let next_token_names: Vec<String> = names[index..index + rule.pattern.len()].to_vec();
                //println!("{:?}, {:?}, {}, {:?}", next_token_names, names, index, rule.pattern);
                //matches
                if next_token_names == rule.pattern {
                    let next_token_values: Vec<Node> = values[index..index + rule.pattern.len()].to_vec();

                    let node = Node {
                        class_name: rule.class_name.clone(),
                        token_values: next_token_values,
                        token_names: next_token_names,
                        index: index,
                        line: line.to_string(),
                        token: None
                    };
                    
                    for _ in 0..rule.pattern.len() {
                        values.remove(index);
                        names.remove(index);
                    }
                    
                    values.insert(index, node);
                    names.insert(index, rule.name.clone());
                    
                    index += 1;
                    //println!("matches: {:?}, {}", names, index);
                } else {
                    index += 1;
                }


                if names.len() < rule.pattern.len() {
                    break;
                }
            }

            //println!("{:?}", names);
        }

        //println!("names at the end: {:?}", names);

        (values, names)
    }   

    pub fn parse(&self, tokens: &Vec<Token>, line: &str) -> Node {
        let mut names: Vec<String> = vec![];
        let mut values: Vec<Node> = vec![];

        //put the tokens in the ast tree
        for token in tokens {
            values.push(
                Node {
                    class_name: "TOKEN".to_string(),
                    token_values: vec![],
                    token_names: vec![],
                    index: 0,
                    line: "TOKEN".to_string(),
                    token: Some(token.clone())
                }
            );
            names.push(token.name());
        }

        //aply rules until it hasnt changed
        loop {
            let prev_ast = names.clone();

            for rules_in_precedence in self.rules.iter() {
                (values, names) = self.parse_rules(names, values, line, rules_in_precedence);
            }

            //println!("names: {:?}", names);

            //hasnt changed
            if names == prev_ast {
                break;
            }
        }

        println!("{:?}", values);

        if values.len() != 1 {
            panic!("syntaxt error in line: {line}")
        }

        return values[0].clone();
    }
}