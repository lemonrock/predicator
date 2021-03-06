// This file is part of predicator. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/predicator/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of predicator. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/predicator/master/COPYRIGHT.


pub struct ObjectFile
{
	pub(crate) reference: LLVMObjectFileRef
}

impl Drop for ObjectFile
{
	#[inline(always)]
	fn drop(&mut self)
	{
		unsafe { LLVMDisposeObjectFile(self.reference) }
	}
}

impl ObjectFile
{
	#[inline(always)]
	pub fn create(memoryBuffer: &MemoryBuffer) -> Result<Self, ()>
	{
		let reference = unsafe { LLVMCreateObjectFile(memoryBuffer.reference) };
		if reference.is_null()
		{
			Err(())
		}
		else
		{
			Ok
			(
				Self
				{
					reference: reference
				}
			)
		}
	}
}
