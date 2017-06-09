// This file is part of predicator. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/predicator/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of predicator. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/predicator/master/COPYRIGHT.


pub struct TargetMachine
{
	reference: LLVMTargetMachineRef,
}

impl Drop for TargetMachine
{
	#[inline(always)]
	fn drop(&mut self)
	{
		// orcJitStackReference takes internal ownership of self.reference, so we don't dispose it if this is the case
		if !self.reference.is_null()
		{
			unsafe { LLVMDisposeTargetMachine(self.reference) }
		}
	}
}

impl TargetMachine
{
	#[inline(always)]
	pub fn toOrcJitStack(mut self) -> Result<OrcJitStack, String>
	{
		let orcJitStackReference = unsafe { LLVMOrcCreateInstance(self.reference) };
		
		if orcJitStackReference.is_null()
		{
			Err("Could not create ORC JIT stack".to_owned())
		}
		else
		{
			// orcJitStackReference takes internal ownership of self.reference
			self.reference = null_mut();
			
			Ok
			(
				OrcJitStack
				{
					reference: orcJitStackReference,
					dropWrapper: Rc::new(OrcJitStackDropWrapper(orcJitStackReference)),
				}
			)
		}
	}
}
