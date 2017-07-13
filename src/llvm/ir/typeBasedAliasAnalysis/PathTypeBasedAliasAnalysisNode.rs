// This file is part of mqtt. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/mqtt/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of mqtt. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/mqtt/master/COPYRIGHT.


#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PathTypeBasedAliasAnalysisNode(LLVMValueRef);

impl PathTypeBasedAliasAnalysisNode
{
	#[inline(always)]
	pub fn new(context: &Context, offsetIntoBaseType: u64, baseType: &TypeBasedAliasAnalysisNode, accessType: &TypeBasedAliasAnalysisNode) -> Self
	{
		let node = TypeBasedAliasAnalysisNode::path(context, offsetIntoBaseType, baseType, accessType);
		let reference = context.typeBasedAliasAnalysisNode(&node).asLLVMValueRef();
		PathTypeBasedAliasAnalysisNode(reference)
	}
}

impl Value for PathTypeBasedAliasAnalysisNode
{
	#[inline(always)]
	fn fromLLVMValueRef(value: LLVMValueRef) -> Self
	{
		debug_assert!(!value.is_null(), "value is null pointer");
		
		PathTypeBasedAliasAnalysisNode(value)
	}
	
	#[inline(always)]
	fn asLLVMValueRef(&self) -> LLVMValueRef
	{
		self.0
	}
}
