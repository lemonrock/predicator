// This file is part of predicator. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/predicator/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of predicator. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/predicator/master/COPYRIGHT.


#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Constant
{
	Integer
	{
		llvmType: LlvmType,
		value: u64,
		signed: bool,
	},
	
	Float
	{
		llvmType: LlvmType,
		value: u64,
	},
	
	Struct
	{
		llvmType: LlvmType,
		values: Vec<Constant>,
	},
	
	NullPointer
	{
		llvmType: LlvmType,
	},
	
	Zeroed
	{
		llvmType: LlvmType,
	},
	
	Undefined
	{
		llvmType: LlvmType,
	},
}

impl Constant
{
	#[inline(always)]
	pub fn toConstantValue(&self, context: &Context) -> ConstantValue
	{
		use self::Constant::*;
		
		ConstantValue::fromLLVMValueRef
		(
			match *self
			{
				Integer { ref llvmType, ref value, ref signed } =>
				{
					let typeRef = context.typeRef(llvmType).asLLVMTypeRef();
					
					let signed = if *signed
					{
						1
					}
					else
					{
						0
					};
					
					// NOTE: Need to use LLVMConstIntOfArbitraryPrecision() for 128-bit integers
					unsafe { LLVMConstInt(typeRef, *value, signed) }
				}
				
				Float { ref llvmType, ref value } =>
				{
					let typeRef = context.typeRef(llvmType).asLLVMTypeRef();
					
					unsafe { LLVMConstReal(typeRef, transmute(*value)) }
				}
				
				Struct { ref llvmType, ref values } =>
				{
					let typeRef = context.typeRef(&llvmType).asLLVMTypeRef();
					
					let mut values: Vec<LLVMValueRef> = values.iter().map(|constant| constant.toConstantValue(context).asLLVMValueRef()).collect();
					
					unsafe { LLVMConstNamedStruct(typeRef, values.as_mut_ptr(), values.len() as u32) }
				}
				
				NullPointer { ref llvmType } =>
				{
					let typeRef = context.typeRef(&llvmType).asLLVMTypeRef();
					
					unsafe { LLVMConstPointerNull(typeRef) }
				}
				
				Zeroed { ref llvmType } =>
				{
					let typeRef = context.typeRef(&llvmType).asLLVMTypeRef();
					
					unsafe { LLVMConstNull(typeRef) }
				}
				
				Undefined { ref llvmType } =>
				{
					let typeRef = context.typeRef(&llvmType).asLLVMTypeRef();
					
					unsafe { LLVMGetUndef(typeRef) }
				}
			}
		)
	}
	
	#[inline(always)]
	pub fn llvmType(&self) -> &LlvmType
	{
		use self::Constant::*;
		
		match *self
		{
			Integer { ref llvmType, .. } => llvmType,
			
			Float { ref llvmType, .. } => llvmType,
			
			Struct { ref llvmType, .. } => llvmType,
			
			NullPointer { ref llvmType } => llvmType,
			
			Zeroed { ref llvmType } => llvmType,
			
			Undefined { ref llvmType } => llvmType,
		}
	}
	
	pub const True: Constant = Constant::Integer
	{
		llvmType: LlvmType::Int1,
		value: 1,
		signed: false,
	};
	
	pub const False: Constant = Constant::Integer
	{
		llvmType: LlvmType::Int1,
		value: 1,
		signed: false,
	};
	
	#[inline(always)]
	pub fn integer8BitUnsigned(value: u8) -> Self
	{
		Constant::Integer
		{
			llvmType: LlvmType::Int8,
			value: value as u64,
			signed: false,
		}
	}
	
	#[inline(always)]
	pub fn integer8BitSigned(value: i8) -> Self
	{
		let value: u8 = unsafe { transmute(value) };
		
		Constant::Integer
		{
			llvmType: LlvmType::Int8,
			value: value as u64,
			signed: true,
		}
	}
	
	#[inline(always)]
	pub fn integer16BitUnsigned(value: u16) -> Self
	{
		Constant::Integer
		{
			llvmType: LlvmType::Int16,
			value: value as u64,
			signed: false,
		}
	}
	
	#[inline(always)]
	pub fn integer16BitSigned(value: i16) -> Self
	{
		let value: u16 = unsafe { transmute(value) };
		
		Constant::Integer
		{
			llvmType: LlvmType::Int16,
			value: value as u64,
			signed: true,
		}
	}
	
	#[inline(always)]
	pub fn integer32BitUnsigned(value: u32) -> Self
	{
		Constant::Integer
		{
			llvmType: LlvmType::Int32,
			value: value as u64,
			signed: false,
		}
	}
	
	#[inline(always)]
	pub fn integer32BitSigned(value: i32) -> Self
	{
		let value: u32 = unsafe { transmute(value) };
		
		Constant::Integer
		{
			llvmType: LlvmType::Int32,
			value: value as u64,
			signed: true,
		}
	}
	
	#[inline(always)]
	pub fn integer64BitUnsigned(value: u64) -> Self
	{
		Constant::Integer
		{
			llvmType: LlvmType::Int64,
			value: value,
			signed: false,
		}
	}
	
	#[inline(always)]
	pub fn integer64BitSigned(value: i64) -> Self
	{
		let value: u64 = unsafe { transmute(value) };
		
		Constant::Integer
		{
			llvmType: LlvmType::Int64,
			value: value,
			signed: true,
		}
	}
	#[inline(always)]
	pub fn float16BitUnsigned(value: u16) -> Self
	{
		Constant::Float
		{
			llvmType: LlvmType::Float16,
			value: value as u64,
		}
	}
	
	#[inline(always)]
	pub fn float32BitUnsigned(value: u32) -> Self
	{
		Constant::Float
		{
			llvmType: LlvmType::Float32,
			value: value as u64,
		}
	}
	
	#[inline(always)]
	pub fn float64BitUnsigned(value: u64) -> Self
	{
		Constant::Float
		{
			llvmType: LlvmType::Float64,
			value: value,
		}
	}
	
	#[inline(always)]
	pub fn anonymousStruct(isPacked: bool, values: Vec<Constant>) -> Self
	{
		Constant::Struct
		{
			llvmType: LlvmType::anonymousStruct(isPacked, values.iter().map(|constant| constant.llvmType().clone()).collect()),
			values: values,
		}
	}
	
	#[inline(always)]
	pub fn namedStruct(name: &str, isPacked: bool, values: Vec<Constant>) -> Self
	{
		Constant::Struct
		{
			llvmType: LlvmType::namedStruct(name, isPacked, values.iter().map(|constant| constant.llvmType().clone()).collect()),
			values: values,
		}
	}
	
	#[inline(always)]
	pub fn nullPointer(underlying: LlvmType) -> Self
	{
		Constant::NullPointer
		{
			llvmType: underlying,
		}
	}
	
	#[inline(always)]
	pub fn zeroed(underlying: LlvmType) -> Self
	{
		Constant::Zeroed
		{
			llvmType: underlying,
		}
	}
	
	#[inline(always)]
	pub fn undefined(underlying: LlvmType) -> Self
	{
		Constant::Undefined
		{
			llvmType: underlying,
		}
	}
}
