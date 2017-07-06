// This file is part of mqtt. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/mqtt/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of mqtt. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/mqtt/master/COPYRIGHT.


trait NameOrEmptyName
{
	#[inline(always)]
	fn nameOrEmptyPointer(self)-> *const i8;
	
	#[doc(hidden)]
	#[inline(always)]
	fn emptyName() -> *const i8
	{
		b"\0".as_ptr() as *const _
	}
}

impl<'a> NameOrEmptyName for Option<&'a CStr>
{
	#[inline(always)]
	fn nameOrEmptyPointer(self)-> *const i8
	{
		if let Some(name) = self
		{
			name.as_ptr()
		}
		else
		{
			Self::emptyName()
		}
	}
}
