// This file is part of mqtt. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/mqtt/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of mqtt. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/mqtt/master/COPYRIGHT.


#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FunctionValue(LLVMValueRefWrapper);

impl Value for FunctionValue
{
	#[inline(always)]
	fn fromLLVMValueRef(value: LLVMValueRef) -> Self
	{
		FunctionValue(LLVMValueRefWrapper::fromLLVMValueRef(value))
	}
	
	#[inline(always)]
	fn asLLVMValueRef(&self) -> LLVMValueRef
	{
		self.0.asLLVMValueRef()
	}
}

impl FunctionValue
{
	#[inline(always)]
	pub fn parameterAt(&self, index: usize) -> Option<FunctionParameterValue>
	{
		if index >= self.numberOfParameters()
		{
			None
		}
		else
		{
			Some(FunctionParameterValue::fromLLVMValueRef(unsafe { LLVMGetParam(self.asLLVMValueRef(), index as u32) }))
		}
	}
	
	#[inline(always)]
	pub fn numberOfParameters(&self) -> usize
	{
		(unsafe { LLVMCountParams(self.asLLVMValueRef()) }) as usize
	}
	
	#[inline(always)]
	pub fn parameters(&self) -> Vec<FunctionParameterValue>
	{
		let numberOfParameters = self.numberOfParameters();
		let mut parameters = Vec::with_capacity(numberOfParameters);
		
		unsafe { LLVMGetParams(self.asLLVMValueRef(), parameters.as_mut_ptr()) };
		unsafe { parameters.set_len(numberOfParameters) };
		unsafe { transmute(parameters) }
	}
	
	#[inline(always)]
	pub fn createBasicBlock<'a, S: Into<String> + Clone>(self, context: &'a Context, name: S) -> BasicBlockBuilder<'a>
	{
		BasicBlockBuilder::createBasicBlock(name, context, self)
	}
}
