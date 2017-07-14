// This file is part of mqtt. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/mqtt/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of mqtt. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/mqtt/master/COPYRIGHT.


#[derive(Clone)]
pub struct BlockFactory<'a>
{
	context: &'a Context,
	functionValue: FunctionValue,
}

impl<'a> BlockFactory<'a>
{
	#[inline(always)]
	pub fn new(context: &'a Context,functionValue: FunctionValue) -> Self
	{
		Self
		{
			context,
			functionValue
		}
	}
	
	#[inline(always)]
	pub fn child(&self) -> Block<'a>
	{
		self.functionValue.createBlock(self.context)
	}
}
