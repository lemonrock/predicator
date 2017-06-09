// This file is part of predicator. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/predicator/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of predicator. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/predicator/master/COPYRIGHT.


#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum MemoryBufferCreator<'a>
{
	Buffer(&'a [u8]),
	
	/// We could use Path, but it is such a pain to get a *const c_char null terminated string from...
	File(&'a str),
}

impl<'a> MemoryBufferCreator<'a>
{
	// Potentially could be replaced by an implementation of the From trait
	#[inline(always)]
	pub fn createMemoryBuffer(&self) -> Result<MemoryBuffer<'a>, String>
	{
		use self::MemoryBufferCreator::*;
		
		match *self
		{
			Buffer(buffer) => Ok(MemoryBuffer::fromSlice(buffer)),
			File(filePath) => MemoryBuffer::fromFile(filePath),
		}
	}
}
