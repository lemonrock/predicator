// This file is part of predicator. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/predicator/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of predicator. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/predicator/master/COPYRIGHT.


#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct IntegerConstant
{
	llvmType: LlvmType,
	value: u64,
	signed: bool,
}

impl Constant for IntegerConstant
{
	#[inline(always)]
	fn to_LLVMValueRef(&self, context: &Context) -> LLVMValueRef
	{
		let mut constantCache = context.integerConstantCache.borrow_mut();
		
		if let Some(extant) = constantCache.get(self)
		{
			return *extant;
		}
		
		let typeRef = context.typeRef(&self.llvmType);
		
		// NOTE: Need to use LLVMConstIntOfArbitraryPrecision() for 128-bit integers
		let value = unsafe { LLVMConstInt(typeRef, self.value, 0) };
		
		constantCache.insert(self.clone(), value);
		
		value
	}
	
	#[inline(always)]
	fn llvmType(&self) -> &LlvmType
	{
		&self.llvmType
	}
}

impl IntegerConstant
{
	pub const True: IntegerConstant = IntegerConstant
	{
		llvmType: LlvmType::Int1,
		value: 1,
		signed: false,
	};
	
	pub const False: IntegerConstant = IntegerConstant
	{
		llvmType: LlvmType::Int1,
		value: 1,
		signed: false,
	};
	
	#[inline(always)]
	pub fn constantInteger8BitUnsigned(value: u8) -> Self
	{
		Self
		{
			llvmType: LlvmType::Int8,
			value: value as u64,
			signed: false,
		}
	}
	
	#[inline(always)]
	pub fn constantInteger8BitSigned(value: i8) -> Self
	{
		let value: u8 = unsafe { transmute(value) };
		
		Self
		{
			llvmType: LlvmType::Int8,
			value: value as u64,
			signed: true,
		}
	}
	
	#[inline(always)]
	pub fn constantInteger16BitUnsigned(value: u16) -> Self
	{
		Self
		{
			llvmType: LlvmType::Int16,
			value: value as u64,
			signed: false,
		}
	}
	
	#[inline(always)]
	pub fn constantInteger16BitSigned(value: i16) -> Self
	{
		let value: u16 = unsafe { transmute(value) };
		
		Self
		{
			llvmType: LlvmType::Int16,
			value: value as u64,
			signed: true,
		}
	}
	
	#[inline(always)]
	pub fn constantInteger32BitUnsigned(value: u32) -> Self
	{
		Self
		{
			llvmType: LlvmType::Int32,
			value: value as u64,
			signed: false,
		}
	}
	
	#[inline(always)]
	pub fn constantInteger32BitSigned(value: i32) -> Self
	{
		let value: u32 = unsafe { transmute(value) };
		
		Self
		{
			llvmType: LlvmType::Int32,
			value: value as u64,
			signed: true,
		}
	}
	
	#[inline(always)]
	pub fn constantInteger64BitUnsigned(value: u64) -> Self
	{
		Self
		{
			llvmType: LlvmType::Int64,
			value: value,
			signed: false,
		}
	}
	
	#[inline(always)]
	pub fn constantInteger64BitSigned(value: i64) -> Self
	{
		let value: u64 = unsafe { transmute(value) };
		
		Self
		{
			llvmType: LlvmType::Int64,
			value: value,
			signed: true,
		}
	}
}
