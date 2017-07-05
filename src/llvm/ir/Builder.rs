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
	#[inline(always)]
	fn positionAtEndOfBasicBlock(&self, basicBlockReference: LLVMBasicBlockRef)
	{
		unsafe { LLVMPositionBuilderAtEnd(self.reference, basicBlockReference) }
	}
	
	#[inline(always)]
	fn returnVoid(&self) -> TerminatorValue
	{
		TerminatorValue::fromLLVMValueRef(unsafe { LLVMBuildRetVoid(self.reference) })
	}
	
	#[inline(always)]
	fn returnValue(&self, value: &Constant) -> TerminatorValue
	{
		TerminatorValue::fromLLVMValueRef(unsafe { LLVMBuildRet(self.reference, self.context.constant(value).asLLVMValueRef()) })
	}
	
	#[inline(always)]
	fn unconditionalBranch(&self, to: LLVMBasicBlockRef) -> TerminatorValue
	{
		TerminatorValue::fromLLVMValueRef(unsafe { LLVMBuildBr(self.reference, to) })
	}
	
	#[inline(always)]
	fn conditionalBranch(&self, ifConditional: ComparisonResultValue, thenBlock: LLVMBasicBlockRef, elseBlock: LLVMBasicBlockRef) -> TerminatorValue
	{
		TerminatorValue::fromLLVMValueRef(unsafe { LLVMBuildCondBr(self.reference, ifConditional.asLLVMValueRef(), thenBlock, elseBlock) })
	}
	
	#[inline(always)]
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
	#[inline(always)]
	fn getElementPointerPointerToStructToPointerToField(&self, arrayPointer: PointerValue, arrayIndex: u64, fieldIndex: u32) -> PointerValue
	{
		let mut indices: [LLVMValueRef; 2] =
		[
			self.context.constant(&Constant::integer64BitUnsigned(arrayIndex)).asLLVMValueRef(),
			self.context.constant(&Constant::integer32BitUnsigned(fieldIndex)).asLLVMValueRef(),
		];
		
		let x = unsafe { LLVMBuildInBoundsGEP(self.reference, arrayPointer.asLLVMValueRef(), indices.as_mut_ptr(), indices.len() as u32, Self::EmptyName()) };
		PointerValue::fromLLVMValueRef(x)
	}
	
	#[inline(always)]
	fn getElementPointerAtArrayIndex(&self, arrayPointer: PointerValue, arrayIndexInt64: LLVMValueRefWrapper) -> PointerValue
	{
		let mut indices =
		[
			arrayIndexInt64.asLLVMValueRef(),
		];
		
		PointerValue::fromLLVMValueRef(unsafe { LLVMBuildInBoundsGEP(self.reference, arrayPointer.asLLVMValueRef(), indices.as_mut_ptr(), indices.len() as u32, Self::EmptyName()) })
	}
	
	#[inline(always)]
	fn load(&self, from: PointerValue, alignment: Option<PowerOfTwoThirtyTwoBit>, typeBasedAliasAnalysisNode: Option<TypeBasedAliasAnalysisNode>) -> LLVMValueRefWrapper
	{
		let instruction = unsafe { LLVMBuildLoad(self.reference, from.asLLVMValueRef(), Self::EmptyName()) };
		
		if let Some(alignment) = alignment
		{
			unsafe { LLVMSetAlignment(instruction, alignment.as_u32()) };
		}
		
		if let Some(ref typeBasedAliasAnalysisNode) = typeBasedAliasAnalysisNode
		{
			unsafe { LLVMSetMetadata(instruction, self.context.metadataKind_tbaa(), self.context.typeBasedAliasAnalysisNode(typeBasedAliasAnalysisNode).asLLVMValueRef()) };
		}
		
		LLVMValueRefWrapper::fromLLVMValueRef(instruction)
	}
	
	#[inline(always)]
	fn store(&self, into: PointerValue, value: LLVMValueRefWrapper, alignment: Option<PowerOfTwoThirtyTwoBit>, typeBasedAliasAnalysisNode: Option<TypeBasedAliasAnalysisNode>) -> LLVMValueRefWrapper
	{
		let instruction = unsafe { LLVMBuildStore(self.reference, value.asLLVMValueRef(), into.asLLVMValueRef()) };
		
		if let Some(alignment) = alignment
		{
			unsafe { LLVMSetAlignment(instruction, alignment.as_u32()) };
		}
		
		if let Some(ref typeBasedAliasAnalysisNode) = typeBasedAliasAnalysisNode
		{
			unsafe { LLVMSetMetadata(instruction, self.context.metadataKind_tbaa(), self.context.typeBasedAliasAnalysisNode(typeBasedAliasAnalysisNode).asLLVMValueRef()) };
		}
		
		LLVMValueRefWrapper::fromLLVMValueRef(instruction)
	}
	
	#[inline(always)]
	fn bitcastPointerToInt8Pointer(&self, pointerValue: PointerValue) -> PointerValue
	{
		PointerValue::fromLLVMValueRef(unsafe { LLVMBuildBitCast(self.reference, pointerValue.asLLVMValueRef(), self.context.typeRef(&LlvmType::int8Pointer()).asLLVMTypeRef(), Self::EmptyName()) })
	}
	
	fn call(&self, context: &Context, functionReference: FunctionValue, builderTailCall: BuilderTailCall, functionAttributes: &HashSet<FunctionAttribute>, callingConvention: UsefulLLVMCallConv, returns: Option<&CallParameter>, arguments: &[(LLVMValueRefWrapper, Option<&CallParameter>)]) -> LLVMValueRefWrapper
	{
		let mut llvmArguments = Vec::with_capacity(arguments.len());
		
		for argument in arguments.iter()
		{
			llvmArguments.push(argument.0.asLLVMValueRef())
		}
		
		let instruction = unsafe { LLVMBuildCall(self.reference, functionReference.asLLVMValueRef(), llvmArguments.as_mut_ptr(), llvmArguments.len() as u32, Self::EmptyName()) };
		
		use self::BuilderTailCall::*;
		match builderTailCall
		{
			Tail => unsafe { LLVMSetTailCall(instruction, 1) },
			MustTail => panic!("MustTail isn't supported as the API isn't clear"),
			NoTail => unsafe { LLVMSetTailCall(instruction, 0) },
		}
		
		for functionAttribute in functionAttributes.iter()
		{
			unsafe { LLVMAddCallSiteAttribute(instruction, LLVMAttributeFunctionIndex, functionAttribute.toReference(context)) };
		}
		
		unsafe { LLVMSetInstructionCallConv(instruction, callingConvention as u32) };
		
		if let Some(callParameter) = returns
		{
			if let Some(ref alignment) = callParameter.alignment
			{
				unsafe { LLVMSetInstrParamAlignment(instruction, LLVMAttributeReturnIndex, alignment.as_u32()) };
			}
			
			for attribute in callParameter.attributes.iter()
			{
				unsafe { LLVMAddCallSiteAttribute(instruction, LLVMAttributeReturnIndex, attribute.toReference(context)) };
			}
		}
		
		let mut attributeIndex = 1;
		for argument in arguments.iter()
		{
			if let Some(callParameter) = argument.1
			{
				if let Some(ref alignment) = callParameter.alignment
				{
					unsafe { LLVMSetInstrParamAlignment(instruction, attributeIndex, alignment.as_u32()) };
				}
				
				for attribute in callParameter.attributes.iter()
				{
					unsafe { LLVMAddCallSiteAttribute(instruction, attributeIndex, attribute.toReference(context)) };
				}
			}
			
			attributeIndex += 1;
		}
		
		LLVMValueRefWrapper::fromLLVMValueRef(instruction)
	}
	
	#[inline(always)]
	fn add(&self, leftHandSide: LLVMValueRefWrapper, rightHandSide: LLVMValueRefWrapper) -> LLVMValueRefWrapper
	{
		LLVMValueRefWrapper::fromLLVMValueRef(unsafe { LLVMBuildAdd(self.reference, leftHandSide.asLLVMValueRef(), rightHandSide.asLLVMValueRef(), Self::EmptyName()) })
	}
	
	#[inline(always)]
	fn integerComparison(&self, operation: LLVMIntPredicate, leftHandSide: LLVMValueRefWrapper, rightHandSide: LLVMValueRefWrapper, name: Option<&CStr>) -> ComparisonResultValue
	{
		ComparisonResultValue::fromLLVMValueRef(unsafe{ LLVMBuildICmp(self.reference, operation, leftHandSide.asLLVMValueRef(), rightHandSide.asLLVMValueRef(), Self::nameOrEmptyName(name)) })
	}
	
	#[inline(always)]
	fn nameOrEmptyName(name: Option<&CStr>) -> *const i8
	{
		if let Some(name) = name { name.as_ptr() } else { Self::EmptyName() }
	}
	
	#[inline(always)]
	fn EmptyName() -> *const i8
	{
		b"\0".as_ptr() as *const _
	}
}
