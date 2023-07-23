mod lexer;
use lexer::Lexer;

fn main() {
  let json =
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
  let mut lexer = Lexer::new(code);
  let mut token = lexer.next_token();

  loop {
    if token.token_type == lexer::TokenType::EOF {
      break;
    }
    println!("token: {:?}", token);
    token = lexer.next_token();
  }
}
