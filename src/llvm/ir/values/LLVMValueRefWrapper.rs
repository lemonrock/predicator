// This file is part of mqtt. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/mqtt/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of mqtt. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/mqtt/master/COPYRIGHT.


#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct LLVMValueRefWrapper(LLVMValueRef);

impl Debug for LLVMValueRefWrapper
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error>
	{
		write!(f, "LLVMValueRefWrapper({:?}={:?})", self.0, self.toString())
	}
}

impl ToLLVMValueRefWrapper for LLVMValueRefWrapper
{
	#[inline(always)]
	fn toLLVMValueRefWrapper(&self, _: &Context) -> LLVMValueRefWrapper
	{
		*self
	}
}

impl Value for LLVMValueRefWrapper
{
	#[inline(always)]
	fn fromLLVMValueRef(value: LLVMValueRef) -> Self
	{
		debug_assert!(!value.is_null(), "value is null pointer");
		
		LLVMValueRefWrapper(value)
	}
	
	#[inline(always)]
	fn asLLVMValueRef(&self) -> LLVMValueRef
	{
		self.0
	}
}

impl LLVMValueRefWrapper
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
	pub fn asComparison(self) -> ComparisonResultValue
	{
		ComparisonResultValue::fromLLVMValueRefWrapper(self)
	}
}
