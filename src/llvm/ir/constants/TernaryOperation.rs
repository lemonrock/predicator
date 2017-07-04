// This file is part of mqtt. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/mqtt/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of mqtt. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/mqtt/master/COPYRIGHT.


#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum TernaryOperation
{
	Select,
	InsertElement,
	ShuffleVector,
}

impl TernaryOperation
{
	#[inline(always)]
	pub fn operate(&self, context: &Context, first: &Constant, second: &Constant, third: &Constant) -> LLVMValueRef
	{
		use self::TernaryOperation::*;
		
		let firstRef = context.constant(first).asLLVMValueRef();
		let secondRef = context.constant(second).asLLVMValueRef();
		let thirdRef = context.constant(third).asLLVMValueRef();
		
		unsafe
		{
			match *self
			{
				Select => LLVMConstSelect(firstRef, secondRef, thirdRef),
				InsertElement => LLVMConstInsertElement(firstRef, secondRef, thirdRef),
				ShuffleVector => LLVMConstShuffleVector(firstRef, secondRef, thirdRef),
			}
		}
	}
	
	#[inline(always)]
	pub fn llvmType<'a>(&self, first: &'a Constant, second: &'a Constant, _: &'a Constant) -> &'a LlvmType
	{
		use self::TernaryOperation::*;
		
		let choice = match *self
		{
			Select => second,
			InsertElement => first,
			ShuffleVector => first,
		};
		
		choice.llvmType()
	}
}
