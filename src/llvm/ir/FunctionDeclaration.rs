// This file is part of predicator. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/predicator/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of predicator. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/predicator/master/COPYRIGHT.


pub struct FunctionDeclaration
{
	name: CString,
	returns: FunctionParameter,
	parameters: Vec<FunctionParameter>,
	hasVarArgs: bool,
	functionAttributes: HashSet<FunctionAttribute>,
	targetDependentFunctionAttributes: HashSet<TargetDependentFunctionAttribute>,
	callingConvention: UsefulLLVMCallConv,
	garbageCollectorStrategy: Option<CString>,
	
	linkage: UsefulLLVMLinkage,
	visibility: UsefulLLVMVisibility,
	dllStorageClass: Option<UsefulLLVMDLLStorageClass>,
	unnamedAddress: Option<UnnamedAddressAttribute>,
	alignment: Option<PowerOfTwoThirtyTwoBit>,
}

impl FunctionDeclaration
{
	#[inline(always)]
	pub fn intrinsic_llvm_memcpy_p0i8_p0i8_i64() -> Self
	{
		Self
		{
			name: CString::new("llvm.memcpy.p0i8.p0i8.i64").unwrap(),
			returns: FunctionParameter::void(),
			parameters: vec!
			[
				// i8* nocapture writeonly, i8* nocapture readonly, i64, i32, i1
				FunctionParameter::pointer(&LlvmType::Int8, hashset!
				{
					ParameterAttribute::nocapture,
					ParameterAttribute::writeonly,
				}),
				FunctionParameter::pointer(&LlvmType::Int8, hashset!
				{
					ParameterAttribute::nocapture,
					ParameterAttribute::readonly,
				}),
				FunctionParameter::simple(LlvmType::Int64),
				FunctionParameter::simple(LlvmType::Int32),
				FunctionParameter::boolean(),
			],
			hasVarArgs: false,
			functionAttributes: hashset!
			{
				FunctionAttribute::argmemonly,
				FunctionAttribute::nounwind
			},
			targetDependentFunctionAttributes: hashset!{},
			callingConvention: UsefulLLVMCallConv::LLVMCCallConv,
			garbageCollectorStrategy: None,
			
			linkage: UsefulLLVMLinkage::LLVMExternalLinkage,
			visibility: UsefulLLVMVisibility::LLVMDefaultVisibility,
			dllStorageClass: None,
			unnamedAddress: None,
			alignment: None,
		}
	}
	
	#[inline(always)]
	pub fn intrinsic(name: &str, returns: FunctionParameter, parameters: Vec<FunctionParameter>, functionAttributes: HashSet<FunctionAttribute>, targetDependentFunctionAttributes: HashSet<TargetDependentFunctionAttribute>) -> Self
	{
		Self
		{
			name: CString::new(name).unwrap(),
			returns: returns,
			parameters: parameters,
			hasVarArgs: false,
			functionAttributes: functionAttributes,
			targetDependentFunctionAttributes: targetDependentFunctionAttributes,
			callingConvention: UsefulLLVMCallConv::LLVMCCallConv,
			garbageCollectorStrategy: None,
			// prefix: ?,
			// prologue: ?,
			
			linkage: UsefulLLVMLinkage::LLVMExternalLinkage,
			visibility: UsefulLLVMVisibility::LLVMDefaultVisibility,
			dllStorageClass: None,
			unnamedAddress: None,
			alignment: None,
		}
	}
	
	#[inline(always)]
	pub(crate) fn create(&self, context: &Context, module: &Module) -> FunctionValue
	{
		let functionType = context.typeRef(&LlvmType::Function { returns: Box::new(self.returns.llvmType.clone()), parameters: self.parameters.iter().map(|ref functionParameter| functionParameter.llvmType.clone() ).collect(), hasVarArgs: self.hasVarArgs }).asLLVMTypeRef();
		let functionReference = unsafe { LLVMAddFunction(module.reference, self.name.as_ptr(), functionType) };
		
		let functionValue = FunctionValue::fromLLVMValueRef(functionReference);
		
		for attribute in self.returns.attributes.iter()
		{
			let attributeRef = context.parameterAttributeRef(attribute);
			unsafe { LLVMAddAttributeAtIndex(functionReference, LLVMAttributeReturnIndex, attributeRef) };
		}
		
		let mut parameterIndex = 1u32;
		for parameter in self.parameters.iter()
		{
			if let Some(alignment) = parameter.alignment
			{
				unsafe { LLVMSetParamAlignment(LLVMGetParam(functionReference, parameterIndex - 1), alignment.as_u32()) }
			}
			
			for attribute in parameter.attributes.iter()
			{
				let attributeRef = context.parameterAttributeRef(attribute);
				unsafe { LLVMAddAttributeAtIndex(functionReference, parameterIndex, attributeRef) };
			}
			parameterIndex += 1;
		}
		
		for attribute in self.functionAttributes.iter()
		{
			let attributeRef = context.functionAttributeRef(attribute);
			unsafe { LLVMAddAttributeAtIndex(functionReference, LLVMAttributeFunctionIndex, attributeRef) };
		}
		
		for attribute in self.targetDependentFunctionAttributes.iter()
		{
			attribute.addToFunction(functionValue);
		}
		
		unsafe { LLVMSetFunctionCallConv(functionReference, self.callingConvention as u32) };
		
		if let Some(ref garbageCollectorStrategy) = self.garbageCollectorStrategy
		{
			unsafe { LLVMSetGC(functionReference, garbageCollectorStrategy.as_ptr()) };
		}
		
		unsafe { LLVMSetLinkage(functionReference, self.linkage.to_LLVMLinkage()) };
		
		unsafe { LLVMSetVisibility(functionReference, self.visibility.to_LLVMVisibility()) };
		
		if let Some(ref dllStorageClass) = self.dllStorageClass
		{
			unsafe { LLVMSetDLLStorageClass(functionReference, dllStorageClass.to_LLVMDLLStorageClass()) };
		}
		
		if let Some(unnamedAddress) = self.unnamedAddress
		{
			use self::UnnamedAddressAttribute::*;
			
			match unnamedAddress
			{
				UnnamedAddress => unsafe { LLVMSetUnnamedAddr(functionReference, 1) },
				ModuleWideLocalUnnamedAddress => panic!("local unnamed address setting is not supported at this time for function declarations (we do not know which API call to use)"),
			}
		}
		
		if let Some(alignment) = self.alignment
		{
			unsafe { LLVMSetAlignment(functionReference, alignment.as_u32()) };
		}
		
		functionValue
	}
}
