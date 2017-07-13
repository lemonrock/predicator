// This file is part of mqtt. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/mqtt/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of mqtt. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/mqtt/master/COPYRIGHT.


#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum UnaryArithmetic
{
	Neg,
	NSWNeg,
	NUWNeg,
	FNeg,
	Not,
}

impl UnaryArithmetic
{
	#[inline(always)]
	pub(crate) fn operate<V: Value>(&self, builderReference: LLVMBuilderRef, value: V) -> LLVMValueRefWrapper
	{
		use self::UnaryArithmetic::*;
		
		let value = value.asLLVMValueRef();
		
		let value = unsafe
		{
			match *self
			{
				Neg => LLVMBuildNeg(builderReference, value, emptyName!()),
				NSWNeg => LLVMBuildNSWNeg(builderReference, value, emptyName!()),
				NUWNeg => LLVMBuildNUWNeg(builderReference, value, emptyName!()),
				FNeg => LLVMBuildFNeg(builderReference, value, emptyName!()),
				Not => LLVMBuildNot(builderReference, value, emptyName!()),
			}
		};
		LLVMValueRefWrapper::fromLLVMValueRef(value)
	}
}
