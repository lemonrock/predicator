// This file is part of predicator. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/predicator/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of predicator. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/predicator/master/COPYRIGHT.


#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FieldDefinition
{
	name: String,
	addressSpace: u32,
	llvmType: LlvmType,
	linkage: UsefulLLVMLinkage,
	visibility: UsefulLLVMVisibility,
	section: Option<String>,
	dllStorageClass: Option<UsefulLLVMDLLStorageClass>,
	hasUnnamedAddress: bool,
	alignment: Option<PowerOfTwoThirtyTwoBit>,
	fieldVariant: FieldVariant,
}

impl FieldDefinition
{
	pub fn create(&self, context: &Context, module: &Module) -> GlobalValue
	{
		let cName = CString::new(self.name.clone()).expect("name contains embedded NULLs");
		
		let globalValue = unsafe { LLVMAddGlobalInAddressSpace(module.reference, context.typeRef(&self.llvmType).asLLVMTypeRef(), cName.as_ptr(), self.addressSpace) };
		
		unsafe { LLVMSetLinkage(globalValue, self.linkage.to_LLVMLinkage()) };
		
		unsafe { LLVMSetVisibility(globalValue, self.visibility.to_LLVMVisibility()) };
		
		if let Some(section) = self.section.as_ref().map(String::as_str)
		{
			let cSection = CString::new(section).expect("section contains embedded NULLs");
			unsafe { LLVMSetSection(globalValue, cSection.as_ptr()) };
		}
		
		if let Some(ref dllStorageClass) = self.dllStorageClass
		{
			unsafe { LLVMSetDLLStorageClass(globalValue, dllStorageClass.to_LLVMDLLStorageClass()) };
		}
		
		let HasUnnamedAddr = if self.hasUnnamedAddress
		{
			1
		}
		else
		{
			0
		};
		
		unsafe { LLVMSetUnnamedAddr(globalValue, HasUnnamedAddr) };
		
		if let Some(alignment) = self.alignment
		{
			unsafe { LLVMSetAlignment(globalValue, alignment.as_u32()) };
		}
		
		let globalValue = GlobalValue::fromLLVMValueRef(globalValue);
		
		self.fieldVariant.set(context, globalValue);
		
		globalValue
	}
}
