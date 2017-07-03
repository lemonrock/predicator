// This file is part of mqtt. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/mqtt/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of mqtt. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/mqtt/master/COPYRIGHT.


#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum MetadataKind
{
	String(String),
	Constant(Constant),
	Node(MetadataNode),
}

impl ToReference<LLVMValueRefWrapper> for MetadataKind
{
	fn toReference(&self, context: &Context) -> LLVMValueRefWrapper
	{
		use self::MetadataKind::*;
		
		match *self
		{
			String(ref string) =>
			{
				context.metadataString(string).asLLVMValueRefWrapper()
			}
			
			Constant(ref constant) =>
			{
				context.constant(constant).asLLVMValueRefWrapper()
			}
			
			Node(ref metadataNode) =>
			{
				context.metadataNode(metadataNode).asLLVMValueRefWrapper()
			}
		}
	}
}

impl MetadataKind
{
	pub fn string<S: Into<String>>(value: S) -> MetadataKind
	{
		MetadataKind::String(value.into())
	}
	
	pub fn toMetadataNode(self) -> MetadataNode
	{
		MetadataNode(vec!
		[
			self
		])
	}
}
