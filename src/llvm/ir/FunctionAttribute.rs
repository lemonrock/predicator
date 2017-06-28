// This file is part of predicator. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/predicator/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of predicator. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/predicator/master/COPYRIGHT.


//noinspection SpellCheckingInspection
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum FunctionAttribute
{
	alignstack {n: u8},
	allocsize { EltSizeParam: u8, NumEltsParam: Option<u8> },
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
	readnone,
	readonly,
	stack_probe_size,
	writeonly,
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
}

impl Attribute for FunctionAttribute
{
	//noinspection SpellCheckingInspection
	fn to_value(&self) -> (EnumAttributeName, u64)
	{
		use self::FunctionAttribute::*;
		
		match *self
		{
			alignstack {n} => (EnumAttributeName::alignstack, n as u64),
			allocsize { EltSizeParam, NumEltsParam } => (EnumAttributeName::allocsize, EltSizeParam as u64),
			alwaysinline => (EnumAttributeName::alwaysinline, 0),
			builtin => (EnumAttributeName::builtin, 0),
			cold => (EnumAttributeName::cold, 0),
			convergent => (EnumAttributeName::convergent, 0),
			inaccessiblememonly => (EnumAttributeName::inaccessiblememonly, 0),
			inaccessiblemem_or_argmemonly => (EnumAttributeName::inaccessiblemem_or_argmemonly, 0),
			inlinehint => (EnumAttributeName::inlinehint, 0),
			jumptable => (EnumAttributeName::jumptable, 0),
			minsize => (EnumAttributeName::minsize, 0),
			naked => (EnumAttributeName::naked, 0),
			nobuiltin => (EnumAttributeName::nobuiltin, 0),
			noduplicate => (EnumAttributeName::noduplicate, 0),
			noimplicitfloat => (EnumAttributeName::noimplicitfloat, 0),
			noinline => (EnumAttributeName::noinline, 0),
			nonlazybind => (EnumAttributeName::nonlazybind, 0),
			noredzone => (EnumAttributeName::noredzone, 0),
			noreturn => (EnumAttributeName::noreturn, 0),
			norecurse => (EnumAttributeName::norecurse, 0),
			nounwind => (EnumAttributeName::nounwind, 0),
			optnone => (EnumAttributeName::optnone, 0),
			optsize => (EnumAttributeName::optsize, 0),
			patchable_function => (EnumAttributeName::patchable_function, 0),
			probe_stack => (EnumAttributeName::probe_stack, 0),
			readnone => (EnumAttributeName::readnone, 0),
			readonly => (EnumAttributeName::readonly, 0),
			stack_probe_size => (EnumAttributeName::stack_probe_size, 0),
			writeonly => (EnumAttributeName::writeonly, 0),
			argmemonly => (EnumAttributeName::argmemonly, 0),
			safestack => (EnumAttributeName::safestack, 0),
			sanitize_address => (EnumAttributeName::sanitize_address, 0),
			sanitize_memory => (EnumAttributeName::sanitize_memory, 0),
			sanitize_thread => (EnumAttributeName::sanitize_thread, 0),
			speculatable => (EnumAttributeName::speculatable, 0),
			ssp => (EnumAttributeName::ssp, 0),
			sspreq => (EnumAttributeName::sspreq, 0),
			sspstrong => (EnumAttributeName::sspstrong, 0),
			thunk => (EnumAttributeName::thunk, 0),
			uwtable => (EnumAttributeName::uwtable, 0),
		}
	}
}
