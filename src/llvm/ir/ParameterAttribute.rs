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
	align { n: u64 },
	noalias,
	nocapture,
	nest,
	returned,
	nonnull,
	dereferenceable { n: u64 },
	dereferenceable_or_null { n: u64 },
	swiftself,
	swifterror,
}

impl Attribute for ParameterAttribute
{
	//noinspection SpellCheckingInspection
	fn to_value(&self) -> (EnumAttributeName, u64)
	{
		use self::ParameterAttribute::*;
		
		match *self
		{
			zeroext => (EnumAttributeName::zeroext, 0),
			signext => (EnumAttributeName::signext, 0),
			inreg => (EnumAttributeName::inreg, 0),
			byval => (EnumAttributeName::byval, 0),
			inalloca => (EnumAttributeName::inalloca, 0),
			sret => (EnumAttributeName::sret, 0),
			align { n } => (EnumAttributeName::align, n),
			noalias => (EnumAttributeName::noalias, 0),
			nocapture => (EnumAttributeName::nocapture, 0),
			nest => (EnumAttributeName::nest, 0),
			returned => (EnumAttributeName::returned, 0),
			nonnull => (EnumAttributeName::nonnull, 0),
			dereferenceable { n } => (EnumAttributeName::dereferenceable, n),
			dereferenceable_or_null { n } => (EnumAttributeName::dereferenceable_or_null, n),
			swiftself => (EnumAttributeName::swiftself, 0),
			swifterror => (EnumAttributeName::swifterror, 0),
		}
	}
}
