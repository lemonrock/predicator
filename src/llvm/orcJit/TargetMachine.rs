// This file is part of predicator. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/predicator/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of predicator. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/predicator/master/COPYRIGHT.


pub struct TargetMachine<'a>
{
	reference: LLVMTargetMachineRef,
	parent: &'a Target,
}

impl<'a> Drop for TargetMachine<'a>
{
	#[inline(always)]
	fn drop(&mut self)
	{
		if !self.reference.is_null()
		{
			unsafe { LLVMDisposeTargetMachine(self.reference) }
		}
	}
}

impl<'a> TargetMachine<'a>
{
	#[inline(always)]
	pub fn toOrcJitStack(mut self) -> OrcJitStack<'a>
	{
		// orcJitStackReference takes internal ownership of self.reference
		let orcJitStackReference = unsafe { LLVMOrcCreateInstance(self.reference) };
		self.reference = null_mut();
		
		OrcJitStack
		{
			reference: orcJitStackReference,
			parent: self.parent,
		}
	}
}
