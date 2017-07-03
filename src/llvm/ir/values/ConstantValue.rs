// This file is part of mqtt. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/mqtt/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of mqtt. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/mqtt/master/COPYRIGHT.


#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ConstantValue(LLVMValueRefWrapper);

impl Value for ConstantValue
{
	#[inline(always)]
	fn fromLLVMValueRef(value: LLVMValueRef) -> Self
	{
		let value = LLVMValueRefWrapper::fromLLVMValueRef(value);
		
		debug_assert!(value.isConstant(), "value '{:?}' isn't constant", value);
		debug_assert!(value.isUndefined(), "value '{:?}' isn't defined", value);
		
		ConstantValue(value)
	}
	
	#[inline(always)]
	fn asLLVMValueRef(&self) -> LLVMValueRef
	{
		self.0.asLLVMValueRef()
	}
}

impl ConstantValue
{
	#[inline(always)]
	pub fn isNull(&self) -> bool
	{
		if unsafe { LLVMIsNull(self.asLLVMValueRef()) } == 0
		{
			false
		}
		else
		{
			true
		}
	}
	
	#[inline(always)]
	pub fn isString(&self) -> bool
	{
		if unsafe { LLVMIsConstantString(self.asLLVMValueRef()) } == 0
		{
			false
		}
		else
		{
			true
		}
	}
}
