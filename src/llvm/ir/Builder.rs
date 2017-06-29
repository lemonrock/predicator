// This file is part of predicator. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/predicator/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of predicator. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/predicator/master/COPYRIGHT.


pub struct Builder
{
	pub(crate) reference: LLVMBuilderRef,
	pub(crate) parentDropWrapper: Rc<ContextDropWrapper>,
}

impl Drop for Builder
{
	fn drop(&mut self)
	{
		unsafe { LLVMDisposeBuilder(self.reference) };
	}
}

impl Builder
{
	fn positionAtEndOfBasicBlock(&self, basicBlockReference: LLVMBasicBlockRef)
	{
		unsafe { LLVMPositionBuilderAtEnd(self.reference, basicBlockReference) }
	}
	
	fn returnVoid(&self) -> Instruction
	{
		Instruction(unsafe { LLVMBuildRetVoid(self.reference) })
	}
	
	fn returnValue(&self, value: LLVMValueRef) -> Instruction
	{
		Instruction(unsafe { LLVMBuildRet(self.reference, value) })
	}
	
	fn unconditionalBranch(&self, to: LLVMBasicBlockRef) -> Instruction
	{
		Instruction(unsafe { LLVMBuildBr(self.reference, to) })
	}
	
	fn conditionalBranch(&self, ifConditional: LLVMValueRef, thenBlock: LLVMBasicBlockRef, elseBlock: LLVMBasicBlockRef) -> Instruction
	{
		Instruction(unsafe { LLVMBuildCondBr(self.reference, ifConditional, thenBlock, elseBlock) })
	}
	
	fn switchBranch(&self, integerValueOrConstant: LLVMValueRef, defaultBlock: LLVMBasicBlockRef, caseBlocks: usize) -> SwitchInstruction
	{
		SwitchInstruction(unsafe { LLVMBuildSwitch(self.reference, integerValueOrConstant, defaultBlock, caseBlocks as u32) })
	}
}
