// This file is part of mqtt. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/mqtt/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of mqtt. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/mqtt/master/COPYRIGHT.


#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MetadataNodeValue(LLVMValueRefWrapper);

impl Value for MetadataNodeValue
{
	#[inline(always)]
	fn fromLLVMValueRef(value: LLVMValueRef) -> Self
	{
		MetadataNodeValue(LLVMValueRefWrapper::fromLLVMValueRef(value))
	}
	
	#[inline(always)]
	fn asLLVMValueRef(&self) -> LLVMValueRef
	{
		self.0.asLLVMValueRef()
	}
}

impl MetadataNodeValue
{
	#[inline(always)]
	pub(crate) fn toTypeBasedAliasAnalysisNodeValue(self) -> TypeBasedAliasAnalysisNodeValue
	{
		TypeBasedAliasAnalysisNodeValue::fromLLVMValueRef(self.asLLVMValueRef())
	}
}
