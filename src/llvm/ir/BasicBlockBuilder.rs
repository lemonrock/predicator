// This file is part of predicator. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/predicator/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of predicator. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/predicator/master/COPYRIGHT.


pub struct BasicBlockBuilder<'a>
{
	context: &'a Context,
	functionValue: FunctionValue,
	basicBlockReference: LLVMBasicBlockRef,
	builder: Builder<'a>,
	name: String,
}

impl<'a> BasicBlockBuilder<'a>
{
	pub const PointerAlignment: PowerOfTwoThirtyTwoBit = PowerOfTwoThirtyTwoBit::_8;
	
	#[inline(always)]
	pub(crate) fn createBasicBlock<S: Into<String> + Clone>(name: S, context: &'a Context, functionValue: FunctionValue) -> BasicBlockBuilder<'a>
	{
		let name = name.into();
		let cName = CString::new(name.as_bytes()).unwrap();
		let basicBlockReference = unsafe { LLVMAppendBasicBlockInContext(context.reference, functionValue.asLLVMValueRef(), cName.as_ptr()) };
		
		let builder = context.builder();
		
		let this = Self
		{
			context,
			functionValue,
			basicBlockReference,
			builder,
			name: name,
		};
		
		this.builder.positionAtEndOfBasicBlock(this.basicBlockReference);
		
		this
	}
	
	#[inline(always)]
	pub fn newBasicBlock<S: Into<String> + Clone>(&self, name: S) -> BasicBlockBuilder<'a>
	{
		Self::createBasicBlock(format!("{}.{}", &self.name, name.into()), self.context, self.functionValue)
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
		self.builder.returnVoid();
	}
	
	#[inline(always)]
	pub fn returnTrue(&self)
	{
		self.builder.returnValue(&Constant::True);
	}
	
	#[inline(always)]
	pub fn returnFalse(&self)
	{
		self.builder.returnValue(&Constant::False);
	}
	
	#[inline(always)]
	pub fn unconditionalBranch(&self, to: &BasicBlockBuilder<'a>)
	{
		self.builder.unconditionalBranch(to.basicBlockReference);
	}
	
	#[inline(always)]
	pub fn conditionalBranch(&self, ifCondition: ComparisonResultValue, thenBlock: &BasicBlockBuilder<'a>, elseBlock: &BasicBlockBuilder<'a>)
	{
		self.builder.conditionalBranch(ifCondition, thenBlock.basicBlockReference, elseBlock.basicBlockReference);
	}
	
	/// integerValueOrConstant's integer type must match IntegerConstant but the API can't easily enforce this
	#[inline(always)]
	pub fn switchBranch(&self, integerValueOrConstant: LLVMValueRef, defaultBlock: &BasicBlockBuilder<'a>, caseBlocks: BTreeMap<u8, BasicBlockBuilder<'a>>)
	{
		let switchInstruction = self.builder.switchBranch(integerValueOrConstant, defaultBlock.basicBlockReference, caseBlocks.len());
		for (constant, caseBlock) in caseBlocks.iter()
		{
			switchInstruction.addCase(*constant, caseBlock.basicBlockReference)
		}
	}
	
	#[inline(always)]
	pub fn phi(&self, llvmType: &LlvmType) -> BuilderPhiInstruction
	{
		self.builder.phi(llvmType)
	}
	
	#[inline(always)]
	pub fn store(&self, into: PointerValue, value: LLVMValueRefWrapper, offsetIntoBaseType: u64, from: TypeBasedAliasAnalysisNode, to: TypeBasedAliasAnalysisNode, alignment: PowerOfTwoThirtyTwoBit) -> LLVMValueRefWrapper
	{
		self.builder.store(into, value, Some(alignment), Some(TypeBasedAliasAnalysisNode::path(offsetIntoBaseType, from, to)))
	}
	
	#[inline(always)]
	pub fn pointerToStructField(&self, pointerValue: PointerValue, fieldIndex: u32) -> PointerValue
	{
		self.builder.getElementPointerPointerToStructToPointerToField(pointerValue, 0, fieldIndex)
	}
	
	#[inline(always)]
	pub fn loadValue(&self, arrayPointer: PointerValue, offsetIntoBaseType: u64, from: TypeBasedAliasAnalysisNode, to: TypeBasedAliasAnalysisNode, alignment: PowerOfTwoThirtyTwoBit) -> LLVMValueRefWrapper
	{
		self.builder.load(arrayPointer, Some(alignment), Some(TypeBasedAliasAnalysisNode::path(offsetIntoBaseType, from, to)))
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
	pub fn bitcastPointerToInt8Pointer(&self, pointerValue: PointerValue) -> PointerValue
	{
		self.builder.bitcastPointerToInt8Pointer(pointerValue)
	}
	
	#[inline(always)]
	pub fn getElementPointerAtArrayIndex<V: ToLLVMValueRefWrapper>(&self, pointerValue: PointerValue, arrayIndexInt64: V) -> PointerValue
	{
		self.builder.getElementPointerAtArrayIndex(pointerValue, arrayIndexInt64.toLLVMValueRefWrapper(self.context))
	}
	
	pub fn tailCallMemCpy64(&self, functionReference: FunctionValue, fromInt8PointerValue: PointerValue, toInt8PointerValue: PointerValue, numberOfBytesToCopy: u64, alignment: PowerOfTwoThirtyTwoBit, isVolatile: bool)
	{
		let arguments =
		[
			(toInt8PointerValue.asLLVMValueRefWrapper(), None),
			(fromInt8PointerValue.asLLVMValueRefWrapper(), None),
			(self.context.constant(&Constant::integer64BitUnsigned(numberOfBytesToCopy)).asLLVMValueRefWrapper(), None),
			(self.context.constant(&Constant::integer32BitUnsigned(alignment.as_u32())).asLLVMValueRefWrapper(), None),
			(self.context.constant(&Constant::boolean(isVolatile)).asLLVMValueRefWrapper(), None),
		];
		
		self.builder.call(self.context, functionReference, BuilderTailCall::Tail, &HashSet::default(), UsefulLLVMCallConv::LLVMCCallConv, None, &arguments);
		
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
	pub fn add<LHS: ToLLVMValueRefWrapper, RHS: ToLLVMValueRefWrapper>(&self, leftHandSide: LHS, rightHandSide: RHS) -> LLVMValueRefWrapper
	{
		self.builder.add(leftHandSide.toLLVMValueRefWrapper(self.context), rightHandSide.toLLVMValueRefWrapper(self.context))
	}
	
	#[inline(always)]
	pub fn ifInteger<S: Into<String> + Clone, LHS: ToLLVMValueRefWrapper, RHS: ToLLVMValueRefWrapper>(&self, ifName: S, leftHandSide: LHS, predicate: LLVMIntPredicate, rightHandSide: RHS) -> (ComparisonResultValue, BasicBlockBuilder<'a>, BasicBlockBuilder<'a>)
	{
		let ifName = ifName.into();
		
		let thenBlock = self.newBasicBlock(format!("{}.then", &ifName));
		let elseBlock = self.newBasicBlock(format!("{}.else", &ifName));
		let ifName = CString::new(format!("{}.{}.if", &self.name, &ifName)).unwrap();
		
		(self.builder.integerComparison(predicate, leftHandSide.toLLVMValueRefWrapper(self.context), rightHandSide.toLLVMValueRefWrapper(self.context), Some(&ifName)), thenBlock, elseBlock)
	}
	
	#[inline(always)]
	pub fn ifIntegerGuard<S: Into<String> + Clone, LHS: ToLLVMValueRefWrapper, RHS: ToLLVMValueRefWrapper>(&self, ifName: S, leftHandSide: LHS, predicate: LLVMIntPredicate, rightHandSide: RHS) -> (ComparisonResultValue, BasicBlockBuilder<'a>)
	{
		let ifName = ifName.into();
		
		let carryOnBlock = self.newBasicBlock(format!("{}.carry-on", &ifName));
		let ifName = CString::new(format!("{}.{}.if", &self.name, &ifName)).unwrap();
		
		(self.builder.integerComparison(predicate, leftHandSide.toLLVMValueRefWrapper(self.context), rightHandSide.toLLVMValueRefWrapper(self.context), Some(&ifName)), carryOnBlock)
	}
}
