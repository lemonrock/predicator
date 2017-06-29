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
	
	Struct { name: Option<CString>, body: StructBody },
	
	Function { returns: Box<LlvmType>, parameters: Vec<LlvmType>, hasVarArgs: bool },
	
	Array { elementType: Box<LlvmType>, numberOfElements: u32 },
	
	Vector { elementType: Box<LlvmType>, numberOfElements: u32 },
	
	Pointer { elementType: Box<LlvmType>, addressSpace: u32 },
}

impl LlvmType
{
	#[inline(always)]
	pub(crate) fn to_LLVMTypeRef(&self, C: LLVMContextRef, typeRefCache: &mut LLVMTypeRefCache) -> LLVMTypeRef
	{
		use self::LlvmType::*;
		
		if let Some(extant) = typeRefCache.get(self)
		{
			return *extant;
		}
		
		let value = unsafe
		{
			match *self
			{
				Int1 => LLVMInt1TypeInContext(C),
				Int8 => LLVMInt8TypeInContext(C),
				Int16 => LLVMInt16TypeInContext(C),
				Int32 => LLVMInt32TypeInContext(C),
				Int64 => LLVMInt64TypeInContext(C),
				Int128 => LLVMInt128TypeInContext(C),
				
				Float16 => LLVMHalfTypeInContext(C),
				Float32 => LLVMFloatTypeInContext(C),
				Float64 => LLVMDoubleTypeInContext(C),
				Float128 => LLVMFP128TypeInContext(C),
				Float80ForX86 => LLVMX86FP80TypeInContext(C),
				Float128ForPowerPCLegacy => LLVMPPCFP128TypeInContext(C),
				
				Void => LLVMVoidTypeInContext(C),
				Label => LLVMLabelTypeInContext(C),
				MmxX86 => LLVMX86MMXTypeInContext(C),
				
				Struct { ref name, ref body } =>
				{
					let mut ElementTypes: Vec<LLVMTypeRef> = body.elements.iter().map(|llvmType| llvmType.to_LLVMTypeRef(C, typeRefCache)).collect();
					
					let Packed = if body.isPacked
					{
						1
					}
					else
					{
						0
					};
					
					match name
					{
						&None => LLVMStructTypeInContext(C, ElementTypes.as_mut_ptr(), ElementTypes.len() as c_uint, Packed),
						
						&Some(ref name) =>
						{
							let StructTy = LLVMStructCreateNamed(C, name.as_ptr());
							LLVMStructSetBody(StructTy, ElementTypes.as_mut_ptr(), ElementTypes.len() as c_uint, Packed);
							StructTy
						}
					}
					
				}
				
				Function { ref returns, ref parameters, hasVarArgs } =>
				{
					let ReturnType = returns.to_LLVMTypeRef(C, typeRefCache);
					
					let mut ParamTypes: Vec<LLVMTypeRef> = parameters.iter().map(|llvmType| llvmType.to_LLVMTypeRef(C, typeRefCache)).collect();
					
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
				
				Array { ref elementType, ref numberOfElements } => LLVMArrayType(elementType.to_LLVMTypeRef(C, typeRefCache), *numberOfElements),
				
				Vector { ref elementType, ref numberOfElements } => LLVMVectorType(elementType.to_LLVMTypeRef(C, typeRefCache), *numberOfElements),
				
				Pointer { ref elementType, ref addressSpace } => LLVMPointerType(elementType.to_LLVMTypeRef(C, typeRefCache), *addressSpace),
			}
		};
		
		typeRefCache.insert(self.clone(), value);
		
		value
	}
	
	pub fn anonymousStruct(isPacked: bool, elements: Vec<LlvmType>) -> Self
	{
		LlvmType::Struct
		{
			name: None,
			body: StructBody
			{
				isPacked,
				elements,
			}
		}
	}
	
	pub fn namedStruct(name: &str, isPacked: bool, elements: Vec<LlvmType>) -> Self
	{
		LlvmType::Struct
		{
			name: Some(CString::new(name).unwrap()),
			body: StructBody
			{
				isPacked,
				elements,
			}
		}
	}
	
	pub fn pointer(of: LlvmType) -> Self
	{
		LlvmType::Pointer
		{
			elementType: Box::new(of),
			addressSpace: 0,
		}
	}
}
