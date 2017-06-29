// This file is part of predicator. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/predicator/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of predicator. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/predicator/master/COPYRIGHT.


pub struct BasicBlockBuilder<'a>
{
	context: &'a Context,
	functionReference: LLVMValueRef,
	basicBlockReference: LLVMBasicBlockRef,
	builder: Builder,
}

impl<'a> BasicBlockBuilder<'a>
{
	#[inline(always)]
	fn createBasicBlock(name: &str, context: &'a Context, functionReference: LLVMValueRef) -> BasicBlockBuilder<'a>
	{
		let name = CString::new(name.as_bytes()).unwrap();
		let basicBlockReference = unsafe { LLVMAppendBasicBlockInContext(context.reference, functionReference, name.as_ptr()) };
		
		let builder = context.builder();
		
		let this = Self
		{
			context: context,
			functionReference: functionReference,
			basicBlockReference: basicBlockReference,
			builder: builder,
		};
		
		this.builder.positionAtEndOfBasicBlock(this.basicBlockReference);
		
		this
	}
	
	#[inline(always)]
	pub fn newBasicBlock(&self, to: &str) -> BasicBlockBuilder<'a>
	{
		Self::createBasicBlock(to, self.context, self.functionReference)
	}
	
	pub fn returnVoid(self)
	{
		self.builder.returnVoid();
	}
	
	pub fn returnTrue(self)
	{
		self.builder.returnValue(self.True());
	}
	
	pub fn returnFalse(self)
	{
		self.builder.returnValue(self.False());
	}
	
	pub fn unconditionalBranch(self, to: &BasicBlockBuilder<'a>)
	{
		self.builder.unconditionalBranch(to.basicBlockReference);
	}
	
	pub fn unconditionalBranchWithCreation(self, to: &str) -> BasicBlockBuilder<'a>
	{
		let to = self.newBasicBlock(to);
		self.builder.unconditionalBranch(to.basicBlockReference);
		to
	}
	
	pub fn conditionalBranch(self, ifCondition: LLVMValueRef, thenBlock: &BasicBlockBuilder<'a>, elseBlock: &BasicBlockBuilder<'a>)
	{
		self.builder.conditionalBranch(ifCondition, thenBlock.basicBlockReference, elseBlock.basicBlockReference);
	}
	
	/// integerValueOrConstant's integer type must match IntegerConstant but the API can't easily enforce this
	pub fn switchBranch(self, integerValueOrConstant: LLVMValueRef, defaultBlock: &BasicBlockBuilder<'a>, caseBlocks: BTreeMap<IntegerConstant, BasicBlockBuilder<'a>>)
	{
		let switchInstruction = self.builder.switchBranch(integerValueOrConstant, defaultBlock.basicBlockReference, caseBlocks.len());
		for (constant, caseBlock) in caseBlocks.iter()
		{
			switchInstruction.addCase(self.integerConstant(constant), caseBlock.basicBlockReference)
		}
	}
	
	#[inline(always)]
	fn True(&self) -> LLVMValueRef
	{
		self.integerConstant(&IntegerConstant::True)
	}
	
	#[inline(always)]
	fn False(&self) -> LLVMValueRef
	{
		self.integerConstant(&IntegerConstant::False)
	}
	
	#[inline(always)]
	fn integerConstant(&self, constant: &IntegerConstant) -> LLVMValueRef
	{
		self.context.integerConstant(constant)
	}
	
	
	
	pub fn push(&self)
	{
		let ClientIdentifierType = LlvmType::Int64;
		let SubscriptionForThatClientIdentifierType = LlvmType::Int64;
		
		let SubscriberType = LlvmType::namedStruct
		(
			"Subscriber",
			false,
			vec!
			[
				ClientIdentifierType,                     // clientIdentifier
				SubscriptionForThatClientIdentifierType,  // subscriberIdentifier
			]
		);
		
		let CountType = LlvmType::Int64;
		let SubscriberPointerType = LlvmType::pointer(SubscriberType);
		
		let SubscribersType = LlvmType::namedStruct
		(
			"Subscribers",
			false,
			vec!
			[
				CountType,              // count
				SubscriberPointerType,  // subscribers (malloc'd in advance)
			]
		);
		
	}
}
