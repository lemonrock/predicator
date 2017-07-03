// This file is part of predicator. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/predicator/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of predicator. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/predicator/master/COPYRIGHT.


#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum FieldVariant
{
	Constant(Constant),
	
	Value(UsefulLLVMThreadLocalMode, Option<Constant>),
}

impl FieldVariant
{
	fn set(&self, context: &Context, globalValue: GlobalValue)
	{
		let globalValue = globalValue.asLLVMValueRef();
		
		match *self
		{
			FieldVariant::Constant(ref constant) =>
			{
				unsafe { LLVMSetGlobalConstant(globalValue, 1) };
				unsafe { LLVMSetInitializer(globalValue, constant.toConstantValue(context).asLLVMValueRef()) };
			}
			
			FieldVariant::Value(ref threadLocalMode, ref constant) =>
			{
				unsafe { LLVMSetGlobalConstant(globalValue, 0) };
				
				if let &Some(ref constant) = constant
				{
					unsafe { LLVMSetInitializer(globalValue, constant.toConstantValue(context).asLLVMValueRef()) };
				}
				else
				{
					unsafe { LLVMSetExternallyInitialized(globalValue, 1) };
				}
				
				unsafe { LLVMSetThreadLocalMode(globalValue, threadLocalMode.to_LLVMThreadLocalMode()) };
				
				match *threadLocalMode
				{
					UsefulLLVMThreadLocalMode::LLVMNotThreadLocal => (),
					_ => unsafe { LLVMSetThreadLocal(globalValue, 1) }
				}
			}
		}
	}
}
