mod lexer;
mod parser;
mod readfile;
mod ast;

fn main() {
    let lines = readfile::read("fufu");

    let mut PG = parser::Parser::new();

    PG.add_rule(vec!["NAME"], "expr", "Name");
    PG.add_rule(vec!["NUM"], "expr", "Num");
    PG.add_rule(vec!["STRING"], "expr", "String");

    PG.new_precedence();

    PG.add_rule(vec!["expr", "STAR", "expr"], "expr", "Mul");
    PG.add_rule(vec!["expr", "SLASH", "expr"], "expr", "Div");

    PG.new_precedence();

    PG.add_rule(vec!["expr", "PLUS", "expr"], "expr", "Add");
    PG.add_rule(vec!["expr", "MINUS", "expr"], "expr", "Minus");

    for line in lines {
        println!("{line}");
        let tokens = lexer::lex(&line);
        println!("toks: {:?}", tokens);
        unsafe {
            lexer::AST_PRINT = true;
        }
        let ast = PG.parse(&tokens, &line);
        
        println!("ast: {:?}", ast);

        println!("eval: {:?}", ast.eval());
        println!("-------------------------------------------------------------------------------------------");
    }

    
}
