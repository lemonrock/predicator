// This file is part of predicator. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/predicator/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of predicator. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/predicator/master/COPYRIGHT.


pub struct Block<'a>
{
	context: &'a Context,
	functionValue: FunctionValue,
	basicBlockReference: LLVMBasicBlockRef,
	builderReference: LLVMBuilderRef,
	name: String,
	nextValue: Cell<u64>,
}

impl<'a> Drop for Block<'a>
{
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
			nextValue: Cell::new(0),
		}
	}
	
	#[inline(always)]
	pub fn namedChild(&self, childName: &str) -> Block<'a>
	{
		Self::create(format!("{}.{}", &self.name, childName), self.context, self.functionValue)
	}
	
	#[inline(always)]
	pub fn child(&self) -> Block<'a>
	{
		Self::create(self.nextValueName(), self.context, self.functionValue)
	}
	
	#[inline(always)]
	fn nextValueName(&self) -> String
	{
		let nextChild = self.nextValue.get();
		let name = format!("{}.{}", &self.name, nextChild);
		self.nextValue.set(nextChild + 1);
		name
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
	pub fn unconditionalBranch<ToBlockReference: ToLLVMBasicBlockRef>(&self, to: &ToBlockReference)
	{
		self.builderReference.unconditionalBranch(to);
	}
	
	#[inline(always)]
	pub fn unconditionalBranchToChild(&self, childName: &str) -> Block<'a>
	{
		let child = self.namedChild(childName);
		self.unconditionalBranch(&child);
		child
	}
	
	#[inline(always)]
	pub fn conditionalBranch<ThenToBlockReference: ToLLVMBasicBlockRef, ElseToBlockReference: ToLLVMBasicBlockRef>(&self, ifCondition: ComparisonResultValue, thenBlock: &ThenToBlockReference, elseBlock: &ElseToBlockReference)
	{
		self.builderReference.conditionalBranch(ifCondition, thenBlock, elseBlock);
	}
	
	#[inline(always)]
	pub fn switchBranch<V: ToLLVMValueRefWrapper, DefaultToBlockReference: ToLLVMBasicBlockRef, CaseToBlockReference: ToLLVMBasicBlockRef>(&self, switchOnValue: V, defaultBlock: &DefaultToBlockReference, caseBlocks: Vec<(u8, CaseToBlockReference)>)
	{
		self.builderReference.switchBranch(self.context, self.toLLVMValueRefWrapper(switchOnValue), defaultBlock, caseBlocks);
	}
	
	#[inline(always)]
	pub fn phi(&self, llvmType: &LlvmType, named: &str) -> PhiInstructionValue
	{
		self.builderReference.phi(self.typeRef(llvmType), Some(&CString::new(named).unwrap()))
	}
	
	#[inline(always)]
	pub fn bitcastPointerToInt8Pointer(&self, pointerValue: PointerValue) -> PointerValue
	{
		self.builderReference.bitcastPointerToInt8Pointer(self.context, pointerValue, None)
	}
	
	#[inline(always)]
	pub fn arithmetic<LHS: ToLLVMValueRefWrapper, RHS: ToLLVMValueRefWrapper>(&self, leftHandSide: LHS, operation: BinaryArithmetic, rightHandSide: RHS, named: &str) -> LLVMValueRefWrapper
	{
		operation.operate(self.builderReference, self.toLLVMValueRefWrapper(leftHandSide), self.toLLVMValueRefWrapper(rightHandSide), Some(&CString::new(named).unwrap()))
	}
	
	#[inline(always)]
	pub fn invert<V: ToLLVMValueRefWrapper>(&self, operation: UnaryArithmetic, value: V) -> LLVMValueRefWrapper
	{
		operation.operate(self.builderReference, self.toLLVMValueRefWrapper(value), None)
	}
	
	#[inline(always)]
	pub fn increment<V: Value>(&self, original: V, increment: u64, named: &str) -> LLVMValueRefWrapper
	{
		self.arithmetic(original, BinaryArithmetic::Add, increment, named)
	}
	
	#[inline(always)]
	pub fn getElementPointerAtArrayIndex<V: ToLLVMValueRefWrapper>(&self, named: &str, pointerValue: PointerValue, arrayIndexInt64: V) -> PointerValue
	{
		let named = format!("{}_pointer_at_array_index", named);
		self.builderReference.getElementPointerAtArrayIndex(pointerValue, arrayIndexInt64.toLLVMValueRefWrapper(self.context), Some(&CString::new(named).unwrap()))
	}
	
	#[inline(always)]
	pub fn pointerToStructField(&self, named: &str, pointerValue: PointerValue, fieldIndex: u32) -> PointerValue
	{
		let named = format!("{}_pointer", named);
		self.builderReference.getElementPointerAtArrayIndexFieldIndex(pointerValue, self.integer64BitUnsigned(0), self.integer32BitUnsigned(fieldIndex), Some(&CString::new(named).unwrap()))
	}
	
	#[inline(always)]
	pub fn storeValue(&self, into: PointerValue, value: LLVMValueRefWrapper, offsetIntoBaseType: u64, from: TypeBasedAliasAnalysisNode, to: TypeBasedAliasAnalysisNode, alignment: PowerOfTwoThirtyTwoBit) -> LLVMValueRefWrapper
	{
		self.builderReference.store(self.context, into, value, Self::path(offsetIntoBaseType, from, to), Some(alignment))
	}
	
	#[inline(always)]
	pub fn loadValue(&self, named: &str, arrayPointer: PointerValue, offsetIntoBaseType: u64, from: TypeBasedAliasAnalysisNode, to: TypeBasedAliasAnalysisNode, alignment: PowerOfTwoThirtyTwoBit) -> LLVMValueRefWrapper
	{
		self.builderReference.load(self.context, arrayPointer, Self::path(offsetIntoBaseType, from, to), Some(alignment), Some(&CString::new(named).unwrap()))
	}
	
	#[inline(always)]
	pub fn comparison<LHS: ToLLVMValueRefWrapper, RHS: ToLLVMValueRefWrapper>(&self, leftHandSide: LHS, predicate: LLVMIntPredicate, rightHandSide: RHS, named: &str) -> ComparisonResultValue
	{
		self.builderReference.integerComparison(self.toLLVMValueRefWrapper(leftHandSide), predicate, self.toLLVMValueRefWrapper(rightHandSide), Some(&CString::new(named).unwrap()))
	}
	
	#[inline(always)]
	pub fn ifInteger<LHS: ToLLVMValueRefWrapper, RHS: ToLLVMValueRefWrapper>(&self, leftHandSide: LHS, predicate: LLVMIntPredicate, rightHandSide: RHS, ifName: &str, thenName: &str, elseName: &str) -> (ComparisonResultValue, Block<'a>, Block<'a>)
	{
		let thenBlock = self.namedChild(thenName);
		let elseBlock = self.namedChild(elseName);
		(self.comparison(leftHandSide, predicate,rightHandSide, ifName), thenBlock, elseBlock)
	}
	
	#[inline(always)]
	pub fn ifFalseCarryOn<TrueToBlockReference: ToLLVMBasicBlockRef>(&self, isTrue: ComparisonResultValue, ifTrueBlock: &TrueToBlockReference) -> Block<'a>
	{
		let carryOnBlock = self.child();
		self.conditionalBranch(isTrue, ifTrueBlock, &carryOnBlock);
		carryOnBlock
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
	pub fn loadPointer(&self, named: &str, arrayPointer: PointerValue, offsetIntoBaseType: u64, from: TypeBasedAliasAnalysisNode) -> PointerValue
	{
		PointerValue::fromLLVMValueRefWrapper(self.loadValue(named, arrayPointer, offsetIntoBaseType, from, TypeBasedAliasAnalysisNode::any_pointer(), Self::PointerAlignment))
	}
	
	#[inline(always)]
	pub fn loadValueFromReferencedStructField(&self, named: &str, pointerValue: PointerValue, fieldIndex: u32, offsetIntoBaseType: u64, from: TypeBasedAliasAnalysisNode, to: TypeBasedAliasAnalysisNode, valueAlignment: PowerOfTwoThirtyTwoBit) -> (LLVMValueRefWrapper, PointerValue)
	{
		let named = format!("{}_value", named);
		let arrayPointer = self.pointerToStructField(&named, pointerValue, fieldIndex);
		let loadedValue = self.loadValue(&named, arrayPointer, offsetIntoBaseType, from, to, valueAlignment);
		(loadedValue, arrayPointer)
	}
	
	#[inline(always)]
	pub fn loadPointerFromReferencedStructField(&self, named: &str, pointerValue: PointerValue, fieldIndex: u32, offsetIntoBaseType: u64, from: TypeBasedAliasAnalysisNode) -> (PointerValue, PointerValue)
	{
		let named = format!("{}_pointer", named);
		let arrayPointer = self.pointerToStructField(&named, pointerValue, fieldIndex);
		let loadedPointer = self.loadPointer(&named, arrayPointer, offsetIntoBaseType, from);
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
