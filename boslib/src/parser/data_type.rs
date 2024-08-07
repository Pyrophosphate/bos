use std::rc::Rc;

#[derive(Copy, Clone, Debug)]
pub enum DataType {
	// Integer types
	// INT8(i8),
	// INT16(i16),
	// INT32(i32),
	// INT64(i64),
	// UINT8(u8),
	// UINT16(u16),
	// UINT32(u32),
	// UINT64(u64),
	INT(i64),
	FLOAT(f64),
	
	// Floating-point types
	// FLOAT(f32),
	// DOUBLE(f64),
	
	// Other types
	BOOL(bool),
	CHAR(char),
	STRING,
}

