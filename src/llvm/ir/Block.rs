// This file is part of predicator. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/predicator/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of predicator. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/predicator/master/COPYRIGHT.


pub struct Block<'a>
{
	context: &'a Context,
	basicBlockReference: LLVMBasicBlockRef,
	pub builderReference: LLVMBuilderRef,
}

impl<'a> Drop for Block<'a>
{
	#[inline(always)]
	fn drop(&mut self)
	{
		self.builderReference.dispose();
	}
}

impl<'a> ToLLVMBasicBlockRef for Block<'a>
{
	#[inline(always)]
	fn toLLVMBasicBlockRef(&self) -> LLVMBasicBlockRef
	{
		self.basicBlockReference
	}
}

impl<'a> Block<'a>
{
	pub const PointerAlignment: PowerOfTwoThirtyTwoBit = PowerOfTwoThirtyTwoBit::_8;
	
	#[inline(always)]
	pub fn builderReference(&self) -> LLVMBuilderRef
	{
		self.builderReference
	}
	
	#[inline(always)]
	pub(crate) fn create(context: &'a Context, functionValue: FunctionValue) -> Block<'a>
	{
		let basicBlockReference = unsafe { LLVMAppendBasicBlockInContext(context.reference, functionValue.asLLVMValueRef(), b"\0".as_ptr() as *const _) };
		
		let builderReference = context.builder();
		
		builderReference.positionAtEndOfBasicBlock(basicBlockReference);
		
		Self
		{
			context,
			basicBlockReference,
			builderReference,
		}
	}
	
	#[inline(always)]
	pub fn returnVoid(&self)
	{
		self.builderReference.returnVoid();
	}
	
	#[inline(always)]
	pub fn returnTrue(&self)
	{
		self.builderReference.returnValue(self.booleanTrue());
	}
	
	#[inline(always)]
	pub fn returnFalse(&self)
	{
		self.builderReference.returnValue(self.booleanFalse());
	}
	
	#[inline(always)]
	pub fn unconditionalBranch<ToBlockReference: ToLLVMBasicBlockRef>(&self, to: &ToBlockReference)
	{
		self.builderReference.unconditionalBranch(to);
	}
	
	#[inline(always)]
	pub fn unconditionalBranchToChild(&self, blockFactory: &BlockFactory<'a>) -> Block<'a>
	{
		let child = blockFactory.child();
		self.unconditionalBranch(&child);
		child
	}
	
	#[inline(always)]
	pub fn conditionalBranch<ThenToBlockReference: ToLLVMBasicBlockRef, ElseToBlockReference: ToLLVMBasicBlockRef>(&self, ifCondition: ComparisonResultValue, thenBlock: &ThenToBlockReference, elseBlock: &ElseToBlockReference)
	{
		self.builderReference.conditionalBranch(ifCondition, thenBlock, elseBlock);
	}
	
	#[inline(always)]
	pub fn switchBranch<V: ToLLVMValueRefWrapper, DefaultToBlockReference: ToLLVMBasicBlockRef, CaseToBlockReference: ToLLVMBasicBlockRef>(&self, switchOnValue: V, defaultBlock: &DefaultToBlockReference, caseBlocks: &[(u8, CaseToBlockReference)])
	{
		self.builderReference.switchBranch(self.context, self.toLLVMValueRefWrapper(switchOnValue), defaultBlock, caseBlocks);
	}
	
	#[inline(always)]
	pub fn phi(&self, typeRef: LLVMTypeRef) -> PhiInstructionValue
	{
		self.builderReference.phi(typeRef)
	}
	
	#[inline(always)]
	pub fn bitcastPointerTo(&self, pointerValue: PointerValue, toTypeRef: LLVMTypeRef) -> PointerValue
	{
		self.builderReference.bitcastPointerTo(pointerValue, toTypeRef)
	}
	
	#[inline(always)]
	pub fn arithmetic<LHS: ToLLVMValueRefWrapper, RHS: ToLLVMValueRefWrapper>(&self, leftHandSide: LHS, operation: BinaryArithmetic, rightHandSide: RHS) -> LLVMValueRefWrapper
	{
		operation.operate(self.builderReference, self.toLLVMValueRefWrapper(leftHandSide), self.toLLVMValueRefWrapper(rightHandSide))
	}
	
	#[inline(always)]
	pub fn invert<V: ToLLVMValueRefWrapper>(&self, operation: UnaryArithmetic, value: V) -> LLVMValueRefWrapper
	{
		operation.operate(self.builderReference, self.toLLVMValueRefWrapper(value))
	}
	
	#[inline(always)]
	pub fn increment<LHS: Value, RHS: Value>(&self, original: LHS, increment: RHS) -> LLVMValueRefWrapper
	{
		self.arithmetic(original, BinaryArithmetic::Add, increment)
	}
	
	#[inline(always)]
	pub fn getElementPointerAtArrayIndex<V: ToLLVMValueRefWrapper>(&self, pointerValue: PointerValue, arrayIndexInt64: V) -> PointerValue
	{
		self.builderReference.getElementPointerAtArrayIndex(pointerValue, arrayIndexInt64.toLLVMValueRefWrapper(self.context))
	}
	
	#[inline(always)]
	pub fn pointerToStructField(&self, pointerValue: PointerValue, fieldIndex: u32) -> PointerValue
	{
		self.builderReference.getElementPointerAtArrayIndexFieldIndex(pointerValue, self.context.constantZeroInteger64BitUnsigned(), self.integer32BitUnsigned(fieldIndex))
	}
	
	#[inline(always)]
	pub fn storeValue(&self, into: PointerValue, value: LLVMValueRefWrapper, path: &PathTypeBasedAliasAnalysisNode, alignment: PowerOfTwoThirtyTwoBit) -> LLVMValueRefWrapper
	{
		self.builderReference.store(self.metadataKind_tbaa(), into, value, path.asLLVMValueRef(), Some(alignment))
	}
	
	#[inline(always)]
	pub fn loadValue(&self, arrayPointer: PointerValue, path: &PathTypeBasedAliasAnalysisNode, alignment: PowerOfTwoThirtyTwoBit) -> LLVMValueRefWrapper
	{
		self.builderReference.load(self.metadataKind_tbaa(), arrayPointer, path.asLLVMValueRef(), Some(alignment))
	}
	
	#[inline(always)]
	pub fn comparison<LHS: ToLLVMValueRefWrapper, RHS: ToLLVMValueRefWrapper>(&self, leftHandSide: LHS, predicate: LLVMIntPredicate, rightHandSide: RHS) -> ComparisonResultValue
	{
		self.builderReference.integerComparison(self.toLLVMValueRefWrapper(leftHandSide), predicate, self.toLLVMValueRefWrapper(rightHandSide))
	}
	
	#[inline(always)]
	pub fn ifInteger<LHS: ToLLVMValueRefWrapper, RHS: ToLLVMValueRefWrapper>(&self, leftHandSide: LHS, predicate: LLVMIntPredicate, rightHandSide: RHS, blockFactory: &BlockFactory<'a>) -> (ComparisonResultValue, Block<'a>, Block<'a>)
	{
		let thenBlock = blockFactory.child();
		let elseBlock = blockFactory.child();
		(self.comparison(leftHandSide, predicate,rightHandSide), thenBlock, elseBlock)
	}
	
	#[inline(always)]
	pub fn ifFalseCarryOn<TrueToBlockReference: ToLLVMBasicBlockRef>(&self, isTrue: ComparisonResultValue, ifTrueBlock: &TrueToBlockReference, blockFactory: &BlockFactory<'a>) -> Block<'a>
	{
		let carryOnBlock = blockFactory.child();
		self.conditionalBranch(isTrue, ifTrueBlock, &carryOnBlock);
		carryOnBlock
	}
	
	#[inline(always)]
	pub fn loadPointer(&self, arrayPointer: PointerValue, pointerPath: &PointerPathTypeBasedAliasAnalysisNode) -> PointerValue
	{
		PointerValue::fromLLVMValueRefWrapper(self.loadValue(arrayPointer, pointerPath.asPathTypeBasedAliasAnalysisNode(), Self::PointerAlignment))
	}
	
	#[inline(always)]
	pub fn loadValueFromReferencedStructField(&self, pointerValue: PointerValue, fieldIndex: u32, path: &PathTypeBasedAliasAnalysisNode, valueAlignment: PowerOfTwoThirtyTwoBit) -> (LLVMValueRefWrapper, PointerValue)
	{
		let arrayPointer = self.pointerToStructField(pointerValue, fieldIndex);
		let loadedValue = self.loadValue(arrayPointer, path, valueAlignment);
		(loadedValue, arrayPointer)
	}
	
	#[inline(always)]
	pub fn loadPointerFromReferencedStructField(&self, pointerValue: PointerValue, fieldIndex: u32, pointerPath: &PointerPathTypeBasedAliasAnalysisNode) -> (PointerValue, PointerValue)
	{
		let arrayPointer = self.pointerToStructField(pointerValue, fieldIndex);
		let loadedPointer = self.loadPointer(arrayPointer, pointerPath);
		(loadedPointer, arrayPointer)
	}
	
	#[inline(always)]
	fn metadataKind_tbaa(&self) -> u32
	{
		self.context.metadataKind_tbaa()
	}
	
	#[inline(always)]
	fn booleanTrue(&self) -> LLVMValueRef
	{
		self.context.constantBooleanTrue()
	}
	
	#[inline(always)]
	fn booleanFalse(&self) -> LLVMValueRef
	{
		self.context.constantBooleanFalse()
	}
	
	#[inline(always)]
	fn integer32BitUnsigned(&self, value: u32) -> LLVMValueRef
	{
		self.context.constantInteger32BitUnsigned(value)
	}
	
	
	
	
	#[inline(always)]
	fn toLLVMValueRefWrapper<V: ToLLVMValueRefWrapper>(&self, value: V) -> LLVMValueRefWrapper
	{
		value.toLLVMValueRefWrapper(self.context)
	}
}
