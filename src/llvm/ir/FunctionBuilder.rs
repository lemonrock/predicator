// This file is part of predicator. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/predicator/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of predicator. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/predicator/master/COPYRIGHT.


pub struct FunctionBuilder<'a>
{
	context: &'a Context,
	functionReference: LLVMValueRef,
}

impl<'a> FunctionBuilder<'a>
{
	#[inline(always)]
	pub fn appendBasicBlock(&'a mut self, name: &str) -> BasicBlockBuilder<'a>
	{
		let name = CString::new(name.as_bytes()).unwrap();
		let basicBlockReference = unsafe { LLVMAppendBasicBlockInContext(self.context.reference, self.functionReference, name.as_ptr()) };
		BasicBlockBuilder::new(self.context, basicBlockReference)
	}
	
	#[inline(always)]
	pub fn insertBasicBlockBefore(&'a mut self, name: &str, before: LLVMBasicBlockRef) -> BasicBlockBuilder<'a>
	{
		let name = CString::new(name.as_bytes()).unwrap();
		let basicBlockReference = unsafe { LLVMInsertBasicBlockInContext(self.context.reference, before, name.as_ptr()) };
		BasicBlockBuilder::new(self.context, basicBlockReference)
	}
}
