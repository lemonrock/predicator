// This file is part of mqtt. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/mqtt/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of mqtt. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/mqtt/master/COPYRIGHT.


pub(crate) trait Builder
{
	#[inline(always)]
	fn dispose(self);
	
	#[inline(always)]
	fn positionAtEndOfBasicBlock<ToBlockReference: ToLLVMBasicBlockRef>(self, basicBlockReference: ToBlockReference);
	
	#[inline(always)]
	fn returnVoid(self);
	
	#[inline(always)]
	fn returnValue<V: Value>(self, value: V);
	
	#[inline(always)]
	fn unconditionalBranch<ToBlockReference: ToLLVMBasicBlockRef>(self, to: &ToBlockReference);
	
	#[inline(always)]
	fn conditionalBranch<ThenToBlockReference: ToLLVMBasicBlockRef, ElseToBlockReference: ToLLVMBasicBlockRef>(self, ifConditional: ComparisonResultValue, thenBlock: &ThenToBlockReference, elseBlock: &ElseToBlockReference);
	
	#[inline(always)]
	fn switchBranch<V: Value, DefaultToBlockReference: ToLLVMBasicBlockRef, CaseToBlockReference: ToLLVMBasicBlockRef>(self, context: &Context, switchOnValue: V, defaultBlock: &DefaultToBlockReference, caseBlocks: &[(u8, CaseToBlockReference)]);
	
	#[inline(always)]
	fn phi(self, typeReference: LLVMTypeRef) -> PhiInstructionValue;
	
	#[inline(always)]
	fn bitcastPointerToInt8Pointer(self, context: &Context, pointerValue: PointerValue) -> PointerValue;
	
	#[inline(always)]
	fn getElementPointerAtArrayIndex<ArrayIndex: Value>(self, arrayPointer: PointerValue, arrayIndexInt64: ArrayIndex) -> PointerValue;
	
	#[inline(always)]
	fn getElementPointerAtArrayIndexFieldIndex<ArrayIndex: Value, FieldIndex: Value>(self, arrayPointer: PointerValue, arrayIndexInt64: ArrayIndex, fieldIndexInt32: FieldIndex) -> PointerValue;
	
	#[inline(always)]
	fn store(self, context: &Context, into: PointerValue, value: LLVMValueRefWrapper, typeBasedAliasAnalysisNode: Option<TypeBasedAliasAnalysisNode>, alignment: Option<PowerOfTwoThirtyTwoBit>) -> LLVMValueRefWrapper;
	
	#[inline(always)]
	fn load(self, context: &Context, from: PointerValue, typeBasedAliasAnalysisNode: Option<TypeBasedAliasAnalysisNode>, alignment: Option<PowerOfTwoThirtyTwoBit>) -> LLVMValueRefWrapper;
	
	#[inline(always)]
	fn integerComparison(self, leftHandSide: LLVMValueRefWrapper, operation: LLVMIntPredicate, rightHandSide: LLVMValueRefWrapper) -> ComparisonResultValue;
	
	fn call(self, context: &Context, functionReference: FunctionValue, tailCall: TailCall, functionAttributes: &HashSet<FunctionAttribute>, callingConvention: UsefulLLVMCallConv, returns: Option<&CallParameter>, arguments: &[(LLVMValueRef, Option<&CallParameter>)]) -> CallValue;
}

impl Builder for LLVMBuilderRef
{
	#[inline(always)]
	fn dispose(self)
	{
		unsafe { LLVMDisposeBuilder(self) };
	}
	
	#[inline(always)]
	fn positionAtEndOfBasicBlock<ToBlockReference: ToLLVMBasicBlockRef>(self, basicBlockReference: ToBlockReference)
	{
		unsafe { LLVMPositionBuilderAtEnd(self, basicBlockReference.toLLVMBasicBlockRef()) }
	}
	
	#[inline(always)]
	fn returnVoid(self)
	{
		unsafe { LLVMBuildRetVoid(self) };
	}
	
	#[inline(always)]
	fn returnValue<V: Value>(self, value: V)
	{
		unsafe { LLVMBuildRet(self, value.asLLVMValueRef()) };
	}
	
	#[inline(always)]
	fn unconditionalBranch<ToBlockReference: ToLLVMBasicBlockRef>(self, to: &ToBlockReference)
	{
		unsafe { LLVMBuildBr(self, to.toLLVMBasicBlockRef()) };
	}
	
	#[inline(always)]
	fn conditionalBranch<ThenToBlockReference: ToLLVMBasicBlockRef, ElseToBlockReference: ToLLVMBasicBlockRef>(self, ifConditional: ComparisonResultValue, thenBlock: &ThenToBlockReference, elseBlock: &ElseToBlockReference)
	{
		unsafe { LLVMBuildCondBr(self, ifConditional.asLLVMValueRef(), thenBlock.toLLVMBasicBlockRef(), elseBlock.toLLVMBasicBlockRef()) };
	}
	
	#[inline(always)]
	fn switchBranch<V: Value, DefaultToBlockReference: ToLLVMBasicBlockRef, CaseToBlockReference: ToLLVMBasicBlockRef>(self, context: &Context, switchOnValue: V, defaultBlock: &DefaultToBlockReference, caseBlocks: &[(u8, CaseToBlockReference)])
	{
		let switchReference = unsafe { LLVMBuildSwitch(self, switchOnValue.asLLVMValueRef(), defaultBlock.toLLVMBasicBlockRef(), caseBlocks.len() as u32) };
		
		for &(ref constant, ref caseBlock) in caseBlocks
		{
			let constantValue = context.constantInteger8BitUnsigned(*constant);
			unsafe { LLVMAddCase(switchReference, constantValue, caseBlock.toLLVMBasicBlockRef()) };
		}
	}
	
	#[inline(always)]
	fn phi(self, typeReference: LLVMTypeRef) -> PhiInstructionValue
	{
		PhiInstructionValue::fromLLVMValueRef(unsafe { LLVMBuildPhi(self, typeReference, emptyName!()) })
	}
	
	#[inline(always)]
	fn bitcastPointerToInt8Pointer(self, context: &Context, pointerValue: PointerValue) -> PointerValue
	{
		PointerValue::fromLLVMValueRef(unsafe { LLVMBuildBitCast(self, pointerValue.asLLVMValueRef(), context.typeRef(&LlvmType::int8Pointer()).asLLVMTypeRef(), emptyName!()) })
	}
	
	#[inline(always)]
	fn getElementPointerAtArrayIndex<ArrayIndex: Value>(self, arrayPointer: PointerValue, arrayIndexInt64: ArrayIndex) -> PointerValue
	{
		let mut indices =
		[
			arrayIndexInt64.asLLVMValueRef(),
		];
		
		PointerValue::fromLLVMValueRef(unsafe { LLVMBuildInBoundsGEP(self, arrayPointer.asLLVMValueRef(), indices.as_mut_ptr(), indices.len() as u32, emptyName!()) })
	}
	
	#[inline(always)]
	fn getElementPointerAtArrayIndexFieldIndex<ArrayIndex: Value, FieldIndex: Value>(self, arrayPointer: PointerValue, arrayIndexInt64: ArrayIndex, fieldIndexInt32: FieldIndex) -> PointerValue
	{
		let mut indices =
		[
			arrayIndexInt64.asLLVMValueRef(),
			fieldIndexInt32.asLLVMValueRef(),
		];
		
		PointerValue::fromLLVMValueRef(unsafe { LLVMBuildInBoundsGEP(self, arrayPointer.asLLVMValueRef(), indices.as_mut_ptr(), indices.len() as u32, emptyName!()) })
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
	fn load(self, context: &Context, from: PointerValue, typeBasedAliasAnalysisNode: Option<TypeBasedAliasAnalysisNode>, alignment: Option<PowerOfTwoThirtyTwoBit>) -> LLVMValueRefWrapper
	{
		let instruction = unsafe { LLVMBuildLoad(self, from.asLLVMValueRef(), emptyName!()) };
		
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
	fn integerComparison(self, leftHandSide: LLVMValueRefWrapper, operation: LLVMIntPredicate, rightHandSide: LLVMValueRefWrapper) -> ComparisonResultValue
	{
		ComparisonResultValue::fromLLVMValueRef(unsafe{ LLVMBuildICmp(self, operation, leftHandSide.asLLVMValueRef(), rightHandSide.asLLVMValueRef(), emptyName!()) })
	}
	
	fn call(self, context: &Context, functionReference: FunctionValue, tailCall: TailCall, functionAttributes: &HashSet<FunctionAttribute>, callingConvention: UsefulLLVMCallConv, returns: Option<&CallParameter>, arguments: &[(LLVMValueRef, Option<&CallParameter>)]) -> CallValue
	{
		let mut llvmArguments = Vec::with_capacity(arguments.len());
		
		for argument in arguments.iter()
		{
			llvmArguments.push(argument.0.asLLVMValueRef())
		}
		
		let instruction = unsafe { LLVMBuildCall(self, functionReference.asLLVMValueRef(), llvmArguments.as_mut_ptr(), llvmArguments.len() as u32, emptyName!()) };
		
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
