// This file is part of predicator. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/predicator/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of predicator. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/predicator/master/COPYRIGHT.


pub struct Builder<'a>
{
	pub(crate) reference: LLVMBuilderRef,
	pub(crate) context: &'a Context,
}

impl<'a> Drop for Builder<'a>
{
	fn drop(&mut self)
	{
		unsafe { LLVMDisposeBuilder(self.reference) };
	}
}

impl<'a> Builder<'a>
{
	fn positionAtEndOfBasicBlock(&self, basicBlockReference: LLVMBasicBlockRef)
	{
		unsafe { LLVMPositionBuilderAtEnd(self.reference, basicBlockReference) }
	}
	
	fn returnVoid(&self) -> LLVMValueRef
	{
		unsafe { LLVMBuildRetVoid(self.reference) }
	}
	
	fn returnValue(&self, value: &IntegerConstant) -> LLVMValueRef
	{
		unsafe { LLVMBuildRet(self.reference, self.context.integerConstant(value)) }
	}
	
	fn unconditionalBranch(&self, to: LLVMBasicBlockRef) -> LLVMValueRef
	{
		unsafe { LLVMBuildBr(self.reference, to) }
	}
	
	fn conditionalBranch(&self, ifConditional: LLVMValueRef, thenBlock: LLVMBasicBlockRef, elseBlock: LLVMBasicBlockRef) -> LLVMValueRef
	{
		unsafe { LLVMBuildCondBr(self.reference, ifConditional, thenBlock, elseBlock) }
	}
	
	fn switchBranch(&self, integerValueOrConstant: LLVMValueRef, defaultBlock: LLVMBasicBlockRef, caseBlocks: usize) -> BuilderSwitchInstruction<'a>
	{
		BuilderSwitchInstruction
		{
			switchInstruction: unsafe { LLVMBuildSwitch(self.reference, integerValueOrConstant, defaultBlock, caseBlocks as u32) },
			context: self.context,
		}
	}
	
	/*
		struct MyStruct
		{
			field0,
			field1,
			field2,
		}
		
		let x: &MyStruct = ...
		let z = &x.field2;
		
		LLVM treats pointers to structs as if they were arrays
		
	*/
	fn getElementPointer_PointerToStructToPointerToField(&self, arrayPointer: Pointer, arrayIndex: u64, fieldIndex: u32) -> Pointer
	{
		let mut indices =
		[
			self.context.integerConstant(&IntegerConstant::constantInteger64BitUnsigned(arrayIndex)),
			self.context.integerConstant(&IntegerConstant::constantInteger32BitUnsigned(fieldIndex)),
		];
		
		Pointer(unsafe { LLVMBuildInBoundsGEP(self.reference, arrayPointer.0, indices.as_mut_ptr(), indices.len() as u32, Self::EmptyName()) })
	}
	
	fn getElementPointer_ArrayIndex(&self, arrayPointer: Pointer, arrayIndex: u64) -> Pointer
	{
		let mut indices =
		[
			self.context.integerConstant(&IntegerConstant::constantInteger64BitUnsigned(arrayIndex)),
		];
		
		Pointer(unsafe { LLVMBuildInBoundsGEP(self.reference, arrayPointer.0, indices.as_mut_ptr(), indices.len() as u32, Self::EmptyName()) })
	}
	
	fn load(&self, pointerValue: Pointer, alignment: Option<PowerOfTwoThirtyTwoBit>, tbaaNode: Option<TbaaNode>) -> LLVMValueRef
	{
		let instruction = unsafe { LLVMBuildLoad(self.reference, pointerValue.0, Self::EmptyName()) };
		
		if let Some(alignment) = alignment
		{
			unsafe { LLVMSetAlignment(instruction, alignment.as_u32()) };
		}
		
		if let Some(ref tbaaNode) = tbaaNode
		{
			unsafe { LLVMSetMetadata(instruction, self.context.metadataKind_tbaa(), self.context.tbaaNode(tbaaNode)) };
		}
		
		instruction
	}
	
	fn bitcastPointerToUnsignedCharPointer(&self, pointerValue: Pointer) -> Pointer
	{
		let unsignedCharPointer = LlvmType::pointer(LlvmType::Int8);
		Pointer(unsafe { LLVMBuildBitCast(self.reference, pointerValue.0, self.context.typeRef(&unsignedCharPointer), Self::EmptyName()) })
	}
	
	#[inline(always)]
	fn EmptyName() -> *const i8
	{
		b"\0".as_ptr() as *const _
	}
}
