// This file is part of mqtt. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/mqtt/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of mqtt. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/mqtt/master/COPYRIGHT.


#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PhiInstructionValue(LLVMValueRefWrapper);

impl Value for PhiInstructionValue
{
	#[inline(always)]
	fn fromLLVMValueRef(value: LLVMValueRef) -> Self
	{
		PhiInstructionValue(LLVMValueRefWrapper::fromLLVMValueRef(value))
	}
	
	#[inline(always)]
	fn asLLVMValueRef(&self) -> LLVMValueRef
	{
		self.0.asLLVMValueRef()
	}
}

impl PhiInstructionValue
{
	#[inline(always)]
	pub fn addPredecessor<'a, V: Value>(self, value: V, block: &Block<'a>) -> Self
	{
		let mut IncomingValues =
		[
			value.asLLVMValueRef(),
		];
		
		let mut IncomingBlocks =
		[
			block.toLLVMBasicBlockRef(),
		];
		
		unsafe { LLVMAddIncoming(self.asLLVMValueRef(), IncomingValues.as_mut_ptr(), IncomingBlocks.as_mut_ptr(), 1); };
		
		self
	}
}
