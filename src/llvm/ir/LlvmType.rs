// This file is part of predicator. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/predicator/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of predicator. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/predicator/master/COPYRIGHT.


#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum LlvmType
{
	Int1,
	Int8,
	Int16,
	Int32,
	Int64,
	Int128,
	
	Float16,
	Float32,
	Float64,
	Float128,
	Float80ForX86,
	Float128ForPowerPCLegacy,
	
	Void,
	Label,
	MmxX86,
	
	Struct { name: Option<CString>, isPacked: bool, elements: Vec<LlvmType> },
	
	Function { returns: Box<LlvmType>, parameters: Vec<LlvmType>, hasVarArgs: bool },
	
	Array { elementType: Box<LlvmType>, numberOfElements: u32 },
	
	Vector { elementType: Box<LlvmType>, numberOfElements: u32 },
	
	Pointer { elementType: Box<LlvmType>, addressSpace: u32 },
}

impl ToReference<LLVMTypeRefWrapper> for LlvmType
{
	#[inline(always)]
	fn toReference(&self, context: &Context) -> LLVMTypeRefWrapper
	{
		use self::LlvmType::*;
		
		let value = unsafe
		{
			match *self
			{
				Int1 => LLVMInt1TypeInContext(context.reference),
				Int8 => LLVMInt8TypeInContext(context.reference),
				Int16 => LLVMInt16TypeInContext(context.reference),
				Int32 => LLVMInt32TypeInContext(context.reference),
				Int64 => LLVMInt64TypeInContext(context.reference),
				Int128 => LLVMInt128TypeInContext(context.reference),
				
				Float16 => LLVMHalfTypeInContext(context.reference),
				Float32 => LLVMFloatTypeInContext(context.reference),
				Float64 => LLVMDoubleTypeInContext(context.reference),
				Float128 => LLVMFP128TypeInContext(context.reference),
				Float80ForX86 => LLVMX86FP80TypeInContext(context.reference),
				Float128ForPowerPCLegacy => LLVMPPCFP128TypeInContext(context.reference),
				
				Void => LLVMVoidTypeInContext(context.reference),
				Label => LLVMLabelTypeInContext(context.reference),
				MmxX86 => LLVMX86MMXTypeInContext(context.reference),
				
				Struct { ref name, ref isPacked, ref elements } =>
				{
					let mut ElementTypes: Vec<LLVMTypeRef> = elements.iter().map(|llvmType| llvmType.toReference(context).asLLVMTypeRef()).collect();
					
					let Packed = if *isPacked
					{
						1
					}
					else
					{
						0
					};
					
					match name
					{
						&None => LLVMStructTypeInContext(context.reference, ElementTypes.as_mut_ptr(), ElementTypes.len() as c_uint, Packed),
						
						&Some(ref name) =>
						{
							let StructTy = LLVMStructCreateNamed(context.reference, name.as_ptr());
							LLVMStructSetBody(StructTy, ElementTypes.as_mut_ptr(), ElementTypes.len() as c_uint, Packed);
							StructTy
						}
					}
					
				}
				
				Function { ref returns, ref parameters, hasVarArgs } =>
				{
					let ReturnType = returns.toReference(context).asLLVMTypeRef();
					
					let mut ParamTypes: Vec<LLVMTypeRef> = parameters.iter().map(|llvmType| llvmType.toReference(context).asLLVMTypeRef()).collect();
					
					let IsVarArg = if hasVarArgs
					{
						1
					}
					else
					{
						0
					};
					
					LLVMFunctionType(ReturnType, ParamTypes.as_mut_ptr(), ParamTypes.len() as c_uint, IsVarArg)
				},
				
				Array { ref elementType, ref numberOfElements } => LLVMArrayType(elementType.toReference(context).asLLVMTypeRef(), *numberOfElements),
				
				Vector { ref elementType, ref numberOfElements } => LLVMVectorType(elementType.toReference(context).asLLVMTypeRef(), *numberOfElements),
				
				Pointer { ref elementType, ref addressSpace } => LLVMPointerType(elementType.toReference(context).asLLVMTypeRef(), *addressSpace),
			}
		};
		
		LLVMTypeRefWrapper(value)
	}
}

impl LlvmType
{
	#[inline(always)]
	pub fn int8Pointer() -> Self
	{
		Self::pointer(LlvmType::Int8)
	}
	
	#[inline(always)]
	pub fn anonymousStruct(isPacked: bool, elements: Vec<LlvmType>) -> Self
	{
		LlvmType::Struct
		{
			name: None,
			isPacked: isPacked,
			elements: elements,
		}
	}
	
	#[inline(always)]
	pub fn namedStruct(name: &str, isPacked: bool, elements: Vec<LlvmType>) -> Self
	{
		LlvmType::Struct
		{
			name: Some(CString::new(name).unwrap()),
			isPacked: isPacked,
			elements: elements,
		}
	}
	
	#[inline(always)]
	pub fn pointer(of: LlvmType) -> Self
	{
		LlvmType::Pointer
		{
			elementType: Box::new(of),
			addressSpace: 0,
		}
	}
}
