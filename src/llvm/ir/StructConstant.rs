// This file is part of predicator. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/predicator/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of predicator. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/predicator/master/COPYRIGHT.


#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct StructConstant
{
	llvmType: LlvmType,
	values: Vec<AnyConstant>,
}

impl Constant for StructConstant
{
	#[inline(always)]
	fn to_LLVMValueRef(&self, context: &Context) -> LLVMValueRef
	{
		let mut constantCache = context.structConstantCache.borrow_mut();
		
		if let Some(extant) = constantCache.get(self)
		{
			return *extant;
		}
		
		let typeRef = context.typeRef(&self.llvmType);
		
		let mut values: Vec<LLVMValueRef> = self.values.iter().map(|constant| constant.to_LLVMValueRef(context)).collect();
		
		let value = unsafe { LLVMConstNamedStruct(typeRef, values.as_mut_ptr(), values.len() as u32) };
		
		constantCache.insert(self.clone(), value);
		
		value
	}
	
	#[inline(always)]
	fn llvmType(&self) -> &LlvmType
	{
		&self.llvmType
	}
}

impl StructConstant
{
	pub fn new(name: Option<CString>, values: Vec<AnyConstant>, isPacked: bool) -> Self
	{
		Self
		{
			llvmType: LlvmType::Struct
			{
				name: name,
				body: StructBody
				{
					elements: values.iter().map(|constant| constant.llvmType().clone()).collect(),
					isPacked: isPacked,
				}
			},
			values: values,
		}
	}
}
