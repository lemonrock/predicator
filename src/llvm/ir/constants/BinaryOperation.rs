// This file is part of mqtt. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/mqtt/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of mqtt. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/mqtt/master/COPYRIGHT.


#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum BinaryOperation
{
	Add,
	NSWAdd,
	NUWAdd,
	FAdd,
	Sub,
	NSWSub,
	NUWSub,
	FSub,
	Mul,
	NSWMul,
	NUWMul,
	FMul,
	UDiv,
	ExactUDiv,
	SDiv,
	ExactSDiv,
	FDiv,
	URem,
	SRem,
	FRem,
	And,
	Or,
	Xor,
	Shl,
	LShr,
	AShr,
	
	ExtractElement,
}

impl BinaryOperation
{
	#[inline(always)]
	pub fn operate(&self, context: &Context, leftHandSide: &Constant, rightHandSide: &Constant) -> LLVMValueRef
	{
		use self::BinaryOperation::*;
		
		let leftHandSideRef = context.constant(leftHandSide).asLLVMValueRef();
		let rightHandSideRef = context.constant(rightHandSide).asLLVMValueRef();
		
		unsafe
		{
			match *self
			{
				Add => LLVMConstAdd(leftHandSideRef, rightHandSideRef),
				NSWAdd => LLVMConstNSWAdd(leftHandSideRef, rightHandSideRef),
				NUWAdd => LLVMConstNUWAdd(leftHandSideRef, rightHandSideRef),
				FAdd => LLVMConstFAdd(leftHandSideRef, rightHandSideRef),
				Sub => LLVMConstSub(leftHandSideRef, rightHandSideRef),
				NSWSub => LLVMConstNSWSub(leftHandSideRef, rightHandSideRef),
				NUWSub => LLVMConstNUWSub(leftHandSideRef, rightHandSideRef),
				FSub => LLVMConstFSub(leftHandSideRef, rightHandSideRef),
				Mul => LLVMConstMul(leftHandSideRef, rightHandSideRef),
				NSWMul => LLVMConstNSWMul(leftHandSideRef, rightHandSideRef),
				NUWMul => LLVMConstNUWMul(leftHandSideRef, rightHandSideRef),
				FMul => LLVMConstFMul(leftHandSideRef, rightHandSideRef),
				UDiv => LLVMConstUDiv(leftHandSideRef, rightHandSideRef),
				ExactUDiv => LLVMConstExactUDiv(leftHandSideRef, rightHandSideRef),
				SDiv => LLVMConstSDiv(leftHandSideRef, rightHandSideRef),
				ExactSDiv => LLVMConstExactSDiv(leftHandSideRef, rightHandSideRef),
				FDiv => LLVMConstFDiv(leftHandSideRef, rightHandSideRef),
				URem => LLVMConstURem(leftHandSideRef, rightHandSideRef),
				SRem => LLVMConstSRem(leftHandSideRef, rightHandSideRef),
				FRem => LLVMConstFRem(leftHandSideRef, rightHandSideRef),
				And => LLVMConstAnd(leftHandSideRef, rightHandSideRef),
				Or => LLVMConstOr(leftHandSideRef, rightHandSideRef),
				Xor => LLVMConstXor(leftHandSideRef, rightHandSideRef),
				Shl => LLVMConstShl(leftHandSideRef, rightHandSideRef),
				LShr => LLVMConstLShr(leftHandSideRef, rightHandSideRef),
				AShr => LLVMConstAShr(leftHandSideRef, rightHandSideRef),
				
				ExtractElement => LLVMConstExtractElement(leftHandSideRef, rightHandSideRef),
			}
		}
	}
}
