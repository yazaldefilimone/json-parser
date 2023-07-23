#[derive(Debug)]
#[derive(PartialEq)]
pub enum TokenType {
  ILLEGAL,
  EOF,
  TRUE,
  FALSE,
  NULL,
  STRING,
  NUMBER,
  COMMA,
  SEMICOLON,
  COLON,
  LEFTBRACE,
  RIGHTBRACE,
  LEFTBRACKET,
  RIGHTBRACKET,
}
#[derive(Debug)]
pub struct Token {
  pub token_type: TokenType,
  pub literal: String,
}

pub struct Lexer {
  input: String,
  position: usize,
  read_position: usize,
  character: char,
}

impl Lexer {
  pub fn new(input: String) -> Self {
    let mut lexer = Self {
      input,
      position: 0,
      read_position: 0,
      character: '\0',
    };
    lexer.read_charecter();
    lexer
  }

  pub fn next_token(&mut self) -> Token {
    self.skip_whitespace();
    let token: Token = match self.character {
      ':' => self.create_token(TokenType::COLON, self.character.to_string()),
      ';' => self.create_token(TokenType::SEMICOLON, self.character.to_string()),
      ',' => self.create_token(TokenType::COMMA, self.character.to_string()),
      '{' => self.create_token(TokenType::LEFTBRACE, self.character.to_string()),
      '}' => self.create_token(TokenType::RIGHTBRACE, self.character.to_string()),
      '[' => self.create_token(TokenType::LEFTBRACKET, self.character.to_string()),
      ']' => self.create_token(TokenType::RIGHTBRACKET, self.character.to_string()),
      '"' => {
        self.read_charecter();
        let literal: String = self.read_string();
        return self.create_token(TokenType::STRING, literal);
      }
      't' => {
        let literal: String = self.read_identifier();
        if literal == "true" {
          return self.create_token(TokenType::TRUE, literal);
        }
        return self.create_token(TokenType::ILLEGAL, literal);
      }
      'f' => {
        let literal: String = self.read_identifier();
        if literal == "false" {
          return self.create_token(TokenType::FALSE, literal);
        }
        return self.create_token(TokenType::ILLEGAL, literal);
      }
      'n' => {
        let literal: String = self.read_identifier();
        if literal == "null" {
          return self.create_token(TokenType::NULL, literal);
        }
        return self.create_token(TokenType::ILLEGAL, literal);
      }
      '\0' => self.create_token(TokenType::EOF, self.character.to_string()),
      _ => {
        if self.is_digit(self.character) {
          let literal: String = self.read_number();
          return self.create_token(TokenType::NUMBER, literal);
        }

        // if self.is_letter(self.character) {
        //   let literal: String = self.read_identifier();
        //   return self.create_token(TokenType::STRING, literal);
        // }
        return self.create_token(TokenType::ILLEGAL, self.character.to_string());
      }
    };

    return token;
  }
  fn create_token(&mut self, token_type: TokenType, literal: String) -> Token {
    self.read_charecter();
    let token: Token = Token {
      token_type,
      literal,
    };
    return token;
  }
  fn skip_whitespace(&mut self) {
    while self.is_whitespace(self.character) {
      self.read_charecter();
    }
  }

  fn read_charecter(&mut self) -> () {
    if self.read_position >= self.input.len() {
      self.character = '\0';
    } else {
      self.character = self.input.chars().nth(self.read_position).unwrap();
    }
    self.position = self.read_position;
    self.read_position += 1;
  }
  fn read_number(&mut self) -> String {
    let start_postion: usize = self.position;
    while self.is_digit(self.character) {
      self.read_charecter();
    }
    let litteral: &str = self.input.get(start_postion..self.position).unwrap();
    return litteral.to_owned();
  }

  fn read_identifier(&mut self) -> String {
    let start_postion: usize = self.position;
    while self.is_letter(self.character) {
      self.read_charecter();
    }
    let litteral: &str = self.input.get(start_postion..self.position).unwrap();
    return litteral.to_owned();
  }
  fn read_string(&mut self) -> String {
    let start_postion: usize = self.position;
    while self.character != '"' {
      self.read_charecter();
    }
    let litteral: &str = self.input.get(start_postion..self.position).unwrap();
    return litteral.to_owned();
  }

  // validate if the character is a letter
  fn is_whitespace(&self, character: char) -> bool {
    let is_space: bool =
      character.is_whitespace() ||
      character == '\t' ||
      character == '\n' ||
      character == '\r' ||
      character == '\0';
    return is_space;
  }

  fn is_letter(&self, character: char) -> bool {
    let is_lower_letter: bool = 'a' <= character && character <= 'z';
    let is_upper_letter = 'A' <= character && character <= 'Z';
    return is_lower_letter || is_upper_letter || character == '_';
  }

  fn is_digit(&self, character: char) -> bool {
    return character.is_digit(10);
  }
}
