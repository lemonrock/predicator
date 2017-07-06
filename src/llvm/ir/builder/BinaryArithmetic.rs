// This file is part of mqtt. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/mqtt/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of mqtt. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/mqtt/master/COPYRIGHT.


#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum BinaryArithmetic
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
	Shl,
	LShr,
	AShr,
	And,
	Or,
	Xor,
}

impl BinaryArithmetic
{
	#[inline(always)]
	pub(crate) fn operate<LHS: Value, RHS: Value>(&self, builderReference: LLVMBuilderRef, leftHandSide: LHS, rightHandSide: RHS, name: Option<&CStr>) -> LLVMValueRefWrapper
	{
		use self::BinaryArithmetic::*;
		
		let leftHandSide = leftHandSide.asLLVMValueRef();
		let rightHandSide = rightHandSide.asLLVMValueRef();
		let name = name.nameOrEmptyPointer();
		
		let value = unsafe
		{
			match *self
			{
				Add => LLVMBuildAdd(builderReference, leftHandSide, rightHandSide, name),
				NSWAdd => LLVMBuildNSWAdd(builderReference, leftHandSide, rightHandSide, name),
				NUWAdd => LLVMBuildNUWAdd(builderReference, leftHandSide, rightHandSide, name),
				FAdd => LLVMBuildFAdd(builderReference, leftHandSide, rightHandSide, name),
				Sub => LLVMBuildSub(builderReference, leftHandSide, rightHandSide, name),
				NSWSub => LLVMBuildNSWSub(builderReference, leftHandSide, rightHandSide, name),
				NUWSub => LLVMBuildNUWSub(builderReference, leftHandSide, rightHandSide, name),
				FSub => LLVMBuildFSub(builderReference, leftHandSide, rightHandSide, name),
				Mul => LLVMBuildMul(builderReference, leftHandSide, rightHandSide, name),
				NSWMul => LLVMBuildNSWMul(builderReference, leftHandSide, rightHandSide, name),
				NUWMul => LLVMBuildNUWMul(builderReference, leftHandSide, rightHandSide, name),
				FMul => LLVMBuildFMul(builderReference, leftHandSide, rightHandSide, name),
				UDiv => LLVMBuildUDiv(builderReference, leftHandSide, rightHandSide, name),
				ExactUDiv => LLVMBuildExactUDiv(builderReference, leftHandSide, rightHandSide, name),
				SDiv => LLVMBuildSDiv(builderReference, leftHandSide, rightHandSide, name),
				ExactSDiv => LLVMBuildExactSDiv(builderReference, leftHandSide, rightHandSide, name),
				FDiv => LLVMBuildFDiv(builderReference, leftHandSide, rightHandSide, name),
				URem => LLVMBuildURem(builderReference, leftHandSide, rightHandSide, name),
				SRem => LLVMBuildSRem(builderReference, leftHandSide, rightHandSide, name),
				FRem => LLVMBuildFRem(builderReference, leftHandSide, rightHandSide, name),
				Shl => LLVMBuildShl(builderReference, leftHandSide, rightHandSide, name),
				LShr => LLVMBuildLShr(builderReference, leftHandSide, rightHandSide, name),
				AShr => LLVMBuildAShr(builderReference, leftHandSide, rightHandSide, name),
				And => LLVMBuildAnd(builderReference, leftHandSide, rightHandSide, name),
				Or => LLVMBuildOr(builderReference, leftHandSide, rightHandSide, name),
				Xor => LLVMBuildXor(builderReference, leftHandSide, rightHandSide, name),
			}
		};
		LLVMValueRefWrapper::fromLLVMValueRef(value)
	}
}
