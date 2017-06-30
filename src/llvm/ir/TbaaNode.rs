// This file is part of predicator. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/predicator/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of predicator. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/predicator/master/COPYRIGHT.


#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum TbaaNode
{
	Scalar
	{
		name: String,
		parent: Option<Box<TbaaNode>>,
		isConstant: bool,
	},
	
	Struct
	{
		name: String,
		fields: Vec<TbaaNodeStructField>
	},
	
	Path
	{
		baseType: Box<TbaaNode>, // eg a TbaaNode::Struct
		accessType: Box<TbaaNode>, // eg a TbaaNode::any_pointer()
		offsetIntoBaseType: u64,
		isConstant: bool,
	}
}

impl TbaaNode
{
	#[inline(always)]
	pub(crate) fn to_LLVMValueRef(&self, context: &Context) -> LLVMValueRef
	{
		use self::TbaaNode::*;
		
		match *self
		{
			Scalar { ref name, ref parent, ref isConstant } =>
			{
				let parentX = if let &Some(ref parentY) = parent
				{
					parentY.to_LLVMValueRef(context)
				}
				else
				{
					context.metadataString("Simple C/C++ TBAA")
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
					context.metadataString(name),
					parentX,
					context.integerConstant(&IntegerConstant::constantInteger64BitUnsigned(constantValue)),
				];
				
				unsafe { LLVMMDNodeInContext(context.reference, values.as_mut_ptr(), values.len() as u32) }
			}
			
			Struct { ref name, ref fields } =>
			{
				let mut values = Vec::with_capacity(1 + 2 * fields.len());
				values.push(context.metadataString(name));
				
				for field in fields.iter()
				{
					values.push(field.kind.to_LLVMValueRef(context));
					values.push(context.integerConstant(&IntegerConstant::constantInteger64BitUnsigned(field.offset)));
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
					baseType.to_LLVMValueRef(context),
					accessType.to_LLVMValueRef(context),
					context.integerConstant(&IntegerConstant::constantInteger64BitUnsigned(*offsetIntoBaseType)),
					context.integerConstant(&IntegerConstant::constantInteger64BitUnsigned(constantValue)),
				];
				
				unsafe { LLVMMDNodeInContext(context.reference, values.as_mut_ptr(), values.len() as u32) }
			}
		}
	}
	
	pub fn omnipotent_char() -> Self
	{
		TbaaNode::Scalar
		{
			name: "omnipotent char".to_owned(),
			parent: None,
			isConstant: false,
		}
	}
	
	pub fn short() -> Self
	{
		TbaaNode::Scalar
		{
			name: "short".to_owned(),
			parent: Some(Box::new(Self::omnipotent_char())),
			isConstant: false,
		}
	}
	
	pub fn int() -> Self
	{
		TbaaNode::Scalar
		{
			name: "int".to_owned(),
			parent: Some(Box::new(Self::omnipotent_char())),
			isConstant: false,
		}
	}
	
	pub fn long() -> Self
	{
		TbaaNode::Scalar
		{
			name: "long".to_owned(),
			parent: Some(Box::new(Self::omnipotent_char())),
			isConstant: false,
		}
	}
	
	pub fn long_long() -> Self
	{
		TbaaNode::Scalar
		{
			name: "long long".to_owned(),
			parent: Some(Box::new(Self::omnipotent_char())),
			isConstant: false,
		}
	}
	
	pub fn any_pointer() -> Self
	{
		TbaaNode::Scalar
		{
			name: "any pointer".to_owned(),
			parent: Some(Box::new(Self::omnipotent_char())),
			isConstant: false,
		}
	}
}
