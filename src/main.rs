mod lexer;
mod parser;
use lexer::Lexer;
use parser::parser;

fn main() {
  let json: &str =
    r#"
    {
      "name": "John Doe",
      "age": 43,
      "is_active": true,
      "address": {
        "street": "123 Main St",
        "city": "New York",
        "state": "NY",
        "zip": "10001"
        "en": null
      },
      "phone_numbers": [
        "+1 (555) 555-1234",
        "+1 (555) 555-5678"
      ]
    }
    "#;

  let code: String = String::from(json);
  let mut lexer: Lexer = Lexer::new(code);
  let mut token: lexer::Token = lexer.next_token();
  let mut tokens: Vec<lexer::Token> = Vec::new();
  loop {
    tokens.push(token);
    token = lexer.next_token();
    if token.token_type == lexer::TokenType::EOF {
      break;
    }
  }
  let mut paser = parser(tokens);
  let json: parser::Ast = paser();
  dbg!(json);
}
