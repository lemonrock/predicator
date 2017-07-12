// This file is part of predicator. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/predicator/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of predicator. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/predicator/master/COPYRIGHT.


//noinspection SpellCheckingInspection
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum CallParameterAttribute
{
	zeroext, // Overlaps with parameter attributes
	signext, // Overlaps with parameter attributes
	inreg, // Overlaps with parameter attributes
	byval, // Overlaps with parameter attributes
}

impl ToReference<LLVMAttributeRef> for CallParameterAttribute
{
	//noinspection SpellCheckingInspection
	#[inline(always)]
	fn toReference(&self, context: &Context) -> LLVMAttributeRef
	{
		use self::CallParameterAttribute::*;
		
		match *self
		{
			zeroext => context.enumAttribute(EnumAttributeName::zeroext, 0),
			signext => context.enumAttribute(EnumAttributeName::signext, 0),
			inreg => context.enumAttribute(EnumAttributeName::inreg, 0),
			byval => context.enumAttribute(EnumAttributeName::byval, 0),
		}
	}
}
