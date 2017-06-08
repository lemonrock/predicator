// This file is part of predicator. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/predicator/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of predicator. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/predicator/master/COPYRIGHT.


quick_error!
{
	#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
	pub enum FunctionPassManagerError
	{
		CouldNotCreate
		{
			display("Could not create")
		}
		
		CouldNotInitialize
		{
			display("Could not initialize")
		}
		
		FunctionInvalidBeforeRunningPasses(functionName: CString)
		{
			display("Function invalid before running passes {:?}", functionName)
		}
		
		CouldNotRunPassesOnFunction(functionName: CString)
		{
			display("Could not run passes on function {:?}", functionName)
		}
		
		FunctionInvalidAfterRunningPasses(functionName: CString)
		{
			display("Function invalid after running passes {:?}", functionName)
		}
		
		CouldNotFinalize
		{
			display("Could not finalize")
		}
	}
}
