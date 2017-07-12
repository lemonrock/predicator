// This file is part of mqtt. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/mqtt/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of mqtt. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/mqtt/master/COPYRIGHT.


#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FunctionParameterValue(LLVMValueRefWrapper);

impl Value for FunctionParameterValue
{
	#[inline(always)]
	fn fromLLVMValueRef(value: LLVMValueRef) -> Self
	{
		FunctionParameterValue(LLVMValueRefWrapper::fromLLVMValueRef(value))
	}
	
	#[inline(always)]
	fn asLLVMValueRef(&self) -> LLVMValueRef
	{
		self.0.asLLVMValueRef()
	}
}

impl FunctionParameterValue
{
	#[inline(always)]
	pub fn setAlignment(&self, alignment: PowerOfTwoThirtyTwoBit)
	{
		unsafe { LLVMSetParamAlignment(self.asLLVMValueRef(), alignment.as_u32()) };
	}
	
	#[inline(always)]
	pub fn parentFunctionValue(&self) -> FunctionValue
	{
		FunctionValue::fromLLVMValueRef(unsafe { LLVMGetParamParent(self.asLLVMValueRef()) })
	}
	
	#[inline(always)]
	pub fn setName(&self, name: &str)
	{
		let name = CString::new(name.as_bytes()).unwrap();
		unsafe { LLVMSetValueName(self.asLLVMValueRef(), name.as_ptr()) };
	}
}
