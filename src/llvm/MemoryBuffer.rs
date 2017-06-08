// This file is part of predicator. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/predicator/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of predicator. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/predicator/master/COPYRIGHT.


pub struct MemoryBuffer<'a>
{
	reference: LLVMMemoryBufferRef,
	#[allow(dead_code)] slice: Option<&'a [u8]>,
}

impl<'a> Drop for MemoryBuffer<'a>
{
	#[inline(always)]
	fn drop(&mut self)
	{
		unsafe { LLVMDisposeMemoryBuffer(self.reference) }
	}
}

impl<'a> MemoryBuffer<'a>
{
	#[inline(always)]
	pub fn fromSlice(slice: &'a [u8]) -> Self
	{
		// "a\0"
		static BufferName: [i8; 2] = [65, 0];
		
		Self
		{
			reference: unsafe { LLVMCreateMemoryBufferWithMemoryRange(slice.as_ptr() as *const c_char, slice.len(), BufferName.as_ptr(), 1) },
			slice: Some(slice),
		}
	}
	
	#[inline(always)]
	pub fn fromFile(filePath: &str) -> Result<Self, String>
	{
		let filePath = CString::new(filePath).expect("File path contains embedded NULs");
		
		let mut reference = unsafe { uninitialized() };
		let mut errorMessage = null_mut();
		let boolean = unsafe { LLVMCreateMemoryBufferWithContentsOfFile(filePath.as_ptr(), &mut reference, &mut errorMessage) };
		handle_boolean_and_error_message!(boolean, errorMessage, LLVMParseIRInContext);
		
		Ok
		(
			Self
			{
				reference: reference,
				slice: None,
			}
		)
	}
	
	#[inline(always)]
	pub fn size(&self) -> usize
	{
		unsafe { LLVMGetBufferSize(self.reference) }
	}
	
	#[inline(always)]
	pub fn address(&self) -> *const c_char
	{
		unsafe { LLVMGetBufferStart(self.reference) }
	}
}
