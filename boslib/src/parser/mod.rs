use std::cmp::PartialEq;
use std::num::ParseIntError;
use std::string::ParseError;
use crate::error_list::BosError;
use crate::parser::ast_nodes::{Comparison, Equality, Expression, Evaluable, Factor, Primary, Term, Unary, UnaryOperator, FactorOperator};
use crate::parser::ast_nodes::Primary::{Literal, ParenExpr};
use crate::parser::data_type::DataType;
use crate::token::{Token, TokenType};

mod ast_nodes;
mod data_type;


pub struct BosParser {
	//tokens: Vec<Token>,
	current: usize,
	
}


impl BosParser {
	
	pub fn new(list: Vec<Token>) -> Self {
		Self {
			//tokens: list,
			current: 0,
		}
	}
	

	pub fn parse(&mut self, tokens: Vec<Token>) {

	}
	
	
	
	fn parse_expression(&mut self, tokens: &Vec<Token>) -> Result<Expression, BosError> {
		return match self.parse_equality(tokens) {
			Ok(eq) => {Ok(Expression{lhs: eq})}
			Err(e) => {Err(e)}
		};
		// Err(BosError {
		// 	name: "Parse Error".to_string(),
		// 	detail: "Unable to find next token.".to_string(),
		// 	line: 0,
		// 	column: 0,
		// }) //Expr { lhs: self.parse_equality() }
	}
	
	
	fn parse_equality(&mut self, tokens: &Vec<Token>) -> Result<Equality, BosError> {
		return Err(BosError {
			name: "Parse Error".to_string(),
			detail: "Unable to find next token.".to_string(),
			line: 0,
			column: 0,
		}) //Equality {lhs: self.parse_comparison(), rhs }
	}
	
	
	fn parse_comparison(&mut self, tokens: &Vec<Token>) -> Result<Comparison, BosError> {
		Err(BosError {
			name: "Parse Error".to_string(),
			detail: "Unable to find next token.".to_string(),
			line: 0,
			column: 0,
		})
	}
	
	
	fn parse_term(&mut self, tokens: &Vec<Token>) -> Result<Term, BosError> {
		Err(BosError {
			name: "Parse Error".to_string(),
			detail: "Unable to find next token.".to_string(),
			line: 0,
			column: 0,
		})
	}


	fn parse_factor(&mut self, tokens: &Vec<Token>) -> Result<Factor, BosError> {
		let lhs = match self.parse_unary(tokens) {
			Ok(u) => {u}
			Err(e) => {return Err(e);}
		};

		let mut current_factor: Factor;

		while (self.match_current(&*[TokenType::STAR, TokenType::SLASH, TokenType::MODULO], tokens)) {
			let operator = match self.previous(tokens).unwrap().get_type() {
				TokenType::STAR => {FactorOperator::Multiply}
				TokenType::SLASH => {FactorOperator::Divide}
				TokenType::MODULO => {FactorOperator::Modulo}
				_ => {return Err(self.parser_error("This should be unreachable.".to_string(),
												   self.previous(tokens).unwrap().get_line(),
												   self.previous(tokens).unwrap().get_column()))}
			};

			let rhs = match self.parse_unary(tokens) {
				Ok(u) => {u}
				Err(e) => {return Err(e);}
			};

			current_factor = Factor {
				lhs:lhs,
				rhs: Some((operator, rhs))
			};
		}


	}
	
	
	fn parse_unary(&mut self, tokens: &Vec<Token>) -> Result<Unary, BosError> {
		if (self.match_current(&*[TokenType::BANG, TokenType::MINUS], tokens)) {
			let operator = match self.previous(tokens).unwrap().get_type() {
				TokenType::MINUS => {UnaryOperator::Negate}
				TokenType::BANG => {UnaryOperator::Not}
				_ => {return Err(self.parser_error("This should be unreachable.".to_string(),
												   self.previous(tokens).unwrap().get_line(),
												   self.previous(tokens).unwrap().get_column()))}
			};

			let rhs = match self.parse_unary(tokens) {
				Ok(p) => {p}
				Err(e) => {return Err(e);}
			};

			return Ok(Unary::WithOp(operator, Box::from(rhs)));
		}

		let lhs = match self.parse_primary(tokens) {
			Ok(p) => {p}
			Err(e) => {return Err(e);}
		};

		return Ok(Unary::WithoutOp(lhs));
	}
	
	
	fn parse_primary(&mut self, tokens: &Vec<Token>) -> Result<Primary, BosError> {
		let next_token = match tokens.get(self.current) {
			None => {return Err(BosError {
				name: "Parse Error".to_string(),
				detail: "Unable to find next token.".to_string(),
				line: 0,
				column: 0,
			})}
			Some(t) => { t }
		};
		
		match next_token.get_type() {
			
			TokenType::CHAR_LIT => {
				let data: char = next_token.get_lexeme().chars().next().unwrap();
				Ok(Literal(DataType::CHAR(data)))
			}
			
			TokenType::INTEGER_LIT => {
				let data: i64 = match next_token.get_lexeme().parse::<i64>() {
					Ok(i) => {i}
					Err(_) => {return Err(BosError {
						name: "Parse Error".to_string(),
						detail: "Unable to parse integer token.".to_string(),
						line: next_token.get_line(),
						column: next_token.get_column(),
					})}
				};
				Ok(Literal(DataType::INT(data)))
			}
			
			TokenType::FLOAT_LIT => {
				let data: f64 = match next_token.get_lexeme().parse::<f64>() {
					Ok(f) => {f}
					Err(_) => {return Err(BosError {
						name: "Parse Error".to_string(),
						detail: "Unable to parse floating-point token.".to_string(),
						line: next_token.get_line(),
						column: next_token.get_column(),
					})}
				};
				Ok(Literal(DataType::FLOAT(data)))
			}
			
			TokenType::LEFT_PAREN => {
				let expr = self.parse_expression(tokens);
				
				match expr {
					Ok(ex) => {
						if self.match_current(&[TokenType::RIGHT_PAREN], tokens) {
							Ok(ParenExpr(Box::from(ex)))
						} else {
							Err(self.parser_error("Expected ')' token.".to_string(), next_token.get_line(), next_token.get_column()))
						}
					}
					Err(e) => { return Err(e) }
				}
			}
			_ => Err(self.parser_error("Unexpected token.".to_string(), next_token.get_line(), next_token.get_column()))
		}
	}
	
	
	fn peek<'a>(&'a self, tokens: &'a Vec<Token>) -> Option<&Token> {
		tokens.get(self.current)
	}
	
	fn advance(&mut self, tokens: &Vec<Token>) {
		if(!self.is_at_end(tokens)) {
			self.current += 1;
		}
	}

	fn is_at_end(&self, tokens: &Vec<Token>) -> bool {
		return match self.peek(tokens) {
			None => { true }
			Some(t) => { t.get_type() == TokenType::EOF }
		}
	}

	fn check_current(&self, t_type: TokenType, tokens: &Vec<Token>) -> bool {
		if(self.is_at_end(tokens)) {
			return false;
		}
		return tokens.get(self.current).unwrap().get_type() == t_type;
	}

	fn match_current(&mut self, tt: &[TokenType], tokens: &Vec<Token>) -> bool {
		for &t in tt {
			if self.check_current(t, tokens) {
				self.advance(tokens);
				return true;
			}
		}
		return false;
	}

	fn previous<'a>(&'a self, tokens: &'a Vec<Token>) -> Option<&Token> {
		return match tokens.get(self.current - 1) {
			None => {None}
			Some(t) => {Some(t)}
		}
	}

	fn parser_error(&self, msg: String, line: u32, col: usize) -> BosError {
		BosError{
			name: "Parse Error".to_string(),
			detail: msg,
			line: line,
			column: col,
		}
	}
}