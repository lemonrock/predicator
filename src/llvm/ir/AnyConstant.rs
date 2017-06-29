// This file is part of predicator. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/predicator/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of predicator. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/predicator/master/COPYRIGHT.


#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum AnyConstant
{
	Integer(IntegerConstant),
	Float(FloatConstant),
	Struct(StructConstant),
}

impl Constant for AnyConstant
{
	#[inline(always)]
	fn to_LLVMValueRef(&self, context: &Context) -> LLVMValueRef
	{
		use self::AnyConstant::*;
		
		match *self
		{
			Integer(ref constant) => constant.to_LLVMValueRef(context),
			Float(ref constant) => constant.to_LLVMValueRef(context),
			Struct(ref constant) => constant.to_LLVMValueRef(context),
		}
	}
	
	#[inline(always)]
	fn llvmType(&self) -> &LlvmType
	{
		use self::AnyConstant::*;
		
		match *self
		{
			Integer(ref constant) => constant.llvmType(),
			Float(ref constant) => constant.llvmType(),
			Struct(ref constant) => constant.llvmType(),
		}
	}
}
