// This file is part of predicator. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/predicator/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of predicator. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/predicator/master/COPYRIGHT.


quick_error!
{
	#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
	pub enum InterProceduralOptimisationPassManagerError
	{
		CouldNotCreate
		{
			display("Could not create")
		}
		
		ModuleInvalidBeforeRunningPasses(error: String)
		{
			display("Module invalid before running passes {:?}", error)
		}
		
		CouldNotRunPassesOnModule
		{
			display("Could not run passes on module")
		}
		
		ModuleInvalidAfterRunningPasses(error: String)
		{
			display("Module invalid after running passes {:?}", error)
		}
	}
}
