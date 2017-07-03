// This file is part of predicator. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/predicator/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of predicator. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/predicator/master/COPYRIGHT.


#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FunctionParameter
{
	pub llvmType: LlvmType,
	pub alignment: Option<PowerOfTwoThirtyTwoBit>,
	pub attributes: HashSet<ParameterAttribute>,
}

impl FunctionParameter
{
	pub fn void() -> Self
	{
		Self::simple(LlvmType::Void)
	}
	
	pub fn boolean() -> Self
	{
		Self::simple(LlvmType::Int1)
	}
	
	pub fn simple(llvmType: LlvmType) -> Self
	{
		Self
		{
			llvmType: llvmType,
			alignment: None,
			attributes: hashset!
			{
			},
		}
	}
	
	pub fn wrap(llvmType: &LlvmType, attributes: HashSet<ParameterAttribute>) -> Self
	{
		Self
		{
			llvmType: llvmType.clone(),
			alignment: None,
			attributes: attributes,
		}
	}
	
	pub fn pointer(llvmType: &LlvmType, attributes: HashSet<ParameterAttribute>) -> Self
	{
		Self
		{
			llvmType: LlvmType::pointer(llvmType.clone()),
			alignment: None,
			attributes: attributes,
		}
	}
}
