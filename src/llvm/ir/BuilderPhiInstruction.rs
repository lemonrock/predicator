// This file is part of predicator. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/predicator/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of predicator. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/predicator/master/COPYRIGHT.


pub struct BuilderPhiInstruction<'a>
{
	phiInstruction: LLVMValueRefWrapper,
	context: &'a Context
}

impl<'a> BuilderPhiInstruction<'a>
{
	#[inline(always)]
	pub fn addPredecessor<V: ToLLVMValueRefWrapper>(&self, value: &V, block: &BasicBlockBuilder<'a>)
	{
		let mut IncomingValues =
		[
			value.toLLVMValueRefWrapper(self.context).asLLVMValueRef(),
		];
		
		let mut IncomingBlocks =
		[
			block.basicBlockReference,
		];
		
		unsafe { LLVMAddIncoming(self.phiInstruction.asLLVMValueRef(), IncomingValues.as_mut_ptr(), IncomingBlocks.as_mut_ptr(), 1); }
	}
	
	#[inline(always)]
	pub fn value(&self) -> LLVMValueRefWrapper
	{
		self.phiInstruction
	}
}
