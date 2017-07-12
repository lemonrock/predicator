// This file is part of predicator. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/predicator/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of predicator. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/predicator/master/COPYRIGHT.


#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum GlobalFieldVariant
{
	Constant(Constant),
	
	Value(UsefulLLVMThreadLocalMode, Option<Constant>),
}

impl GlobalFieldVariant
{
	#[inline(always)]
	fn set(&self, context: &Context, globalValue: GlobalValue)
	{
		match *self
		{
			GlobalFieldVariant::Constant(ref constant) =>
			{
				globalValue.setIsConstant();
				
				globalValue.setIsInternallyInitialized(context, constant);
			}
			
			GlobalFieldVariant::Value(ref threadLocalMode, ref constant) =>
			{
				globalValue.setIsVariable(threadLocalMode.to_LLVMThreadLocalMode());
				
				if let &Some(ref constant) = constant
				{
					globalValue.setIsInternallyInitialized(context, constant);
				}
				else
				{
					globalValue.setIsExternallyInitialized();
				}
			}
		}
	}
}
