// This file is part of predicator. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/predicator/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of predicator. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/predicator/master/COPYRIGHT.


#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FloatConstant
{
	llvmType: LlvmType,
	value: u64,
}

impl Constant for FloatConstant
{
	#[inline(always)]
	fn to_LLVMValueRef(&self, context: &Context, constantCache: &mut HashMap<Self, LLVMValueRef>) -> LLVMValueRef
	{
		if let Some(extant) = constantCache.get(self)
		{
			return *extant;
		}
		
		let typeRef = context.typeRef(&self.llvmType);
		
		let value = unsafe { LLVMConstReal(typeRef, transmute(self.value)) };
		
		constantCache.insert(self.clone(), value);
		
		value
	}
}

impl FloatConstant
{
	#[inline(always)]
	pub fn constantFloat16BitUnsigned(value: u16) -> Self
	{
		Self
		{
			llvmType: LlvmType::Float16,
			value: value as u64,
		}
	}
	
	#[inline(always)]
	pub fn constantFloat32BitUnsigned(value: u32) -> Self
	{
		Self
		{
			llvmType: LlvmType::Float32,
			value: value as u64,
		}
	}
	
	#[inline(always)]
	pub fn constantFloat64BitUnsigned(value: u64) -> Self
	{
		Self
		{
			llvmType: LlvmType::Float64,
			value: value,
		}
	}
}
