// This file is part of predicator. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/predicator/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of predicator. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/predicator/master/COPYRIGHT.


//noinspection SpellCheckingInspection
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum EnumAttributeName
{
	// Function attributes
	alignstack,
	allocsize,
	alwaysinline,
	builtin,
	cold,
	convergent,
	inaccessiblememonly,
	inaccessiblemem_or_argmemonly,
	inlinehint,
	jumptable,
	minsize,
	naked,
	nobuiltin,
	noduplicate,
	noimplicitfloat,
	noinline,
	nonlazybind,
	noredzone,
	noreturn,
	norecurse,
	nounwind,
	optnone,
	optsize,
	patchable_function,
	probe_stack,
	stack_probe_size,
	argmemonly,
	safestack,
	sanitize_address,
	sanitize_memory,
	sanitize_thread,
	speculatable,
	ssp,
	sspreq,
	sspstrong,
	thunk,
	uwtable,
	
	// Parameter attributes
	zeroext,
	signext,
	inreg,
	byval,
	inalloca,
	sret,
	align,
	noalias,
	nocapture,
	nest,
	returned,
	nonnull,
	dereferenceable,
	dereferenceable_or_null,
	swiftself,
	swifterror,
	
	// Both
	readnone,
	readonly,
	writeonly,
}

impl EnumAttributeName
{
	#[inline(always)]
	pub fn identifier(self) -> EnumAttributeIdentifier
	{
		let name = self.cString();
		EnumAttributeIdentifier(unsafe { LLVMGetEnumAttributeKindForName(name.as_ptr() as *const _, name.len()) })
	}
	
	//noinspection SpellCheckingInspection
	#[inline(always)]
	fn cString(self) -> &'static [u8]
	{
		use self::EnumAttributeName::*;
		
		match self
		{
			alignstack => b"alignstack",
			allocsize => b"allocsize",
			alwaysinline => b"alwaysinline",
			builtin => b"builtin",
			cold => b"cold",
			convergent => b"convergent",
			inaccessiblememonly => b"inaccessiblememonly",
			inaccessiblemem_or_argmemonly => b"inaccessiblemem_or_argmemonly",
			inlinehint => b"inlinehint",
			jumptable => b"jumptable",
			minsize => b"minsize",
			naked => b"naked",
			nobuiltin => b"nobuiltin",
			noduplicate => b"noduplicate",
			noimplicitfloat => b"noimplicitfloat",
			noinline => b"noinline",
			nonlazybind => b"nonlazybind",
			noredzone => b"noredzone",
			noreturn => b"noreturn",
			norecurse => b"norecurse",
			nounwind => b"nounwind",
			optnone => b"optnone",
			optsize => b"optsize",
			patchable_function => b"patchable-function",
			probe_stack => b"probe-stack",
			readnone => b"readnone",
			readonly => b"readonly",
			stack_probe_size => b"stack-probe-size",
			writeonly => b"writeonly",
			argmemonly => b"argmemonly",
			safestack => b"safestack",
			sanitize_address => b"sanitize_address",
			sanitize_memory => b"sanitize_memory",
			sanitize_thread => b"sanitize_thread",
			speculatable => b"speculatable",
			ssp => b"ssp",
			sspreq => b"sspreq",
			sspstrong => b"sspstrong",
			thunk => b"thunk",
			uwtable => b"uwtable",
			
			zeroext => b"zeroext",
			signext => b"signext",
			inreg => b"inreg",
			byval => b"byval",
			inalloca => b"inalloca",
			sret => b"sret",
			align => b"align",
			noalias => b"noalias",
			nocapture => b"nocapture",
			nest => b"nest",
			returned => b"returned",
			nonnull => b"nonnull",
			dereferenceable => b"dereferenceable",
			dereferenceable_or_null => b"dereferenceable_or_null",
			swiftself => b"swiftself",
			swifterror => b"swifterror",
		}
	}
}
