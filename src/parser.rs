use lexer::Token;

use crate::lexer;
#[derive(Debug)]
pub struct Ast {
  kind: String,
  children: Vec<Ast>,
  value: Option<String>,
}

pub fn parser(tokens: Vec<Token>) -> impl Fn() -> Ast {
  let scan = move || {
    let mut current_index: usize = 0;

    let mut ast: Ast = Ast {
      kind: "ObjectExpression".to_string(),
      children: Vec::new(),
      value: None,
    };
    while current_index < tokens.len() {
      let token: &Token = tokens.get(current_index).unwrap();
      match token.token_type {
        lexer::TokenType::LEFTBRACE => {
          let mut object: Ast = register_object("{".to_string());
          current_index += 1;

          loop {
            if tokens.get(current_index).unwrap().token_type == lexer::TokenType::RIGHTBRACE {
              ast.children.push(object);
              current_index += 1;
              let close_brace: Ast = register_object("}".to_string());
              ast.children.push(close_brace);
              break;
            }
            let key: Ast = register_literal(&tokens.get(current_index).unwrap());
            object.children.push(key);
            current_index += 1;
          }
        }
        lexer::TokenType::RIGHTBRACE => {
          current_index += 1;
        }
        lexer::TokenType::LEFTBRACKET => {
          let mut array: Ast = register_array("[".to_string());
          current_index += 1;
          loop {
            if tokens.get(current_index).unwrap().token_type == lexer::TokenType::RIGHTBRACKET {
              ast.children.push(array);
              current_index += 1;
              let close_bracket: Ast = register_array("]".to_string());
              ast.children.push(close_bracket);
              break;
            }
            let value: Ast = register_literal(&tokens.get(current_index).unwrap());
            array.children.push(value);
            current_index += 1;
          }
        }
        lexer::TokenType::RIGHTBRACKET => {
          current_index += 1;
        }
        _ => {
          current_index += 1;
        }
      }
    }
    ast
  };

  scan
}

fn register_object(typ: String) -> Ast {
  Ast {
    kind: "ObjectExpression".to_string(),
    children: Vec::new(),
    value: Some(typ),
  }
}

fn register_array(typ: String) -> Ast {
  Ast {
    kind: "ArrayExpression".to_string(),
    children: Vec::new(),
    value: Some(typ),
  }
}

fn register_literal(token_type: &lexer::Token) -> Ast {
  let kind = match token_type.token_type {
    lexer::TokenType::STRING => "StringLiteral",
    lexer::TokenType::NUMBER => "NumericLiteral",
    lexer::TokenType::BOOLEAN => "BooleanLiteral",
    lexer::TokenType::NULL => "NullLiteral",
    lexer::TokenType::UNDEFINED => "UndefinedLiteral",
    _ => "UnknownLiteral",
  };
  Ast {
    kind: kind.to_string(),
    children: Vec::new(),
    value: Some(token_type.literal.to_string()),
  }
}
