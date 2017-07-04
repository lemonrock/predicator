// This file is part of predicator. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/predicator/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of predicator. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/predicator/master/COPYRIGHT.


pub struct BasicBlockBuilder<'a>
{
	context: &'a Context,
	functionValue: FunctionValue,
	pub basicBlockReference: LLVMBasicBlockRef,
	pub builder: Builder<'a>,
}

impl<'a> BasicBlockBuilder<'a>
{
	#[inline(always)]
	pub(crate) fn createBasicBlock<S: Into<String>>(name: S, context: &'a Context, functionValue: FunctionValue) -> BasicBlockBuilder<'a>
	{
		let name = CString::new(name.into().as_bytes()).unwrap();
		let basicBlockReference = unsafe { LLVMAppendBasicBlockInContext(context.reference, functionValue.asLLVMValueRef(), name.as_ptr()) };
		
		let builder = context.builder();
		
		let this = Self
		{
			context,
			functionValue,
			basicBlockReference,
			builder,
		};
		
		this.builder.positionAtEndOfBasicBlock(this.basicBlockReference);
		
		this
	}
	
	#[inline(always)]
	pub fn newBasicBlock<S: Into<String>>(&self, name: S) -> BasicBlockBuilder<'a>
	{
		Self::createBasicBlock(name, self.context, self.functionValue)
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
	pub fn returnVoid(self)
	{
		self.builder.returnVoid();
	}
	
	#[inline(always)]
	pub fn returnTrue(self)
	{
		self.builder.returnValue(&Constant::True);
	}
	
	#[inline(always)]
	pub fn returnFalse(self)
	{
		self.builder.returnValue(&Constant::False);
	}
	
	#[inline(always)]
	pub fn unconditionalBranch(self, to: &BasicBlockBuilder<'a>)
	{
		self.builder.unconditionalBranch(to.basicBlockReference);
	}
	
	#[inline(always)]
	pub fn unconditionalBranchWithCreation(self, to: &str) -> BasicBlockBuilder<'a>
	{
		let to = self.newBasicBlock(to);
		self.builder.unconditionalBranch(to.basicBlockReference);
		to
	}
	
	#[inline(always)]
	pub fn conditionalBranch(self, ifCondition: LLVMValueRef, thenBlock: &BasicBlockBuilder<'a>, elseBlock: &BasicBlockBuilder<'a>)
	{
		self.builder.conditionalBranch(ifCondition, thenBlock.basicBlockReference, elseBlock.basicBlockReference);
	}
	
	/// integerValueOrConstant's integer type must match IntegerConstant but the API can't easily enforce this
	#[inline(always)]
	pub fn switchBranch(self, integerValueOrConstant: LLVMValueRef, defaultBlock: &BasicBlockBuilder<'a>, caseBlocks: BTreeMap<u8, BasicBlockBuilder<'a>>)
	{
		let switchInstruction = self.builder.switchBranch(integerValueOrConstant, defaultBlock.basicBlockReference, caseBlocks.len());
		for (constant, caseBlock) in caseBlocks.iter()
		{
			switchInstruction.addCase(*constant, caseBlock.basicBlockReference)
		}
	}
	
	pub fn loadFromReferencedStructField(&self, PointerValue: PointerValue, fieldIndex: u32, offsetIntoBaseType: u64, from: TypeBasedAliasAnalysisNode, to: TypeBasedAliasAnalysisNode) -> LLVMValueRefWrapper
	{
		let arrayPointer = self.builder.getElementPointer_PointerToStructToPointerToField(PointerValue, 0, fieldIndex);
		self.builder.load(arrayPointer, Some(PowerOfTwoThirtyTwoBit::_8), Some(TypeBasedAliasAnalysisNode::path(offsetIntoBaseType, from, to)))
	}
	
	pub fn bitcastPointerToInt8Pointer(&self, pointerValue: PointerValue) -> PointerValue
	{
		self.builder.bitcastPointerToInt8Pointer(pointerValue)
	}
	
	pub fn getElementPointer_ArrayIndex(&self, pointerValue: PointerValue, arrayIndex: u64) -> PointerValue
	{
		self.builder.getElementPointer_ArrayIndex(pointerValue, arrayIndex)
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
}
