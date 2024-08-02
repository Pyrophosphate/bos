use crate::error_list::BosError;
use crate::token::Token;
use crate::parser::data_type::DataType;



pub trait Evaluable {
	fn evaluate(&self) -> DataType;
	fn pre_evaluate(&mut self) -> Option<DataType> {
		None
	}
}


pub struct Expression {
	pub lhs: Equality,
}

impl Evaluable for Expression {
	fn evaluate(&self) -> DataType {
		todo!()
	}

	fn pre_evaluate(&mut self) -> Option<DataType> {
		todo!()
	}
}


pub struct Equality {
	pub lhs: Comparison,
	pub rhs: Option<(EqualityOperator, Comparison)>,
}

pub enum EqualityOperator {
	Equal, NotEqual,
}

impl Evaluable for Equality {
	fn evaluate(&self) -> DataType {
		todo!()
	}

	fn pre_evaluate(&mut self) -> Option<DataType> {
		todo!()
	}
}


pub struct Comparison {
	pub lhs: Term,
	pub rhs: Option<(ComparisonOperator, Term)>,
}

pub enum ComparisonOperator {
	Greater, GreaterEqual, Less, LessEqual,
}

impl Evaluable for Comparison {
	fn evaluate(&self) -> DataType {
		todo!()
	}

	fn pre_evaluate(&mut self) -> Option<DataType> {
		todo!()
	}
}


pub struct Term {
	pub lhs: Factor,
	pub rhs: Option<(TermOperator, Factor)>
}

pub enum TermOperator {
	Plus, Minus,
}

impl Evaluable for Term {
	fn evaluate(&self) -> DataType {
		todo!()
	}

	fn pre_evaluate(&mut self) -> Option<DataType> {
		todo!()
	}
}


pub struct Factor {
	pub lhs: Unary,
	pub rhs: Option<(FactorOperator, Unary)>,
}

pub enum FactorOperator {
	Multiply, Divide, Modulo,
}

impl Evaluable for Factor {
	fn evaluate(&self) -> DataType {
		todo!()
	}
	
	fn pre_evaluate(&mut self) -> Option<DataType> {
		todo!()
	}
}


pub enum Unary {
	WithOp(UnaryOperator, Box<Unary>),
	WithoutOp(Primary),
}

pub enum UnaryOperator {
	Not, Negate, Increment, Decrement,
}


impl Evaluable for Unary {
	fn evaluate(&self) -> DataType {
		todo!()
	}

	fn pre_evaluate(&mut self) -> Option<DataType> {
		todo!()
	}
}


pub enum Primary {
	Literal(DataType),
	//Identifier,
	ParenExpr(Box<Expression>),
}

impl Evaluable for Primary {
	fn evaluate(&self) -> DataType {
		match self {
			Primary::Literal(d) => { *d }
			Primary::ParenExpr(e) => { e.evaluate() }
		}
	}
	
	fn pre_evaluate(&mut self) -> Option<DataType> {
		match self {
			Primary::Literal(d) => {Some(*d)}
			Primary::ParenExpr(e) => {
				match e.pre_evaluate() {
					Some(d) => Some(d),
					None => None
				}
			}
		}
	}
}