// This file is part of predicator. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/predicator/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of predicator. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/predicator/master/COPYRIGHT.


//noinspection SpellCheckingInspection
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ParameterAttribute
{
	zeroext,
	signext,
	inreg,
	byval,
	inalloca,
	sret,
	align { n: PowerOfTwoThirtyTwoBit },
	noalias,
	nocapture,
	nest,
	returned,
	nonnull,
	dereferenceable { n: u64 },
	dereferenceable_or_null { n: u64 },
	swiftself,
	swifterror,
	
	StringValueless(&'static [u8]),
	StringValue(&'static [u8], &'static [u8]),
	StringBoolean(&'static [u8], bool),
	StringPowerOfTwo(&'static [u8], PowerOfTwoThirtyTwoBit),
}

impl Attribute for ParameterAttribute
{
	//noinspection SpellCheckingInspection
	fn to_attributeRef(&self, context: &Context) -> LLVMAttributeRef
	{
		use self::ParameterAttribute::*;
		
		match *self
		{
			zeroext => context.enumAttribute(EnumAttributeName::zeroext, 0),
			signext => context.enumAttribute(EnumAttributeName::signext, 0),
			inreg => context.enumAttribute(EnumAttributeName::inreg, 0),
			byval => context.enumAttribute(EnumAttributeName::byval, 0),
			inalloca => context.enumAttribute(EnumAttributeName::inalloca, 0),
			sret => context.enumAttribute(EnumAttributeName::sret, 0),
			align { n } => context.enumAttribute(EnumAttributeName::align, n.as_u32() as u64),
			noalias => context.enumAttribute(EnumAttributeName::noalias, 0),
			nocapture => context.enumAttribute(EnumAttributeName::nocapture, 0),
			nest => context.enumAttribute(EnumAttributeName::nest, 0),
			returned => context.enumAttribute(EnumAttributeName::returned, 0),
			nonnull => context.enumAttribute(EnumAttributeName::nonnull, 0),
			dereferenceable { n } => context.enumAttribute(EnumAttributeName::dereferenceable, n),
			dereferenceable_or_null { n } => context.enumAttribute(EnumAttributeName::dereferenceable_or_null, n),
			swiftself => context.enumAttribute(EnumAttributeName::swiftself, 0),
			swifterror => context.enumAttribute(EnumAttributeName::swifterror, 0),
			
			StringValueless(name) => context.stringAttribute(name, None),
			StringValue(name, value) => context.stringAttribute(name, Some(value)),
			StringBoolean(name, boolean) =>
			{
				if boolean
				{
					context.stringAttribute(name, Some(b"true"))
				}
				else
				{
					context.stringAttribute(name, Some(b"false"))
				}
			}
			StringPowerOfTwo(name, powerOfTwo) =>
			{
				let value = format!("{}", powerOfTwo.as_u32());
				
				context.stringAttribute(name, Some(value.as_bytes()))
			},
		}
	}
}
