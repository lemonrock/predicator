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
	
	pub fn returnVoid(&self) -> TerminatorValue
	{
		TerminatorValue::fromLLVMValueRef(unsafe { LLVMBuildRetVoid(self.reference) })
	}
	
	pub fn returnValue(&self, value: &Constant) -> TerminatorValue
	{
		TerminatorValue::fromLLVMValueRef(unsafe { LLVMBuildRet(self.reference, self.context.constant(value).asLLVMValueRef()) })
	}
	
	fn unconditionalBranch(&self, to: LLVMBasicBlockRef) -> TerminatorValue
	{
		TerminatorValue::fromLLVMValueRef(unsafe { LLVMBuildBr(self.reference, to) })
	}
	
	fn conditionalBranch(&self, ifConditional: LLVMValueRef, thenBlock: LLVMBasicBlockRef, elseBlock: LLVMBasicBlockRef) -> TerminatorValue
	{
		TerminatorValue::fromLLVMValueRef(unsafe { LLVMBuildCondBr(self.reference, ifConditional, thenBlock, elseBlock) })
	}
	
	fn switchBranch(&self, integerValueOrConstant: LLVMValueRef, defaultBlock: LLVMBasicBlockRef, caseBlocks: usize) -> BuilderSwitchInstruction<'a>
	{
		BuilderSwitchInstruction
		{
			switchInstruction: TerminatorValue::fromLLVMValueRef(unsafe { LLVMBuildSwitch(self.reference, integerValueOrConstant, defaultBlock, caseBlocks as u32) }),
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
	fn getElementPointer_PointerToStructToPointerToField(&self, arrayPointer: PointerValue, arrayIndex: u64, fieldIndex: u32) -> PointerValue
	{
		let mut indices = Vec::with_capacity(2);
		indices.push(self.context.constant(&Constant::integer64BitUnsigned(arrayIndex)).asLLVMValueRef());
		indices.push(self.context.constant(&Constant::integer32BitUnsigned(fieldIndex)).asLLVMValueRef());
		
//		let mut indices: [LLVMValueRef; 2] =
//		[
//			self.context.constant(&Constant::integer64BitUnsigned(arrayIndex)),
//			self.context.constant(&Constant::integer32BitUnsigned(fieldIndex)),
//		];
		
		println!("HELLO xxx");
		println!("HMMM {:?}", self.reference);
		println!("HMMM2 {:?}", indices[0]);
		println!("HMMM3 {:?}", indices[1]);
		
		let x = CString::new("Hello").unwrap();
		println!("HMMM3 {:?}", x);
		println!("HMMM3 {:?}", arrayPointer.asLLVMValueRef());
		
		
		let x = unsafe { LLVMBuildInBoundsGEP(self.reference, arrayPointer.asLLVMValueRef(), indices.as_mut_ptr(), indices.len() as u32, x.as_ptr()) };
		
		println!("HELLO yyy");
		let z = PointerValue::fromLLVMValueRef(x);
		
		z
	}
	
	fn getElementPointer_ArrayIndex(&self, arrayPointer: PointerValue, arrayIndex: u64) -> PointerValue
	{
		let mut indices =
		[
			self.context.constant(&Constant::integer64BitUnsigned(arrayIndex)).asLLVMValueRef(),
		];
		
		PointerValue::fromLLVMValueRef(unsafe { LLVMBuildInBoundsGEP(self.reference, arrayPointer.asLLVMValueRef(), indices.as_mut_ptr(), indices.len() as u32, Self::EmptyName()) })
	}
	
	fn load(&self, pointerValue: PointerValue, alignment: Option<PowerOfTwoThirtyTwoBit>, typeBasedAliasAnalysisNode: Option<TypeBasedAliasAnalysisNode>) -> LLVMValueRefWrapper
	{
		let instruction = unsafe { LLVMBuildLoad(self.reference, pointerValue.asLLVMValueRef(), Self::EmptyName()) };
		
		if let Some(alignment) = alignment
		{
			unsafe { LLVMSetAlignment(instruction, alignment.as_u32()) };
		}
		
		if let Some(ref typeBasedAliasAnalysisNode) = typeBasedAliasAnalysisNode
		{
			unsafe { LLVMSetMetadata(instruction, self.context.metadataKind_tbaa(), self.context.typeBasedAliasAnalysisNode(typeBasedAliasAnalysisNode).asLLVMValueRef()) };
		}
		
		LLVMValueRefWrapper(instruction)
	}
	
	fn bitcastPointerToUnsignedCharPointer(&self, pointerValue: PointerValue) -> PointerValue
	{
		let unsignedCharPointer = LlvmType::pointer(LlvmType::Int8);
		PointerValue::fromLLVMValueRef(unsafe { LLVMBuildBitCast(self.reference, pointerValue.asLLVMValueRef(), self.context.typeRef(&unsignedCharPointer).asLLVMTypeRef(), Self::EmptyName()) })
	}
	
	#[inline(always)]
	fn EmptyName() -> *const i8
	{
		b"\0".as_ptr() as *const _
	}
}
