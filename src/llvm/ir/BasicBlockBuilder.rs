// This file is part of predicator. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/predicator/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of predicator. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/predicator/master/COPYRIGHT.


pub struct BasicBlockBuilder<'a>
{
	context: &'a Context,
	basicBlockReference: LLVMBasicBlockRef,
	builder: Builder,
}

impl<'a> BasicBlockBuilder<'a>
{
	fn new(context: &'a Context, basicBlockReference: LLVMBasicBlockRef) -> Self
	{
		let builder = context.builder();
		
		let this = Self
		{
			context: context,
			basicBlockReference: basicBlockReference,
			builder: builder,
		};
		
		this.builder.positionAtEndOfBasicBlock(this.basicBlockReference);
		
		this
	}
	
	/// It is not clear if this is valid to do whilst a builder is active
	pub fn moveBefore(&self, before: LLVMBasicBlockRef)
	{
		unsafe { LLVMMoveBasicBlockBefore(self.basicBlockReference, before) }
	}
	
	/// It is not clear if this is valid to do whilst a builder is active
	pub fn moveAfter(&self, after: LLVMBasicBlockRef)
	{
		unsafe { LLVMMoveBasicBlockAfter(self.basicBlockReference, after) }
	}
	
	pub fn returnVoid(self)
	{
		self.builder.returnVoid();
	}
	
	pub fn returnTrue(self)
	{
		self.builder.returnValue(self.True());
	}
	
	pub fn returnFalse(self)
	{
		self.builder.returnValue(self.False());
	}
	
	#[inline(always)]
	fn True(&self) -> LLVMValueRef
	{
		self.context.integerConstant(&IntegerConstant::True)
	}
	
	#[inline(always)]
	fn False(&self) -> LLVMValueRef
	{
		self.context.integerConstant(&IntegerConstant::False)
	}
}
