use crate::error_list::BosError;
use crate::token::Token;
use crate::parser::data_type::DataType;




struct Program {
	pub statements: Vec<Statement>
}


enum Statement {
	BraceStatement(Vec<Statement>),
	IfStatement(Box<Expression>, Box<Statement>),
	ReturnStatement(Box<Expression>),
	WhileStatement(Box<Expression>, Box<Statement>),
	DoWhileStatement(Box<Expression>, Box<Statement>),
	ForStatement(Box<Statement>, Box<Expression>, Box<Expression>, Box<Statement>),
	ExpressionStatement(Box<Expression>),
	PrintStatement(Box<Expression>),
}

enum Expression {
	Equality(Box<Expression>, EqualityOperator, Box<Expression>),
	Comparison(Box<Expression>, ComparisonOperator, Box<Expression>),
	Term(Box<Expression>, TermOperator, Box<Expression>),
	Factor(Box<Expression>, FactorOperator, Box<Expression>),
	Unary(Box<Expression>, UnaryOperator),
	Primary(PrimaryType)
}

struct FunctionCall {
	// Some kind of function identifier.
	pub args: Vec<Expression>
}

enum PrimaryType {
	Identifier(Box<Expression>),
	FunctionExpr(FunctionCall),
	LiteralExpr(DataType),
	ParenExpr(Box<Expression>),
}

enum UnaryOperator {
	NOT,
	NEGATE,
	PRE_INCREMENT,
	POST_INCREMENT,
	PRE_DECREMENT,
	POST_DECREMENT
}
enum TermOperator {
	PLUS,
	MINUS
}

enum FactorOperator {
	MULTIPLY,
	DIVIDE,
	MODULO
}

enum ComparisonOperator {
	GREATER,
	LESS,
	GREATER_EQUAL,
	LESS_EQUAL,
}

enum EqualityOperator {
	EQUAL,
	NOT_EQUAL
}