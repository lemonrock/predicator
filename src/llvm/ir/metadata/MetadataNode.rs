// This file is part of mqtt. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/mqtt/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of mqtt. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/mqtt/master/COPYRIGHT.


#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MetadataNode(pub Vec<MetadataKind>);

impl ToReference<MetadataNodeValue> for MetadataNode
{
	#[inline(always)]
	fn toReference(&self, context: &Context) -> MetadataNodeValue
	{
		let mut values = Vec::with_capacity(self.0.len());
		
		for value in self.0.iter()
		{
			values.push(value.toReference(context).asLLVMValueRef());
		}
		
		MetadataNodeValue::fromLLVMValueRef(unsafe { LLVMMDNodeInContext(context.reference, values.as_mut_ptr(), values.len() as u32) })
	}
}

impl MetadataNode
{
	#[inline(always)]
	pub fn string<S: Into<String>>(value: S) -> Self
	{
		MetadataNode(vec!
		[
			MetadataKind::String(value.into())
		])
	}
}
