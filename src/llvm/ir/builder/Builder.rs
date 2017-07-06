// This file is part of mqtt. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/mqtt/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of mqtt. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/mqtt/master/COPYRIGHT.


pub(crate) trait Builder
{
	#[inline(always)]
	fn dispose(self);
	
	#[inline(always)]
	fn positionAtEndOfBasicBlock(self, basicBlockReference: LLVMBasicBlockRef);
	
	#[inline(always)]
	fn returnVoid(self) -> TerminatorValue;
	
	#[inline(always)]
	fn returnValue<V: Value>(self, value: V) -> TerminatorValue;
	
	#[inline(always)]
	fn unconditionalBranch<'a>(self, to: &Block<'a>) -> TerminatorValue;
	
	#[inline(always)]
	fn conditionalBranch<'a>(self, ifConditional: ComparisonResultValue, thenBlock: &Block<'a>, elseBlock: &Block<'a>) -> TerminatorValue;
	
	#[inline(always)]
	fn switchBranch<'a, 'b, V: Value, I: Iterator<Item=(&'b u8, &'b Block<'a>)> + ExactSizeIterator>(self, context: &Context, switchOnValue: V, defaultBlock: &Block<'a>, caseBlocks: I) -> TerminatorValue
	where 'a : 'b;
	
	#[inline(always)]
	fn phi(self, typeReference: LLVMTypeRefWrapper, name: Option<&CStr>) -> PhiInstructionValue;
	
	#[inline(always)]
	fn bitcastPointerToInt8Pointer(self, context: &Context, pointerValue: PointerValue, name: Option<&CStr>) -> PointerValue;
	
	#[inline(always)]
	fn add<LHS: Value, RHS: Value>(self, leftHandSide: LHS, rightHandSide: RHS, name: Option<&CStr>) -> LLVMValueRefWrapper;
	
	#[inline(always)]
	fn getElementPointerAtArrayIndex<ArrayIndex: Value>(self, arrayPointer: PointerValue, arrayIndexInt64: ArrayIndex, name: Option<&CStr>) -> PointerValue;
	
	#[inline(always)]
	fn getElementPointerAtArrayIndexFieldIndex<ArrayIndex: Value, FieldIndex: Value>(self, arrayPointer: PointerValue, arrayIndexInt64: ArrayIndex, fieldIndexInt32: FieldIndex, name: Option<&CStr>) -> PointerValue;
	
	#[inline(always)]
	fn store(self, context: &Context, into: PointerValue, value: LLVMValueRefWrapper, typeBasedAliasAnalysisNode: Option<TypeBasedAliasAnalysisNode>, alignment: Option<PowerOfTwoThirtyTwoBit>) -> LLVMValueRefWrapper;
	
	#[inline(always)]
	fn load(self, context: &Context, from: PointerValue, typeBasedAliasAnalysisNode: Option<TypeBasedAliasAnalysisNode>, alignment: Option<PowerOfTwoThirtyTwoBit>, name: Option<&CStr>) -> LLVMValueRefWrapper;
	
	#[inline(always)]
	fn integerComparison(self, leftHandSide: LLVMValueRefWrapper, operation: LLVMIntPredicate, rightHandSide: LLVMValueRefWrapper, name: Option<&CStr>) -> ComparisonResultValue;
	
	fn call(self, context: &Context, functionReference: FunctionValue, tailCall: TailCall, functionAttributes: &HashSet<FunctionAttribute>, callingConvention: UsefulLLVMCallConv, returns: Option<&CallParameter>, arguments: &[(LLVMValueRefWrapper, Option<&CallParameter>)], name: Option<&CStr>) -> CallValue;
}

impl Builder for LLVMBuilderRef
{
	#[inline(always)]
	fn dispose(self)
	{
		unsafe { LLVMDisposeBuilder(self) };
	}
	
	#[inline(always)]
	fn positionAtEndOfBasicBlock(self, basicBlockReference: LLVMBasicBlockRef)
	{
		unsafe { LLVMPositionBuilderAtEnd(self, basicBlockReference) }
	}
	
	#[inline(always)]
	fn returnVoid(self) -> TerminatorValue
	{
		TerminatorValue::fromLLVMValueRef(unsafe { LLVMBuildRetVoid(self) })
	}
	
	#[inline(always)]
	fn returnValue<V: Value>(self, value: V) -> TerminatorValue
	{
		TerminatorValue::fromLLVMValueRef(unsafe { LLVMBuildRet(self, value.asLLVMValueRef()) })
	}
	
	#[inline(always)]
	fn unconditionalBranch<'a>(self, to: &Block<'a>) -> TerminatorValue
	{
		TerminatorValue::fromLLVMValueRef(unsafe { LLVMBuildBr(self, to.basicBlockReference) })
	}
	
	#[inline(always)]
	fn conditionalBranch<'a>(self, ifConditional: ComparisonResultValue, thenBlock: &Block<'a>, elseBlock: &Block<'a>) -> TerminatorValue
	{
		TerminatorValue::fromLLVMValueRef(unsafe { LLVMBuildCondBr(self, ifConditional.asLLVMValueRef(), thenBlock.basicBlockReference, elseBlock.basicBlockReference) })
	}
	
	#[inline(always)]
	fn switchBranch<'a, 'b, V: Value, I: Iterator<Item=(&'b u8, &'b Block<'a>)> + ExactSizeIterator>(self, context: &Context, switchOnValue: V, defaultBlock: &Block<'a>, caseBlocks: I) -> TerminatorValue
	where 'a : 'b
	{
		let switchReference = unsafe { LLVMBuildSwitch(self, switchOnValue.asLLVMValueRef(), defaultBlock.basicBlockReference, caseBlocks.len() as u32) };
		
		for (constant, caseBlock) in caseBlocks
		{
			unsafe { LLVMAddCase(switchReference, context.constant(&Constant::integer8BitUnsigned(*constant)).asLLVMValueRef(), caseBlock.basicBlockReference) };
		}
		
		TerminatorValue::fromLLVMValueRef(switchReference)
	}
	
	#[inline(always)]
	fn phi(self, typeReference: LLVMTypeRefWrapper, name: Option<&CStr>) -> PhiInstructionValue
	{
		PhiInstructionValue::fromLLVMValueRef(unsafe { LLVMBuildPhi(self, typeReference.asLLVMTypeRef(), name.nameOrEmptyPointer()) })
	}
	
	#[inline(always)]
	fn bitcastPointerToInt8Pointer(self, context: &Context, pointerValue: PointerValue, name: Option<&CStr>) -> PointerValue
	{
		PointerValue::fromLLVMValueRef(unsafe { LLVMBuildBitCast(self, pointerValue.asLLVMValueRef(), context.typeRef(&LlvmType::int8Pointer()).asLLVMTypeRef(), name.nameOrEmptyPointer()) })
	}
	
	#[inline(always)]
	fn add<LHS: Value, RHS: Value>(self, leftHandSide: LHS, rightHandSide: RHS, name: Option<&CStr>) -> LLVMValueRefWrapper
	{
		LLVMValueRefWrapper::fromLLVMValueRef(unsafe { LLVMBuildAdd(self, leftHandSide.asLLVMValueRef(), rightHandSide.asLLVMValueRef(), name.nameOrEmptyPointer()) })
	}
	
	#[inline(always)]
	fn getElementPointerAtArrayIndex<ArrayIndex: Value>(self, arrayPointer: PointerValue, arrayIndexInt64: ArrayIndex, name: Option<&CStr>) -> PointerValue
	{
		let mut indices =
		[
			arrayIndexInt64.asLLVMValueRef(),
		];
		
		PointerValue::fromLLVMValueRef(unsafe { LLVMBuildInBoundsGEP(self, arrayPointer.asLLVMValueRef(), indices.as_mut_ptr(), indices.len() as u32, name.nameOrEmptyPointer()) })
	}
	
	#[inline(always)]
	fn getElementPointerAtArrayIndexFieldIndex<ArrayIndex: Value, FieldIndex: Value>(self, arrayPointer: PointerValue, arrayIndexInt64: ArrayIndex, fieldIndexInt32: FieldIndex, name: Option<&CStr>) -> PointerValue
	{
		let mut indices =
		[
			arrayIndexInt64.asLLVMValueRef(),
			fieldIndexInt32.asLLVMValueRef(),
		];
		
		PointerValue::fromLLVMValueRef(unsafe { LLVMBuildInBoundsGEP(self, arrayPointer.asLLVMValueRef(), indices.as_mut_ptr(), indices.len() as u32, name.nameOrEmptyPointer()) })
	}
	
	#[inline(always)]
	fn store(self, context: &Context, into: PointerValue, value: LLVMValueRefWrapper, typeBasedAliasAnalysisNode: Option<TypeBasedAliasAnalysisNode>, alignment: Option<PowerOfTwoThirtyTwoBit>) -> LLVMValueRefWrapper
	{
		let instruction = unsafe { LLVMBuildStore(self, value.asLLVMValueRef(), into.asLLVMValueRef()) };
		
		if let Some(ref typeBasedAliasAnalysisNode) = typeBasedAliasAnalysisNode
		{
			unsafe { LLVMSetMetadata(instruction, context.metadataKind_tbaa(), context.typeBasedAliasAnalysisNode(typeBasedAliasAnalysisNode).asLLVMValueRef()) };
		}
		
		if let Some(alignment) = alignment
		{
			unsafe { LLVMSetAlignment(instruction, alignment.as_u32()) };
		}
		
		LLVMValueRefWrapper::fromLLVMValueRef(instruction)
	}
	
	#[inline(always)]
	fn load(self, context: &Context, from: PointerValue, typeBasedAliasAnalysisNode: Option<TypeBasedAliasAnalysisNode>, alignment: Option<PowerOfTwoThirtyTwoBit>, name: Option<&CStr>) -> LLVMValueRefWrapper
	{
		let instruction = unsafe { LLVMBuildLoad(self, from.asLLVMValueRef(), name.nameOrEmptyPointer()) };
		
		if let Some(ref typeBasedAliasAnalysisNode) = typeBasedAliasAnalysisNode
		{
			unsafe { LLVMSetMetadata(instruction, context.metadataKind_tbaa(), context.typeBasedAliasAnalysisNode(typeBasedAliasAnalysisNode).asLLVMValueRef()) };
		}
		
		if let Some(alignment) = alignment
		{
			unsafe { LLVMSetAlignment(instruction, alignment.as_u32()) };
		}
		
		LLVMValueRefWrapper::fromLLVMValueRef(instruction)
	}
	
	#[inline(always)]
	fn integerComparison(self, leftHandSide: LLVMValueRefWrapper, operation: LLVMIntPredicate, rightHandSide: LLVMValueRefWrapper, name: Option<&CStr>) -> ComparisonResultValue
	{
		ComparisonResultValue::fromLLVMValueRef(unsafe{ LLVMBuildICmp(self, operation, leftHandSide.asLLVMValueRef(), rightHandSide.asLLVMValueRef(), name.nameOrEmptyPointer()) })
	}
	
	fn call(self, context: &Context, functionReference: FunctionValue, tailCall: TailCall, functionAttributes: &HashSet<FunctionAttribute>, callingConvention: UsefulLLVMCallConv, returns: Option<&CallParameter>, arguments: &[(LLVMValueRefWrapper, Option<&CallParameter>)], name: Option<&CStr>) -> CallValue
	{
		let mut llvmArguments = Vec::with_capacity(arguments.len());
		
		for argument in arguments.iter()
		{
			llvmArguments.push(argument.0.asLLVMValueRef())
		}
		
		let instruction = unsafe { LLVMBuildCall(self, functionReference.asLLVMValueRef(), llvmArguments.as_mut_ptr(), llvmArguments.len() as u32, name.nameOrEmptyPointer()) };
		
		use self::TailCall::*;
		match tailCall
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
		
		CallValue::fromLLVMValueRef(instruction)
	}
}
