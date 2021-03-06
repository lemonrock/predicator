// This file is part of predicator. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/predicator/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of predicator. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/predicator/master/COPYRIGHT.


#[derive(Debug, Clone)]
pub struct ModuleDefinition
{
	name: String,
	identifier: String,
	targetTriple: CString,
	targetMachineDataLayout: TargetMachineDataLayout,
	inlineAssembler: Option<String>,
	metadata: HashMap<String, MetadataNode>,
}

impl ModuleDefinition
{
	#[inline(always)]
	pub fn newForHost<S: Into<String> + Clone>(name: S) -> Result<Self, String>
	{
		let targetTriple = Target::defaultTargetTriple();
		let targetMachineDataLayout = Target::createHostTargetMachine(LLVMCodeGenOptLevel::LLVMCodeGenLevelNone)?.targetMachineDataLayout();
		
		Ok
		(
			Self
			{
				name: name.clone().into(),
				identifier: name.into(),
				targetTriple: targetTriple,
				targetMachineDataLayout: targetMachineDataLayout,
				inlineAssembler: None,
				metadata: hashmap!
				{
					"llvm.ident".to_owned() => MetadataNode::string("clang version 4.0.0 (tags/RELEASE_400/final)"),
				}
			}
		)
	}
	
	#[inline(always)]
	pub fn create(&self, context: &Context) -> Result<Module, String>
	{
		let module = context.createModule(&self.name, &self.identifier, &self.targetTriple, &self.targetMachineDataLayout, self.inlineAssembler.as_ref().map(String::as_str))?;
		
		for (key, metadata) in self.metadata.iter()
		{
			module.addMetadata(context, key, metadata);
		}
		
		Ok(module)
	}
}
