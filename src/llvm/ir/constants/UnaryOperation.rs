// This file is part of mqtt. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/mqtt/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of mqtt. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/mqtt/master/COPYRIGHT.


#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum UnaryOperation
{
	Neg,
	NSWNeg,
	NUWNeg,
	FNeg,
	Not,
}

impl UnaryOperation
{
	#[inline(always)]
	pub fn operate(&self, context: &Context, value: &Constant) -> LLVMValueRef
	{
		use self::UnaryOperation::*;
		
		let constantRef = context.constant(value).asLLVMValueRef();
		
		unsafe
		{
			match *self
			{
				Neg => LLVMConstNeg(constantRef),
				NSWNeg => LLVMConstNSWNeg(constantRef),
				NUWNeg => LLVMConstNUWNeg(constantRef),
				FNeg => LLVMConstFNeg(constantRef),
				Not => LLVMConstNot(constantRef),
			}
		}
	}
}
