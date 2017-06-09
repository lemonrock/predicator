// This file is part of predicator. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/predicator/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of predicator. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/predicator/master/COPYRIGHT.


#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ModuleSourceCodeType
{
	IntermediateRepresentation,
	BitCode,
}

impl ModuleSourceCodeType
{
	#[inline(always)]
	pub fn createVerifiedModule<'a>(&self, context: &Context, memoryBufferCreator: &MemoryBufferCreator<'a>) -> Result<Module, String>
	{
		use self::ModuleSourceCodeType::*;
		
		let memoryBuffer = memoryBufferCreator.createMemoryBuffer()?;
		
		let module = match *self
		{
			IntermediateRepresentation => context.parseTextualIntermediateRepresentationIntoModule(&memoryBuffer),
			BitCode => context.parseBitCodeIntoModule(&memoryBuffer),
		}?;
		
		module.verify()
	}
}
