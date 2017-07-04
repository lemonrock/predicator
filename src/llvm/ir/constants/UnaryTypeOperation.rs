// This file is part of mqtt. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/mqtt/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of mqtt. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/mqtt/master/COPYRIGHT.


#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum UnaryTypeOperation
{
	Trunc,
	SExt,
	ZExt,
	FPTrunc,
	FPExt,
	UIToFP,
	SIToFP,
	FPToUI,
	FPToSI,
	PtrToInt,
	IntToPtr,
	AddrSpaceCast,
	BitCast,
	ZExtOrBitCast,
	SExtOrBitCast,
	TruncOrBitCast,
	PointerCast,
	FPCast,
	
	IntCastUnsigned,
	IntCastSigned,
}

impl UnaryTypeOperation
{
	#[inline(always)]
	pub fn operate(&self, context: &Context, value: &Constant, to: &LlvmType) -> LLVMValueRef
	{
		use self::UnaryTypeOperation::*;
		
		let constantRef = context.constant(value).asLLVMValueRef();
		let typeRef = context.typeRef(to).asLLVMTypeRef();
		
		unsafe
		{
			match *self
			{
				Trunc => LLVMConstTrunc(constantRef, typeRef),
				SExt => LLVMConstSExt(constantRef, typeRef),
				ZExt => LLVMConstZExt(constantRef, typeRef),
				FPTrunc => LLVMConstFPTrunc(constantRef, typeRef),
				FPExt => LLVMConstFPExt(constantRef, typeRef),
				UIToFP => LLVMConstUIToFP(constantRef, typeRef),
				SIToFP => LLVMConstSIToFP(constantRef, typeRef),
				FPToUI => LLVMConstFPToUI(constantRef, typeRef),
				FPToSI => LLVMConstFPToSI(constantRef, typeRef),
				PtrToInt => LLVMConstPtrToInt(constantRef, typeRef),
				IntToPtr => LLVMConstIntToPtr(constantRef, typeRef),
				AddrSpaceCast => LLVMConstAddrSpaceCast(constantRef, typeRef),
				BitCast => LLVMConstBitCast(constantRef, typeRef),
				ZExtOrBitCast => LLVMConstZExtOrBitCast(constantRef, typeRef),
				SExtOrBitCast => LLVMConstSExtOrBitCast(constantRef, typeRef),
				TruncOrBitCast => LLVMConstTruncOrBitCast(constantRef, typeRef),
				PointerCast => LLVMConstPointerCast(constantRef, typeRef),
				FPCast => LLVMConstFPCast(constantRef, typeRef),
				
				IntCastUnsigned => LLVMConstIntCast(constantRef, typeRef, 0),
				IntCastSigned => LLVMConstIntCast(constantRef, typeRef, 1),
			}
		}
	}
}
