// This file is part of predicator. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/predicator/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of predicator. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/predicator/master/COPYRIGHT.


#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum TypeBasedAliasAnalysisNode
{
	Root,
	
	Scalar
	{
		name: String,
		parent: Box<TypeBasedAliasAnalysisNode>,
		isConstant: bool,
	},
	
	Struct
	{
		name: String,
		fields: Vec<TypeBasedAliasAnalysisNodeStructField>
	},
	
	Path
	{
		baseType: Box<TypeBasedAliasAnalysisNode>, // eg a TypeBasedAliasAnalysisNode::Struct
		accessType: Box<TypeBasedAliasAnalysisNode>, // eg a TypeBasedAliasAnalysisNode::any_pointer()
		offsetIntoBaseType: u64,
		isConstant: bool,
	}
}

impl TypeBasedAliasAnalysisNode
{
	pub fn toMetadataNode(&self) -> MetadataNode
	{
		use self::TypeBasedAliasAnalysisNode::*;
		
		match *self
		{
			Root =>
			{
				MetadataNode(vec!
				[
					MetadataKind::string("Simple C/C++ TBAA")
				])
			}
			
			Scalar { ref name, ref parent, ref isConstant } =>
			{
				let isConstant = if *isConstant
				{
					1
				}
				else
				{
					0
				};
				
				MetadataNode(vec!
				[
					MetadataKind::string(name.as_str()),
					MetadataKind::Node(parent.toMetadataNode()),
					MetadataKind::Constant(Constant::integer64BitUnsigned(isConstant)),
				])
			}
			
			Struct { ref name, ref fields } =>
			{
				let mut values = Vec::with_capacity(1 + 2 * fields.len());
				values.push(MetadataKind::String(name.to_owned()));
				
				for field in fields.iter()
				{
					values.push(MetadataKind::Node(field.kind.toMetadataNode()));
					values.push(MetadataKind::Constant(Constant::integer64BitUnsigned(field.offset)));
				}
				
				MetadataNode(values)
			}
			
			Path { ref baseType, ref accessType, ref offsetIntoBaseType, ref isConstant } =>
			{
				let constantValue = if *isConstant
				{
					1
				}
				else
				{
					0
				};
				
				MetadataNode(vec!
				[
					MetadataKind::Node(baseType.toMetadataNode()),
					MetadataKind::Node(accessType.toMetadataNode()),
					MetadataKind::Constant(Constant::integer64BitUnsigned(*offsetIntoBaseType)),
					MetadataKind::Constant(Constant::integer64BitUnsigned(constantValue)),
				])
			}
		}
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
	pub fn omnipotent_char() -> Self
	{
		TypeBasedAliasAnalysisNode::Scalar
		{
			name: "omnipotent char".to_owned(),
			parent: Box::new(TypeBasedAliasAnalysisNode::Root),
			isConstant: false,
		}
	}
	
	#[inline(always)]
	pub fn short() -> Self
	{
		TypeBasedAliasAnalysisNode::Scalar
		{
			name: "short".to_owned(),
			parent: Box::new(Self::omnipotent_char()),
			isConstant: false,
		}
	}
	
	#[inline(always)]
	pub fn int() -> Self
	{
		TypeBasedAliasAnalysisNode::Scalar
		{
			name: "int".to_owned(),
			parent: Box::new(Self::omnipotent_char()),
			isConstant: false,
		}
	}
	
	#[inline(always)]
	pub fn long() -> Self
	{
		TypeBasedAliasAnalysisNode::Scalar
		{
			name: "long".to_owned(),
			parent: Box::new(Self::omnipotent_char()),
			isConstant: false,
		}
	}
	
	#[inline(always)]
	pub fn long_long() -> Self
	{
		TypeBasedAliasAnalysisNode::Scalar
		{
			name: "long long".to_owned(),
			parent: Box::new(Self::omnipotent_char()),
			isConstant: false,
		}
	}
	
	#[inline(always)]
	pub fn any_pointer() -> Self
	{
		TypeBasedAliasAnalysisNode::Scalar
		{
			name: "any pointer".to_owned(),
			parent: Box::new(Self::omnipotent_char()),
			isConstant: false,
		}
	}
	
	#[inline(always)]
	pub fn path(offsetIntoBaseType: u64, from: TypeBasedAliasAnalysisNode, to: TypeBasedAliasAnalysisNode) -> Self
	{
		TypeBasedAliasAnalysisNode::Path
		{
			baseType: Box::new(from),
			accessType: Box::new(to),
			offsetIntoBaseType: offsetIntoBaseType,
			isConstant: false,
		}
	}
	
	#[inline(always)]
	pub fn namedStruct<S: Into<String>>(name: S, fields: &[(TypeBasedAliasAnalysisNode, u64)]) -> Self
	{
		let mut structFields = Vec::with_capacity(fields.len());
		
		for &(ref kind, offset) in fields
		{
			structFields.push(kind.asStructField(offset));
		}
		
		TypeBasedAliasAnalysisNode::Struct
		{
			name: name.into(),
			fields: structFields,
		}
	}
}
