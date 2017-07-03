// This file is part of predicator. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/predicator/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of predicator. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/predicator/master/COPYRIGHT.


#[derive(Debug, Clone)]
pub struct ModuleDefinition
{
	pub name: String,
	pub identifier: String,
	pub targetTriple: CString,
	pub targetMachineDataLayout: TargetMachineDataLayout,
	pub inlineAssembler: Option<String>,
	pub fieldDefinitions: Vec<FieldDefinition>,
	pub metadata: HashMap<String, MetadataNode>,
}

impl ModuleDefinition
{
	pub fn newForHost<S: Into<String> + Clone>(name: S, fieldDefinitions: Vec<FieldDefinition>) -> Result<Self, String>
	{
		let targetTriple = Target::defaultTargetTriple();
		let targetMachineDataLayout = Target::createHostTargetMachine()?.targetMachineDataLayout();
		
		Ok
		(
			Self
			{
				name: name.clone().into(),
				identifier: name.into(),
				targetTriple: targetTriple,
				targetMachineDataLayout: targetMachineDataLayout,
				inlineAssembler: None,
				fieldDefinitions: fieldDefinitions,
				metadata: hashmap!
				{
					"llvm.ident".to_owned() => MetadataNode::string("clang version 4.0.0 (tags/RELEASE_400/final)"),
				}
			}
		)
	}
	
	#[inline(always)]
	pub fn create(&self, context: &Context) -> Result<(Module, HashMap<String, GlobalValue>), String>
	{
		let module = context.createModule(&self.name, &self.identifier, &self.targetTriple, &self.targetMachineDataLayout, self.inlineAssembler.as_ref().map(String::as_str))?;
		
		let mut fields = HashMap::with_capacity(self.fieldDefinitions.len());
		for fieldDefinition in self.fieldDefinitions.iter()
		{
			fields.insert(fieldDefinition.name.clone(), module.addField(context, fieldDefinition));
		}
		
		for (key, metadata) in self.metadata.iter()
		{
			module.addMetadata(context, key, metadata);
		}
		
		Ok((module, fields))
	}
}
