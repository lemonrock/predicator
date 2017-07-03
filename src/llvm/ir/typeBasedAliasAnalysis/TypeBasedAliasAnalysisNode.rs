// This file is part of predicator. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/predicator/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of predicator. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/predicator/master/COPYRIGHT.


#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum TypeBasedAliasAnalysisNode
{
	Scalar
	{
		name: String,
		parent: Option<Box<TypeBasedAliasAnalysisNode>>,
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
	pub fn toTypeBasedAliasAnalysisNodeValue(&self, context: &Context) -> TypeBasedAliasAnalysisNodeValue
	{
		use self::TypeBasedAliasAnalysisNode::*;
		
		TypeBasedAliasAnalysisNodeValue::fromLLVMValueRef
		(
			match *self
			{
				Scalar { ref name, ref parent, ref isConstant } =>
				{
					let parentX = if let &Some(ref parentY) = parent
					{
						parentY.toTypeBasedAliasAnalysisNodeValue(context).asLLVMValueRef()
					}
					else
					{
						context.metadataString("Simple C/C++ TBAA").asLLVMValueRef()
					};
					
					let constantValue = if *isConstant
					{
						1
					}
					else
					{
						0
					};
					
					let mut values =
					[
						context.metadataString(name).asLLVMValueRef(),
						parentX,
						context.constant(&Constant::integer64BitUnsigned(constantValue)).asLLVMValueRef(),
					];
					
					unsafe { LLVMMDNodeInContext(context.reference, values.as_mut_ptr(), values.len() as u32) }
				}
				
				Struct { ref name, ref fields } =>
				{
					let mut values = Vec::with_capacity(1 + 2 * fields.len());
					values.push(context.metadataString(name).asLLVMValueRef());
					
					for field in fields.iter()
					{
						values.push(field.toTypeBasedAliasAnalysisNodeValue(context).asLLVMValueRef());
						values.push(field.toConstantValue(context).asLLVMValueRef());
					}
					
					unsafe { LLVMMDNodeInContext(context.reference, values.as_mut_ptr(), values.len() as u32) }
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
					
					let mut values =
					[
						baseType.toTypeBasedAliasAnalysisNodeValue(context).asLLVMValueRef(),
						accessType.toTypeBasedAliasAnalysisNodeValue(context).asLLVMValueRef(),
						context.constant(&Constant::integer64BitUnsigned(*offsetIntoBaseType)).asLLVMValueRef(),
						context.constant(&Constant::integer64BitUnsigned(constantValue)).asLLVMValueRef(),
					];
					
					unsafe { LLVMMDNodeInContext(context.reference, values.as_mut_ptr(), values.len() as u32) }
				}
			}
		)
	}
	
	#[inline(always)]
	pub fn omnipotent_char() -> Self
	{
		TypeBasedAliasAnalysisNode::Scalar
		{
			name: "omnipotent char".to_owned(),
			parent: None,
			isConstant: false,
		}
	}
	
	#[inline(always)]
	pub fn short() -> Self
	{
		TypeBasedAliasAnalysisNode::Scalar
		{
			name: "short".to_owned(),
			parent: Some(Box::new(Self::omnipotent_char())),
			isConstant: false,
		}
	}
	
	#[inline(always)]
	pub fn int() -> Self
	{
		TypeBasedAliasAnalysisNode::Scalar
		{
			name: "int".to_owned(),
			parent: Some(Box::new(Self::omnipotent_char())),
			isConstant: false,
		}
	}
	
	#[inline(always)]
	pub fn long() -> Self
	{
		TypeBasedAliasAnalysisNode::Scalar
		{
			name: "long".to_owned(),
			parent: Some(Box::new(Self::omnipotent_char())),
			isConstant: false,
		}
	}
	
	#[inline(always)]
	pub fn long_long() -> Self
	{
		TypeBasedAliasAnalysisNode::Scalar
		{
			name: "long long".to_owned(),
			parent: Some(Box::new(Self::omnipotent_char())),
			isConstant: false,
		}
	}
	
	#[inline(always)]
	pub fn any_pointer() -> Self
	{
		TypeBasedAliasAnalysisNode::Scalar
		{
			name: "any pointer".to_owned(),
			parent: Some(Box::new(Self::omnipotent_char())),
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
}
