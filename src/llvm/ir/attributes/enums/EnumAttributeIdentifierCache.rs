// This file is part of predicator. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/predicator/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of predicator. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/predicator/master/COPYRIGHT.


#[derive(Debug, Clone)]
pub struct EnumAttributeIdentifierCache
{
	cache: Rc<RefCell<HashMap<EnumAttributeName, EnumAttributeIdentifier>>>
}

impl Default for EnumAttributeIdentifierCache
{
	#[inline(always)]
	fn default() -> Self
	{
		let capacity = unsafe { LLVMGetLastEnumAttributeKind() + 1 };
		Self
		{
			cache: Rc::new(RefCell::new(HashMap::with_capacity(capacity as usize))),
		}
	}
}

impl EnumAttributeIdentifierCache
{
	#[inline(always)]
	pub fn identifier(&self, enumAttributeName: EnumAttributeName) -> EnumAttributeIdentifier
	{
		*self.cache.borrow_mut().entry(enumAttributeName).or_insert_with(|| enumAttributeName.identifier())
	}
}
