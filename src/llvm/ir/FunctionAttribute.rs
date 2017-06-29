// This file is part of predicator. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/predicator/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of predicator. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/predicator/master/COPYRIGHT.


//noinspection SpellCheckingInspection
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum FunctionAttribute
{
	alignstack {n: PowerOfTwoThirtyTwoBit},
	allocsize { EltSizeParam: u64, NumEltsParam: Option<u64> },
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
	
	StringValueless(&'static [u8]),
	StringValue(&'static [u8], &'static [u8]),
	StringBoolean(&'static [u8], bool),
	StringPowerOfTwo(&'static [u8], PowerOfTwoThirtyTwoBit),
	StringFeatures(&'static [u8], Vec<ToggledTargetFeature>)
}

impl Attribute for FunctionAttribute
{
	//noinspection SpellCheckingInspection
	fn to_attributeRef(&self, context: &Context) -> LLVMAttributeRef
	{
		use self::FunctionAttribute::*;
		
		match *self
		{
			alignstack {n} => context.enumAttribute(EnumAttributeName::alignstack, n.as_u32() as u64),
			allocsize { EltSizeParam, NumEltsParam } => context.enumAttribute(EnumAttributeName::allocsize, EltSizeParam as u64),
			alwaysinline => context.enumAttribute(EnumAttributeName::alwaysinline, 0),
			builtin => context.enumAttribute(EnumAttributeName::builtin, 0),
			cold => context.enumAttribute(EnumAttributeName::cold, 0),
			convergent => context.enumAttribute(EnumAttributeName::convergent, 0),
			inaccessiblememonly => context.enumAttribute(EnumAttributeName::inaccessiblememonly, 0),
			inaccessiblemem_or_argmemonly => context.enumAttribute(EnumAttributeName::inaccessiblemem_or_argmemonly, 0),
			inlinehint => context.enumAttribute(EnumAttributeName::inlinehint, 0),
			jumptable => context.enumAttribute(EnumAttributeName::jumptable, 0),
			minsize => context.enumAttribute(EnumAttributeName::minsize, 0),
			naked => context.enumAttribute(EnumAttributeName::naked, 0),
			nobuiltin => context.enumAttribute(EnumAttributeName::nobuiltin, 0),
			noduplicate => context.enumAttribute(EnumAttributeName::noduplicate, 0),
			noimplicitfloat => context.enumAttribute(EnumAttributeName::noimplicitfloat, 0),
			noinline => context.enumAttribute(EnumAttributeName::noinline, 0),
			nonlazybind => context.enumAttribute(EnumAttributeName::nonlazybind, 0),
			noredzone => context.enumAttribute(EnumAttributeName::noredzone, 0),
			noreturn => context.enumAttribute(EnumAttributeName::noreturn, 0),
			norecurse => context.enumAttribute(EnumAttributeName::norecurse, 0),
			nounwind => context.enumAttribute(EnumAttributeName::nounwind, 0),
			optnone => context.enumAttribute(EnumAttributeName::optnone, 0),
			optsize => context.enumAttribute(EnumAttributeName::optsize, 0),
			patchable_function => context.enumAttribute(EnumAttributeName::patchable_function, 0),
			probe_stack => context.enumAttribute(EnumAttributeName::probe_stack, 0),
			readnone => context.enumAttribute(EnumAttributeName::readnone, 0),
			readonly => context.enumAttribute(EnumAttributeName::readonly, 0),
			stack_probe_size => context.enumAttribute(EnumAttributeName::stack_probe_size, 0),
			writeonly => context.enumAttribute(EnumAttributeName::writeonly, 0),
			argmemonly => context.enumAttribute(EnumAttributeName::argmemonly, 0),
			safestack => context.enumAttribute(EnumAttributeName::safestack, 0),
			sanitize_address => context.enumAttribute(EnumAttributeName::sanitize_address, 0),
			sanitize_memory => context.enumAttribute(EnumAttributeName::sanitize_memory, 0),
			sanitize_thread => context.enumAttribute(EnumAttributeName::sanitize_thread, 0),
			speculatable => context.enumAttribute(EnumAttributeName::speculatable, 0),
			ssp => context.enumAttribute(EnumAttributeName::ssp, 0),
			sspreq => context.enumAttribute(EnumAttributeName::sspreq, 0),
			sspstrong => context.enumAttribute(EnumAttributeName::sspstrong, 0),
			thunk => context.enumAttribute(EnumAttributeName::thunk, 0),
			uwtable => context.enumAttribute(EnumAttributeName::uwtable, 0),
			
			StringValueless(name) => context.stringAttribute(name, None),
			StringValue(name, value) => context.stringAttribute(name, Some(value)),
			StringBoolean(name, boolean) =>
			{
				if boolean
				{
					context.stringAttribute(name, Some(b"true"))
				}
				else
				{
					context.stringAttribute(name, Some(b"false"))
				}
			}
			StringPowerOfTwo(name, powerOfTwo) =>
			{
				let value = format!("{}", powerOfTwo.as_u32());
				
				context.stringAttribute(name, Some(value.as_bytes()))
			},
			StringFeatures(name, ref toggledFeatures) =>
			{
				let mut features = String::with_capacity(32);
				let mut afterFirst = false;
				for toggledFeature in toggledFeatures.iter()
				{
					if afterFirst
					{
						features.push(',');
					}
					features.push_str(toggledFeature.value());
				}
				context.stringAttribute(name, Some(features.as_bytes()))
			}
		}
	}
}
