// This file is part of mqtt. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/mqtt/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of mqtt. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/mqtt/master/COPYRIGHT.


#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FunctionValue(LLVMValueRefWrapper);

impl Value for FunctionValue
{
	#[inline(always)]
	fn fromLLVMValueRef(value: LLVMValueRef) -> Self
	{
		FunctionValue(LLVMValueRefWrapper::fromLLVMValueRef(value))
	}
	
	#[inline(always)]
	fn asLLVMValueRef(&self) -> LLVMValueRef
	{
		self.0.asLLVMValueRef()
	}
}

impl FunctionValue
{
	#[inline(always)]
	pub fn parameterAt(&self, index: usize) -> Option<FunctionParameterValue>
	{
		if index >= self.numberOfParameters()
		{
			None
		}
		else
		{
			Some(FunctionParameterValue::fromLLVMValueRef(unsafe { LLVMGetParam(self.asLLVMValueRef(), index as u32) }))
		}
	}
	
	#[inline(always)]
	pub fn numberOfParameters(&self) -> usize
	{
		(unsafe { LLVMCountParams(self.asLLVMValueRef()) }) as usize
	}
	
	#[inline(always)]
	pub fn parameters(&self) -> Vec<FunctionParameterValue>
	{
		let numberOfParameters = self.numberOfParameters();
		let mut parameters = Vec::with_capacity(numberOfParameters);
		
		unsafe { LLVMGetParams(self.asLLVMValueRef(), parameters.as_mut_ptr()) };
		unsafe { parameters.set_len(numberOfParameters) };
		unsafe { transmute(parameters) }
	}
	
	#[inline(always)]
	pub fn createBlock<'a, S: Into<String> + Clone>(self, context: &'a Context, name: S) -> Block<'a>
	{
		Block::create(name, context, self)
	}
	
	#[inline(always)]
	pub fn setFunctionAttribute(&self, context: &Context, attribute: &FunctionAttribute)
	{
		let attributeRef = context.functionAttributeRef(attribute);
		self.setAttribute(LLVMAttributeFunctionIndex, attributeRef);
	}
	
	#[inline(always)]
	pub fn setFunctionReturnsAttribute(&self, context: &Context, attribute: &ParameterAttribute)
	{
		self.setFunctionParameterAttribute(context, LLVMAttributeReturnIndex, attribute);
	}
	
	#[inline(always)]
	pub fn setFunctionParameterAttribute(&self, context: &Context, parameterIndex: u32, attribute: &ParameterAttribute)
	{
		let attributeRef = context.parameterAttributeRef(attribute);
		self.setAttribute(parameterIndex, attributeRef);
	}
	
	#[inline(always)]
	pub(crate) fn setAttribute(&self, attributeIndex: u32, attributeRef: LLVMAttributeRef)
	{
		unsafe { LLVMAddAttributeAtIndex(self.asLLVMValueRef(), attributeIndex, attributeRef) };
	}
	
	#[inline(always)]
	pub fn setCallingConvention(&self, callingConvention: UsefulLLVMCallConv)
	{
		unsafe { LLVMSetFunctionCallConv(self.asLLVMValueRef(), callingConvention as u32) };
	}
	
	#[inline(always)]
	pub fn setGarbageCollectorStrategy(&self, garbageCollectorStrategy: &Option<CString>)
	{
		if let &Some(ref garbageCollectorStrategy) = garbageCollectorStrategy
		{
			unsafe { LLVMSetGC(self.asLLVMValueRef(), garbageCollectorStrategy.as_ptr()) };
		}
	}
	
	#[inline(always)]
	pub fn setLinkage(&self, linkage: UsefulLLVMLinkage)
	{
		unsafe { LLVMSetLinkage(self.asLLVMValueRef(), linkage.to_LLVMLinkage()) };
	}
	
	#[inline(always)]
	pub fn setVisibility(&self, visibility: UsefulLLVMVisibility)
	{
		unsafe { LLVMSetVisibility(self.asLLVMValueRef(), visibility.to_LLVMVisibility()) };
	}
	
	#[inline(always)]
	pub fn setDllStorageClass(&self, dllStorageClass: Option<UsefulLLVMDLLStorageClass>)
	{
		if let Some(dllStorageClass) = dllStorageClass
		{
			unsafe { LLVMSetDLLStorageClass(self.asLLVMValueRef(), dllStorageClass.to_LLVMDLLStorageClass()) };
		}
	}
	
	#[inline(always)]
	pub fn setAlignment(&self, alignment: Option<PowerOfTwoThirtyTwoBit>)
	{
		if let Some(alignment) = alignment
		{
			unsafe { LLVMSetAlignment(self.asLLVMValueRef(), alignment.as_u32()) };
		}
	}
	
	#[inline(always)]
	pub fn setUnnamedAddress(&self, hasUnnamedAddress: bool)
	{
		let HasUnnamedAddr = if hasUnnamedAddress
		{
			1
		}
		else
		{
			0
		};
		
		unsafe { LLVMSetUnnamedAddr(self.asLLVMValueRef(), HasUnnamedAddr) };
	}
	
	#[inline(always)]
	pub fn setSection(&self, section: &Option<String>)
	{
		if let Some(section) = section.as_ref().map(String::as_str)
		{
			let cSection = CString::new(section).expect("section contains embedded NULLs");
			unsafe { LLVMSetSection(self.asLLVMValueRef(), cSection.as_ptr()) };
		}
	}
	
	#[inline(always)]
	pub fn setPersonalityFunction(&self, personalityFunctionReference: Option<LLVMValueRefWrapper>)
	{
		if let Some(personalityFunctionReference) = personalityFunctionReference
		{
			unsafe { LLVMSetPersonalityFn(self.asLLVMValueRef(), personalityFunctionReference.asLLVMValueRef()) };
		}
	}
}
