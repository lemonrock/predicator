// This file is part of predicator. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/predicator/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of predicator. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/predicator/master/COPYRIGHT.


pub struct BasicBlockBuilder<'a>
{
	context: &'a Context,
	functionReference: LLVMValueRef,
	basicBlockReference: LLVMBasicBlockRef,
	builder: Builder,
}

impl<'a> BasicBlockBuilder<'a>
{
	#[inline(always)]
	fn createBasicBlock(name: &str, context: &'a Context, functionReference: LLVMValueRef) -> BasicBlockBuilder<'a>
	{
		let name = CString::new(name.as_bytes()).unwrap();
		let basicBlockReference = unsafe { LLVMAppendBasicBlockInContext(context.reference, functionReference, name.as_ptr()) };
		
		let builder = context.builder();
		
		let this = Self
		{
			context: context,
			functionReference: functionReference,
			basicBlockReference: basicBlockReference,
			builder: builder,
		};
		
		this.builder.positionAtEndOfBasicBlock(this.basicBlockReference);
		
		this
	}
	
	#[inline(always)]
	pub fn newBasicBlock(&self, to: &str) -> BasicBlockBuilder<'a>
	{
		Self::createBasicBlock(to, self.context, self.functionReference)
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
	
	// We need some name management
	
	pub fn unconditionalBranchWithCreation(self, to: &str) -> BasicBlockBuilder<'a>
	{
		let to = self.newBasicBlock(to);
		self.builder.unconditionalBranch(to.basicBlockReference);
		to
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
