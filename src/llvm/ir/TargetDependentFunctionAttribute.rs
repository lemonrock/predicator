// This file is part of predicator. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/predicator/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of predicator. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/predicator/master/COPYRIGHT.


#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum TargetDependentFunctionAttribute
{
	StringValueless(&'static [u8]),
	StringValue(&'static [u8], &'static [u8]),
	StringBoolean(&'static [u8], bool),
	StringPowerOfTwo(&'static [u8], PowerOfTwoThirtyTwoBit),
	StringFeatures(&'static [u8], Vec<ToggledTargetFeature>)
}

//noinspection SpellCheckingInspection
impl TargetDependentFunctionAttribute
{
	// PowerOfTwoThirtyTwoBit::_8
	pub fn stack_protector_buffer_size(size: PowerOfTwoThirtyTwoBit) -> Self
	{
		TargetDependentFunctionAttribute::StringPowerOfTwo(b"stack-protector-buffer-size\0", size)
	}
	
	// false
	pub fn disable_tail_calls(on: bool) -> Self
	{
		TargetDependentFunctionAttribute::StringBoolean(b"disable-tail-calls\0", on)
	}
	
	// true
	pub fn no_frame_pointer_elim(on: bool) -> Self
	{
		TargetDependentFunctionAttribute::StringBoolean(b"no-frame-pointer-elim\0", on)
	}
	
	pub const no_frame_pointer_elim_non_leaf: TargetDependentFunctionAttribute = TargetDependentFunctionAttribute::StringValueless(b"no-frame-pointer-elim-non-leaf\0");
	
	// false
	pub fn no_jump_tables(on: bool) -> Self
	{
		TargetDependentFunctionAttribute::StringBoolean(b"no-jump-tables\0", on)
	}
	
	// false
	pub fn correctly_rounded_divide_sqrt_fp_math(on: bool) -> Self
	{
		TargetDependentFunctionAttribute::StringBoolean(b"correctly-rounded-divide-sqrt-fp-math\0", on)
	}
	
	// false
	pub fn less_precise_fpmad(on: bool) -> Self
	{
		TargetDependentFunctionAttribute::StringBoolean(b"less-precise-fpmad\0", on)
	}
	
	// false
	pub fn no_infs_fp_math(on: bool) -> Self
	{
		TargetDependentFunctionAttribute::StringBoolean(b"no-infs-fp-math\0", on)
	}
	
	// false
	pub fn no_nans_fp_math(on: bool) -> Self
	{
		TargetDependentFunctionAttribute::StringBoolean(b"no-nans-fp-math\0", on)
	}
	
	// false
	pub fn no_signed_zeros_fp_math(on: bool) -> Self
	{
		TargetDependentFunctionAttribute::StringBoolean(b"no-signed-zeros-fp-math\0", on)
	}
	
	// false
	pub fn no_trapping_math(on: bool) -> Self
	{
		TargetDependentFunctionAttribute::StringBoolean(b"no-trapping-math\0", on)
	}
	
	// false
	pub fn unsafe_fp_math(on: bool) -> Self
	{
		TargetDependentFunctionAttribute::StringBoolean(b"unsafe-fp-math\0", on)
	}
	
	// false
	pub fn use_soft_float(on: bool) -> Self
	{
		TargetDependentFunctionAttribute::StringBoolean(b"use-soft-float\0", on)
	}
	
	// core2
	pub fn target_cpu(name: &'static [u8]) -> Self
	{
		TargetDependentFunctionAttribute::StringValue(b"target-cpu\0", name)
	}
	
	// vec![On(cx16), On(fxsr), On(cx16), On(sse), On(sse2), On(ssse3), On(x87)]
	pub fn target_features(features: Vec<ToggledTargetFeature>) -> Self
	{
		TargetDependentFunctionAttribute::StringFeatures(b"target-features\0", features)
	}
	
	fn addToFunction(&self, functionReference: LLVMValueRef)
	{
		use self::TargetDependentFunctionAttribute::*;
		
		unsafe
		{
			match *self
			{
				StringValueless(name) => LLVMAddTargetDependentFunctionAttr(functionReference, name.as_ptr() as *const _, null()),
				
				StringValue(name, value) => LLVMAddTargetDependentFunctionAttr(functionReference, name.as_ptr() as *const _, value.as_ptr() as *const _),
				
				StringBoolean(name, boolean) =>
					{
						if boolean
						{
							LLVMAddTargetDependentFunctionAttr(functionReference, name.as_ptr() as *const _, b"true\0".as_ptr() as *const _)
						}
						else
						{
							LLVMAddTargetDependentFunctionAttr(functionReference, name.as_ptr() as *const _, b"false\0".as_ptr() as *const _)
						}
					}
				
				StringPowerOfTwo(name, powerOfTwo) =>
					{
						let value = format!("{}", powerOfTwo.as_u32());
						
						let bytes = value.as_bytes();
						
						LLVMAddTargetDependentFunctionAttr(functionReference, name.as_ptr() as *const _, bytes.as_ptr() as *const _)
					}
				
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
						
						let bytes = features.as_bytes();
						
						LLVMAddTargetDependentFunctionAttr(functionReference, name.as_ptr() as *const _, bytes.as_ptr() as *const _)
					}
			}
		}
	}
}
