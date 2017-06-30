// This file is part of predicator. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/predicator/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of predicator. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/predicator/master/COPYRIGHT.


#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ModuleDefinition
{
	pub name: String,
	pub targetDataLayout: &'static str,
	pub targetTriple: &'static str,
	pub inlineAssembler: Option<String>,
	pub fieldDefinitions: Vec<FieldDefinition>,
}

impl ModuleDefinition
{
	pub fn newForAmd64Musl(name: String, fieldDefinitions: Vec<FieldDefinition>) -> Self
	{
		Self
		{
			name: name,
			targetDataLayout: "e-m:e-i64:64-f80:128-n8:16:32:64-S128",
			targetTriple: "x86_64-pc-linux-musl",
			inlineAssembler: None,
			fieldDefinitions: fieldDefinitions,
		}
	}
	
	#[inline(alway)]
	pub fn create(&self, context: &Context) -> Result<(Module, HashMap<String, LLVMValueRef>), String>
	{
		let module = context.createModule(&self.name, self.targetDataLayout, self.targetTriple, self.inlineAssembler.as_ref().map(String::as_str))?;
		
		let mut fields = HashMap::with_capacity(self.fieldDefinitions.len());
		for fieldDefinition in self.fieldDefinitions.iter()
		{
			fields.insert(fieldDefinition.name.clone(), module.addField(context, fieldDefinition));
		}
		
		Ok((module, fields))
	}
}
