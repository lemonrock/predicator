// This file is part of predicator. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/predicator/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of predicator. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/predicator/master/COPYRIGHT.


//noinspection SpellCheckingInspection
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum TargetFeature
{
	cx16,
	fxsr,
	mmx,
	sse,
	sse2,
	sse3,
	ssse3,
	x87,
}

impl TargetFeature
{
	//noinspection SpellCheckingInspection
	#[inline(always)]
	pub fn on(&self) -> &str
	{
		use self::TargetFeature::*;
		
		match *self
		{
			cx16 => "+cx16",
			fxsr => "+fxsr",
			mmx => "+mmx",
			sse => "+sse",
			sse2 => "+sse2",
			sse3 => "+sse3",
			ssse3 => "+ssse3",
			x87 => "+x87",
		}
	}
	
	//noinspection SpellCheckingInspection
	#[inline(always)]
	pub fn off(&self) -> &str
	{
		use self::TargetFeature::*;
		
		match *self
		{
			cx16 => "-cx16",
			fxsr => "-fxsr",
			mmx => "-mmx",
			sse => "-sse",
			sse2 => "-sse2",
			sse3 => "-sse3",
			ssse3 => "-ssse3",
			x87 => "-x87",
		}
	}
}
