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