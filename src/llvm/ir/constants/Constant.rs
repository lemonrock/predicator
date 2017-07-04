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
	
	ByteString
	{
		llvmType: LlvmType,
		value: Vec<u8>,
		appendAsciiNull: bool,
	},
	
	UnaryOperation
	{
		value: Box<Constant>,
		operation: UnaryOperation,
	},
	
	BinaryOperation
	{
		leftHandSide: Box<Constant>,
		rightHandSide: Box<Constant>,
		operation: BinaryOperation,
	},
	
	TernaryOperation
	{
		first: Box<Constant>,
		second: Box<Constant>,
		third: Box<Constant>,
		operation: TernaryOperation,
	},

	NullaryTypeOperation
	{
		llvmType: LlvmType,
		operation: NullaryTypeOperation,
	},
	
	UnaryTypeOperation
	{
		value: Box<Constant>,
		to: LlvmType,
		operation: UnaryTypeOperation,
	},
	
	IntegerComparison
	{
		leftHandSide: Box<Constant>,
		rightHandSide: Box<Constant>,
		predicate: UsefulLLVMIntPredicate,
	},
	
	FloatComparison
	{
		leftHandSide: Box<Constant>,
		rightHandSide: Box<Constant>,
		predicate: UsefulLLVMRealPredicate,
	},
	
	GetElementPointer
	{
		value: Box<Constant>,
		indices: Vec<Constant>,
		isInBounds: bool,
	},
	
	InlineAssembler
	{
		llvmType: LlvmType,
		assembler: String,
		constraints: String,
		hasSideEffects: bool,
		isAlignStack: bool,
	},
	
	/*
	Unimplemented
		LLVMValueRef LLVMConstExtractValue(LLVMValueRef AggConstant, unsigned *IdxList, unsigned NumIdx);
		LLVMValueRef LLVMConstInsertValue(LLVMValueRef AggConstant, LLVMValueRef ElementValueConstant, unsigned *IdxList, unsigned NumIdx);
		LLVMValueRef LLVMBlockAddress(LLVMValueRef F, LLVMBasicBlockRef BB);
    */
}

impl ToReference<ConstantValue> for Constant
{
	#[inline(always)]
	fn toReference(&self, context: &Context) -> ConstantValue
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
					
					let mut values: Vec<LLVMValueRef> = values.iter().map(|constant| constant.toReference(context).asLLVMValueRef()).collect();
					
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
				
				ByteString { ref value, ref appendAsciiNull, .. } =>
				{
					let appendAsciiNull = if *appendAsciiNull
					{
						1
					}
					else
					{
						0
					};
					
					unsafe { LLVMConstString(value.as_ptr() as *const _, value.len() as u32, appendAsciiNull) }
				}
	
				UnaryOperation { ref value, ref operation } => operation.operate(context, value),
				
				BinaryOperation { ref leftHandSide, ref rightHandSide, ref operation } => operation.operate(context, leftHandSide, rightHandSide),
				
				TernaryOperation { ref first, ref second, ref third, ref operation } => operation.operate(context, first, second, third),
				
				NullaryTypeOperation { ref llvmType, ref operation } => operation.operate(context, llvmType),
				
				UnaryTypeOperation { ref value, ref to, ref operation } => operation.operate(context, value, to),
				
				IntegerComparison { ref leftHandSide, ref rightHandSide, ref predicate } => unsafe { LLVMConstICmp(predicate.to_LLVMIntPredicate(), context.constant(leftHandSide).asLLVMValueRef(), context.constant(rightHandSide).asLLVMValueRef()) },
				
				FloatComparison { ref leftHandSide, ref rightHandSide, ref predicate } => unsafe { LLVMConstFCmp(predicate.to_LLVMRealPredicate(), context.constant(leftHandSide).asLLVMValueRef(), context.constant(rightHandSide).asLLVMValueRef()) },
				
				GetElementPointer { ref value, ref indices, ref isInBounds } =>
				{
					let valueRef = context.constant(value).asLLVMValueRef();
					let mut indicesRef = Vec::with_capacity(indices.len());
					for index in indices
					{
						indicesRef.push(context.constant(index).asLLVMValueRef());
					}
					
					let length = indices.len() as u32;
					
					if *isInBounds
					{
						unsafe { LLVMConstInBoundsGEP(valueRef, indicesRef.as_mut_ptr(), length) }
					}
					else
					{
						unsafe { LLVMConstGEP(valueRef, indicesRef.as_mut_ptr(), length) }
					}
				},
				
				InlineAssembler { ref llvmType, ref assembler, ref constraints, ref hasSideEffects, ref isAlignStack } =>
				{
					let typeRef = context.typeRef(llvmType).asLLVMTypeRef();
					
					let assembler = CString::new(&assembler[..]).unwrap();
					
					let constraints = CString::new(&constraints[..]).unwrap();
					
					let hasSideEffects = if *hasSideEffects
					{
						0
					}
					else
					{
						1
					};
					
					let isAlignStack = if *isAlignStack
					{
						0
					}
					else
					{
						1
					};
					
					unsafe { LLVMConstInlineAsm(typeRef, assembler.as_ptr(), constraints.as_ptr(), hasSideEffects, isAlignStack) }
				}
			}
		)
	}
}

static ComparisonType: LlvmType = LlvmType::Int1;

impl Constant
{
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
			
			ByteString { ref llvmType, .. } => llvmType,
			
			UnaryOperation { ref value, .. } => value.llvmType(),
			
			BinaryOperation { ref leftHandSide, .. } => leftHandSide.llvmType(),
			
			TernaryOperation { ref first, ref second, ref third, ref operation } => operation.llvmType(first, second, third),
			
			NullaryTypeOperation { ref llvmType, .. } => llvmType,
			
			UnaryTypeOperation { ref to, .. } => to,
			
			IntegerComparison { .. } => &ComparisonType,
			
			FloatComparison { .. } => &ComparisonType,
			
			GetElementPointer { .. } => unimplemented!(), // Should be possible to calculate by getting the integer value of self.value
			
			InlineAssembler { ref llvmType, .. } => llvmType,
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
		value: 0,
		signed: false,
	};
	
	#[inline(always)]
	pub fn boolean(value: bool) -> Self
	{
		if value
		{
			Self::True
		}
		else
		{
			Self::False
		}
	}
	
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
	
	#[inline(always)]
	pub fn byteString(value: String, appendAsciiNull: bool) -> Self
	{
		Constant::ByteString
		{
			llvmType: LlvmType::pointer(LlvmType::Int8),
			value: value.into_bytes(),
			appendAsciiNull: appendAsciiNull,
		}
	}
	
	#[inline(always)]
	pub fn sizeOf(llvmType: LlvmType) -> Self
	{
		Constant::NullaryTypeOperation
		{
			llvmType: llvmType,
			operation: NullaryTypeOperation::SizeOf,
		}
	}
	
	#[inline(always)]
	pub fn bitCastTo(value: Constant, to: LlvmType) -> Self
	{
		Constant::UnaryTypeOperation
		{
			value: Box::new(value),
			to: to,
			operation: UnaryTypeOperation::BitCast,
		}
	}
}
