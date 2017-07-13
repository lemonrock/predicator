// This file is part of mqtt. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/mqtt/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of mqtt. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/mqtt/master/COPYRIGHT.


pub trait ToLLVMValueRefWrapper
{
	#[inline(always)]
	fn toLLVMValueRefWrapper(&self, context: &Context) -> LLVMValueRefWrapper;
}

impl<T: Value> ToLLVMValueRefWrapper for T
{
	#[inline(always)]
	default fn toLLVMValueRefWrapper(&self, _: &Context) -> LLVMValueRefWrapper
	{
		self.asLLVMValueRefWrapper()
	}
}

impl ToLLVMValueRefWrapper for u8
{
	#[inline(always)]
	fn toLLVMValueRefWrapper(&self, context: &Context) -> LLVMValueRefWrapper
	{
		LLVMValueRefWrapper::fromLLVMValueRef(context.constantInteger8BitUnsigned(*self))
	}
}

impl ToLLVMValueRefWrapper for u64
{
	#[inline(always)]
	fn toLLVMValueRefWrapper(&self, context: &Context) -> LLVMValueRefWrapper
	{
		LLVMValueRefWrapper::fromLLVMValueRef(context.constantInteger64BitUnsigned(*self))
	}
}
