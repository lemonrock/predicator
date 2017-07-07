// This file is part of predicator. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/predicator/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of predicator. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/predicator/master/COPYRIGHT.


pub struct Block<'a>
{
	context: &'a Context,
	functionValue: FunctionValue,
	pub(crate) basicBlockReference: LLVMBasicBlockRef,
	builderReference: LLVMBuilderRef,
	name: String,
	nextChild: Cell<u64>,
}

impl<'a> Drop for Block<'a>
{
	fn drop(&mut self)
	{
		self.builderReference.dispose();
	}
}

impl<'a> Block<'a>
{
	pub const PointerAlignment: PowerOfTwoThirtyTwoBit = PowerOfTwoThirtyTwoBit::_8;
	
	#[inline(always)]
	pub(crate) fn create<S: Into<String> + Clone>(name: S, context: &'a Context, functionValue: FunctionValue) -> Block<'a>
	{
		let name = name.into();
		let cName = CString::new(name.as_bytes()).unwrap();
		let basicBlockReference = unsafe { LLVMAppendBasicBlockInContext(context.reference, functionValue.asLLVMValueRef(), cName.as_ptr()) };
		
		let builderReference = context.builder();
		
		builderReference.positionAtEndOfBasicBlock(basicBlockReference);
		
		Self
		{
			context,
			functionValue,
			basicBlockReference,
			builderReference,
			name,
			nextChild: Cell::new(0),
		}
	}
	
	#[inline(always)]
	pub fn child(&self) -> Block<'a>
	{
		let nextChild = self.nextChild.get();
		let name = format!("{}.{}", &self.name, nextChild);
		self.nextChild.set(nextChild + 1);
		Self::create(name, self.context, self.functionValue)
	}
	
	#[inline(always)]
	pub fn parameterAt(&self, index: usize) -> Option<FunctionParameterValue>
	{
		self.functionValue.parameterAt(index)
	}
	
	#[inline(always)]
	pub fn parameterAtAsPointer(&self, index: usize) -> Option<PointerValue>
	{
		self.parameterAt(index).map(|functionParameterValue| PointerValue::fromLLVMValueRef(functionParameterValue.asLLVMValueRef()) )
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
	pub fn unconditionalBranch(&self, to: &Block<'a>)
	{
		self.builderReference.unconditionalBranch(to);
	}
	
	#[inline(always)]
	pub fn unconditionalBranchToChild(&self) -> Block<'a>
	{
		let child = self.child();
		self.unconditionalBranch(&child);
		child
	}
	
	#[inline(always)]
	pub fn conditionalBranch(&self, ifCondition: ComparisonResultValue, thenBlock: &Block<'a>, elseBlock: &Block<'a>)
	{
		self.builderReference.conditionalBranch(ifCondition, thenBlock, elseBlock);
	}
	
	#[inline(always)]
	pub fn switchBranch<V: ToLLVMValueRefWrapper>(&self, switchOnValue: V, defaultBlock: &Block<'a>, caseBlocks: &[(u8, LLVMBasicBlockRef)])
	{
		self.builderReference.switchBranch(self.context, self.toLLVMValueRefWrapper(switchOnValue), defaultBlock, caseBlocks);
	}
	
	#[inline(always)]
	pub fn phi(&self, llvmType: &LlvmType) -> PhiInstructionValue
	{
		self.builderReference.phi(self.typeRef(llvmType), None)
	}
	
	#[inline(always)]
	pub fn bitcastPointerToInt8Pointer(&self, pointerValue: PointerValue) -> PointerValue
	{
		self.builderReference.bitcastPointerToInt8Pointer(self.context, pointerValue, None)
	}
	
	#[inline(always)]
	pub fn arithmetic<LHS: ToLLVMValueRefWrapper, RHS: ToLLVMValueRefWrapper>(&self, leftHandSide: LHS, operation: BinaryArithmetic, rightHandSide: RHS) -> LLVMValueRefWrapper
	{
		operation.operate(self.builderReference, self.toLLVMValueRefWrapper(leftHandSide), self.toLLVMValueRefWrapper(rightHandSide), None)
	}
	
	#[inline(always)]
	pub fn invert<V: ToLLVMValueRefWrapper>(&self, operation: UnaryArithmetic, value: V) -> LLVMValueRefWrapper
	{
		operation.operate(self.builderReference, self.toLLVMValueRefWrapper(value), None)
	}
	
	#[inline(always)]
	pub fn increment<V: Value + Copy>(&self, original: V, increment: u64) -> LLVMValueRefWrapper
	{
		self.arithmetic(original, BinaryArithmetic::Add, increment)
	}
	
	#[inline(always)]
	pub fn getElementPointerAtArrayIndex<V: ToLLVMValueRefWrapper>(&self, pointerValue: PointerValue, arrayIndexInt64: V) -> PointerValue
	{
		self.builderReference.getElementPointerAtArrayIndex(pointerValue, arrayIndexInt64.toLLVMValueRefWrapper(self.context), None)
	}
	
	#[inline(always)]
	pub fn pointerToStructField(&self, pointerValue: PointerValue, fieldIndex: u32) -> PointerValue
	{
		self.builderReference.getElementPointerAtArrayIndexFieldIndex(pointerValue, self.integer64BitUnsigned(0), self.integer32BitUnsigned(fieldIndex), None)
	}
	
	#[inline(always)]
	pub fn storeValue(&self, into: PointerValue, value: LLVMValueRefWrapper, offsetIntoBaseType: u64, from: TypeBasedAliasAnalysisNode, to: TypeBasedAliasAnalysisNode, alignment: PowerOfTwoThirtyTwoBit) -> LLVMValueRefWrapper
	{
		self.builderReference.store(self.context, into, value, Self::path(offsetIntoBaseType, from, to), Some(alignment))
	}
	
	#[inline(always)]
	pub fn loadValue(&self, arrayPointer: PointerValue, offsetIntoBaseType: u64, from: TypeBasedAliasAnalysisNode, to: TypeBasedAliasAnalysisNode, alignment: PowerOfTwoThirtyTwoBit) -> LLVMValueRefWrapper
	{
		self.builderReference.load(self.context, arrayPointer, Self::path(offsetIntoBaseType, from, to), Some(alignment), None)
	}
	
	#[inline(always)]
	pub fn comparison<LHS: ToLLVMValueRefWrapper, RHS: ToLLVMValueRefWrapper>(&self, leftHandSide: LHS, predicate: LLVMIntPredicate, rightHandSide: RHS) -> ComparisonResultValue
	{
		self.builderReference.integerComparison(self.toLLVMValueRefWrapper(leftHandSide), predicate, self.toLLVMValueRefWrapper(rightHandSide), None)
	}
	
	#[inline(always)]
	pub fn ifInteger<LHS: ToLLVMValueRefWrapper, RHS: ToLLVMValueRefWrapper>(&self, leftHandSide: LHS, predicate: LLVMIntPredicate, rightHandSide: RHS) -> (ComparisonResultValue, Block<'a>, Block<'a>)
	{
		let thenBlock = self.child();
		let elseBlock = self.child();
		(self.comparison(leftHandSide, predicate,rightHandSide), thenBlock, elseBlock)
	}
	
	#[inline(always)]
	pub fn tailCallMemCpy64(&self, functionReference: FunctionValue, fromInt8PointerValue: PointerValue, toInt8PointerValue: PointerValue, numberOfBytesToCopy: u64, alignment: PowerOfTwoThirtyTwoBit, isVolatile: bool)
	{
		let arguments =
		[
			(toInt8PointerValue.asLLVMValueRefWrapper(), None),
			(fromInt8PointerValue.asLLVMValueRefWrapper(), None),
			(self.integer64BitUnsigned(numberOfBytesToCopy).asLLVMValueRefWrapper(), None),
			(self.integer32BitUnsigned(alignment.as_u32()).asLLVMValueRefWrapper(), None),
			(self.boolean(isVolatile).asLLVMValueRefWrapper(), None),
		];
		
		self.builderReference.call(self.context, functionReference, TailCall::Tail, &HashSet::default(), UsefulLLVMCallConv::LLVMCCallConv, None, &arguments, None);
		
		//  For each group of three, the first operand gives the byte offset of a field in bytes, the second gives its size in bytes, and the third gives its tbaa tag
		// TBAA tag is the from/to/offset/is-constant, ie !tbaa, ie TypeBasedAliasAnalysisNode::path(offsetIntoBaseType, from, to)
		// !{
		// i64 0, i64 4, !1,
		// i64 8, i64 4, !2
		// }
		//		xxx;
		//
		//		unsafe { LLVMSetMetadata(instruction.asLLVMValueRef(), self.context.metadataKind_tbaa_struct(), self.context.xxxx(typeBasedAliasAnalysisNode).asLLVMValueRef()) };
	}
	
	#[inline(always)]
	pub fn loadPointer(&self, arrayPointer: PointerValue, offsetIntoBaseType: u64, from: TypeBasedAliasAnalysisNode) -> PointerValue
	{
		PointerValue::fromLLVMValueRefWrapper(self.loadValue(arrayPointer, offsetIntoBaseType, from, TypeBasedAliasAnalysisNode::any_pointer(), Self::PointerAlignment))
	}
	
	#[inline(always)]
	pub fn loadValueFromReferencedStructField(&self, pointerValue: PointerValue, fieldIndex: u32, offsetIntoBaseType: u64, from: TypeBasedAliasAnalysisNode, to: TypeBasedAliasAnalysisNode, valueAlignment: PowerOfTwoThirtyTwoBit) -> (LLVMValueRefWrapper, PointerValue)
	{
		let arrayPointer = self.pointerToStructField(pointerValue, fieldIndex);
		let loadedValue = self.loadValue(arrayPointer, offsetIntoBaseType, from, to, valueAlignment);
		(loadedValue, arrayPointer)
	}
	
	#[inline(always)]
	pub fn loadPointerFromReferencedStructField(&self, pointerValue: PointerValue, fieldIndex: u32, offsetIntoBaseType: u64, from: TypeBasedAliasAnalysisNode) -> (PointerValue, PointerValue)
	{
		let arrayPointer = self.pointerToStructField(pointerValue, fieldIndex);
		let loadedPointer = self.loadPointer(arrayPointer, offsetIntoBaseType, from);
		(loadedPointer, arrayPointer)
	}
	
	#[inline(always)]
	fn path(offsetIntoBaseType: u64, from: TypeBasedAliasAnalysisNode, to: TypeBasedAliasAnalysisNode) -> Option<TypeBasedAliasAnalysisNode>
	{
		Some(TypeBasedAliasAnalysisNode::path(offsetIntoBaseType, from, to))
	}
	
	#[inline(always)]
	fn toLLVMValueRefWrapper<V: ToLLVMValueRefWrapper>(&self, value: V) -> LLVMValueRefWrapper
	{
		value.toLLVMValueRefWrapper(self.context)
	}
	
	#[inline(always)]
	fn typeRef(&self, llvmType: &LlvmType) -> LLVMTypeRefWrapper
	{
		self.context.typeRef(llvmType)
	}
	
	#[inline(always)]
	fn constant(&self, constant: &Constant) -> ConstantValue
	{
		self.context.constant(constant)
	}
	
	#[inline(always)]
	fn booleanTrue(&self) -> ConstantValue
	{
		self.constant(&Constant::True)
	}
	
	#[inline(always)]
	fn booleanFalse(&self) -> ConstantValue
	{
		self.constant(&Constant::False)
	}
	
	#[inline(always)]
	fn boolean(&self, value: bool) -> ConstantValue
	{
		self.constant(&Constant::boolean(value))
	}
	
	#[inline(always)]
	fn integer32BitUnsigned(&self, value: u32) -> ConstantValue
	{
		self.constant(&Constant::integer32BitUnsigned(value))
	}
	
	#[inline(always)]
	fn integer64BitUnsigned(&self, value: u64) -> ConstantValue
	{
		self.constant(&Constant::integer64BitUnsigned(value))
	}
}
