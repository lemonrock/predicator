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
	pub(crate) fn operate<LHS: Value, RHS: Value>(&self, builderReference: LLVMBuilderRef, leftHandSide: LHS, rightHandSide: RHS) -> LLVMValueRefWrapper
	{
		use self::BinaryArithmetic::*;
		
		let leftHandSide = leftHandSide.asLLVMValueRef();
		let rightHandSide = rightHandSide.asLLVMValueRef();
		
		let value = unsafe
		{
			match *self
			{
				Add => LLVMBuildAdd(builderReference, leftHandSide, rightHandSide, emptyName!()),
				NSWAdd => LLVMBuildNSWAdd(builderReference, leftHandSide, rightHandSide, emptyName!()),
				NUWAdd => LLVMBuildNUWAdd(builderReference, leftHandSide, rightHandSide, emptyName!()),
				FAdd => LLVMBuildFAdd(builderReference, leftHandSide, rightHandSide, emptyName!()),
				Sub => LLVMBuildSub(builderReference, leftHandSide, rightHandSide, emptyName!()),
				NSWSub => LLVMBuildNSWSub(builderReference, leftHandSide, rightHandSide, emptyName!()),
				NUWSub => LLVMBuildNUWSub(builderReference, leftHandSide, rightHandSide, emptyName!()),
				FSub => LLVMBuildFSub(builderReference, leftHandSide, rightHandSide, emptyName!()),
				Mul => LLVMBuildMul(builderReference, leftHandSide, rightHandSide, emptyName!()),
				NSWMul => LLVMBuildNSWMul(builderReference, leftHandSide, rightHandSide, emptyName!()),
				NUWMul => LLVMBuildNUWMul(builderReference, leftHandSide, rightHandSide, emptyName!()),
				FMul => LLVMBuildFMul(builderReference, leftHandSide, rightHandSide, emptyName!()),
				UDiv => LLVMBuildUDiv(builderReference, leftHandSide, rightHandSide, emptyName!()),
				ExactUDiv => LLVMBuildExactUDiv(builderReference, leftHandSide, rightHandSide, emptyName!()),
				SDiv => LLVMBuildSDiv(builderReference, leftHandSide, rightHandSide, emptyName!()),
				ExactSDiv => LLVMBuildExactSDiv(builderReference, leftHandSide, rightHandSide, emptyName!()),
				FDiv => LLVMBuildFDiv(builderReference, leftHandSide, rightHandSide, emptyName!()),
				URem => LLVMBuildURem(builderReference, leftHandSide, rightHandSide, emptyName!()),
				SRem => LLVMBuildSRem(builderReference, leftHandSide, rightHandSide, emptyName!()),
				FRem => LLVMBuildFRem(builderReference, leftHandSide, rightHandSide, emptyName!()),
				Shl => LLVMBuildShl(builderReference, leftHandSide, rightHandSide, emptyName!()),
				LShr => LLVMBuildLShr(builderReference, leftHandSide, rightHandSide, emptyName!()),
				AShr => LLVMBuildAShr(builderReference, leftHandSide, rightHandSide, emptyName!()),
				And => LLVMBuildAnd(builderReference, leftHandSide, rightHandSide, emptyName!()),
				Or => LLVMBuildOr(builderReference, leftHandSide, rightHandSide, emptyName!()),
				Xor => LLVMBuildXor(builderReference, leftHandSide, rightHandSide, emptyName!()),
			}
		};
		LLVMValueRefWrapper::fromLLVMValueRef(value)
	}
}
