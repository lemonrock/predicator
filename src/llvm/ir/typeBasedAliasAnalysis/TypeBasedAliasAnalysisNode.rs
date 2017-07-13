// This file is part of predicator. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/predicator/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of predicator. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/predicator/master/COPYRIGHT.


#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TypeBasedAliasAnalysisNode(MetadataNode);

impl TypeBasedAliasAnalysisNode
{
	#[inline(always)]
	pub fn toMetadataNode(&self) -> MetadataNode
	{
		self.0.clone()
	}
	
	#[inline(always)]
	pub fn asStructField(&self, offset: u64) -> TypeBasedAliasAnalysisNodeStructField
	{
		TypeBasedAliasAnalysisNodeStructField
		{
			kind: self.clone(),
			offset: offset,
		}
	}
	
	#[inline(always)]
	fn Root() -> Self
	{
		TypeBasedAliasAnalysisNode
		(
			MetadataNode(vec!
			[
				MetadataKind::String("Simple C/C++ TBAA".to_owned()),
			])
		)
	}
	
	#[inline(always)]
	fn Scalar(name: &'static str, parent: &TypeBasedAliasAnalysisNode, isConstant: bool) -> Self
	{
		let isConstant = if isConstant
		{
			1
		}
		else
		{
			0
		};
		
		TypeBasedAliasAnalysisNode
		(
			MetadataNode(vec!
			[
				MetadataKind::String(name.to_owned()),
				MetadataKind::Node(parent.toMetadataNode()),
				MetadataKind::Constant(Constant::integer64BitUnsigned(isConstant)),
			])
		)
	}
	
	#[inline(always)]
	pub fn omnipotent_char() -> Self
	{
		Self::Scalar("omnipotent char", &TypeBasedAliasAnalysisNode::Root(), false)
	}
	
	#[inline(always)]
	pub fn short() -> Self
	{
		Self::Scalar("short", &Self::omnipotent_char(), false)
	}
	
	#[inline(always)]
	pub fn int() -> Self
	{
		Self::Scalar("int", &Self::omnipotent_char(), false)
	}
	
	#[inline(always)]
	pub fn long() -> Self
	{
		Self::Scalar("long", &Self::omnipotent_char(), false)
	}
	
	#[inline(always)]
	pub fn long_long() -> Self
	{
		Self::Scalar("long long", &Self::omnipotent_char(), false)
	}
	
	#[inline(always)]
	pub fn any_pointer() -> Self
	{
		Self::Scalar("any pointer", &Self::omnipotent_char(), false)
	}
	
	#[inline(always)]
	pub fn path(offsetIntoBaseType: u64, baseType: &TypeBasedAliasAnalysisNode, accessType: &TypeBasedAliasAnalysisNode) -> Self
	{
		const isConstant: bool = false;
		
		let constantValue = if isConstant
		{
			1
		}
		else
		{
			0
		};
		
		TypeBasedAliasAnalysisNode(MetadataNode(vec!
		[
			MetadataKind::Node(baseType.toMetadataNode()),
			MetadataKind::Node(accessType.toMetadataNode()),
			MetadataKind::Constant(Constant::integer64BitUnsigned(offsetIntoBaseType)),
			MetadataKind::Constant(Constant::integer64BitUnsigned(constantValue)),
		]))
	}
	
	#[inline(always)]
	pub fn namedStruct<S: Into<String>>(name: S, fields: &[(TypeBasedAliasAnalysisNode, u64)]) -> Self
	{
		let mut values = Vec::with_capacity(1 + 2 * fields.len());
		values.push(MetadataKind::String(name.into()));
		
		for &(ref kind, offset) in fields
		{
			values.push(MetadataKind::Node(kind.toMetadataNode()));
			values.push(MetadataKind::Constant(Constant::integer64BitUnsigned(offset)));
		}
		
		TypeBasedAliasAnalysisNode(MetadataNode(values))
	}
}
