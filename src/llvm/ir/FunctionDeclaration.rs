// This file is part of predicator. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/predicator/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of predicator. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/predicator/master/COPYRIGHT.


pub struct FunctionDeclaration
{
	name: CString,
	returns: FunctionParameter,
	parameters: Vec<FunctionParameter>,
	hasVarArgs: bool,
	functionAttributes: HashSet<FunctionAttribute>,
	callingConvention: UsefulCallingConvention, // Ordinarily LLVMCallConv isn't usable
	garbageCollectorStrategy: Option<CString>, // None
	personalityFunctionReference: Option<LLVMValue>, // None
}

impl FunctionDeclaration
{
	#[inline(always)]
	pub fn public(&self, name: &str, returns: FunctionParameter, parameters: Vec<FunctionParameter>, functionAttributes: HashSet<FunctionAttribute>) -> Self
	{
		Self
		{
			name: CString::new(name).unwrap(),
			returns: returns,
			parameters: parameters,
			hasVarArgs: false,
			functionAttributes: functionAttributes,
			callingConvention: UsefulCallingConvention::C,
			garbageCollectorStrategy: None,
			personalityFunctionReference: None,
		}
	}
	
	#[inline(always)]
	pub fn private(&self, name: &str, returns: FunctionParameter, parameters: Vec<FunctionParameter>, functionAttributes: HashSet<FunctionAttribute>) -> Self
	{
		Self
		{
			name: CString::new(name).unwrap(),
			returns: returns,
			parameters: parameters,
			hasVarArgs: false,
			functionAttributes: functionAttributes,
			callingConvention: UsefulCallingConvention::Fast,
			garbageCollectorStrategy: None,
			personalityFunctionReference: None,
		}
	}
	
	#[inline(always)]
	pub(crate) fn create<'a>(mut self, context: &'a Context, module: &Module) -> FunctionBuilder<'a>
	{
		let functionType = context.typeRef(&LlvmType::Function { returns: Box::new(self.returns.0), parameters: self.parameters.iter().map(|ref functionParameter| functionParameter.0.clone() ).collect(), hasVarArgs: self.hasVarArgs });
		let functionReference = unsafe { LLVMAddFunction(module.reference, self.name.as_ptr(), functionType) };
		
		for attribute in self.returns.1.drain()
		{
			let attributeRef = context.parameterAttributeRef(attribute);
			unsafe { LLVMAddAttributeAtIndex(functionReference, LLVMAttributeReturnIndex, attributeRef) };
		}
		
		let mut parameterIndex = 1u32;
		for parameter in self.parameters.iter_mut()
		{
			for attribute in parameter.1.drain()
			{
				let attributeRef = context.parameterAttributeRef(attribute);
				unsafe { LLVMAddAttributeAtIndex(functionReference, parameterIndex, attributeRef) };
			}
			parameterIndex += 1;
		}
		
		for attribute in self.functionAttributes.drain()
		{
			let attributeRef = context.functionAttributeRef(attribute);
			unsafe { LLVMAddAttributeAtIndex(functionReference, LLVMAttributeFunctionIndex, attributeRef) };
		}
		
		unsafe { LLVMSetFunctionCallConv(functionReference, self.callingConvention as u32) };
		
		if let Some(ref garbageCollectorStrategy) = self.garbageCollectorStrategy
		{
			unsafe { LLVMSetGC(functionReference, garbageCollectorStrategy.as_ptr()) };
		}
		
		if let Some(ref mut personalityFunctionReference) = self.personalityFunctionReference
		{
			unsafe { LLVMSetPersonalityFn(functionReference, personalityFunctionReference) };
		}
		
		FunctionBuilder
		{
			context: context,
			functionReference: functionReference,
		}
	}
}