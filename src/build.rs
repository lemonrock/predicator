// This file is part of predicator. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/predicator/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of predicator. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/predicator/master/COPYRIGHT.


#![allow(non_snake_case)]


extern crate cpp_build;


use ::cpp_build::Config;
use ::std::env::var_os;
use ::std::ffi::OsStr;
use ::std::process::Command;
use ::std::process::Stdio;


fn main()
{
	// Also DEP_LLVM_LIBDIR
	let llvmConfigFilePath = var_os("DEP_LLVM_CONFIG_PATH").expect("Cargo dependency environment variable 'DEP_LLVM_CONFIG_PATH' from llvm-sys crate was not set");
	
	let mut config = Config::new();
	config.include(findLlvmIncludeFolderPath(&llvmConfigFilePath));
	addLlvmCxxArguments(&mut config, &llvmConfigFilePath);
	config.build("src/lib.rs");
}

fn addLlvmCxxArguments(config: &mut Config, llvmConfigFilePath: &OsStr)
{
	for flag in turnLlvmConfigOutputInputAString(llvmConfigFilePath, "--cxxflags").split_whitespace()
	{
		config.flag(flag);
	}
}

fn findLlvmIncludeFolderPath(llvmConfigFilePath: &OsStr) -> String
{
	turnLlvmConfigOutputInputAString(llvmConfigFilePath, "--includedir")
}

fn turnLlvmConfigOutputInputAString(llvmConfigFilePath: &OsStr, argumentName: &str) -> String
{
	let output = Command::new(llvmConfigFilePath).arg(argumentName).stdin(Stdio::null()).output().expect(&format!("Failed to execute llvm-config {}", argumentName));
	if !output.status.success()
	{
		panic!("llvm-config {} failed with output '{:?}'", argumentName, output);
	}
	if output.stdout.is_empty()
	{
		panic!("llvm-config {} was empty", argumentName);
	}
	
	// Strip trailing '\n'
	let mut vec = output.stdout;
	let lastIndex = vec.len() - 1;
	if vec[lastIndex] != '\n' as u8
	{
		panic!("llvm-config {} does not end in '\\n'", argumentName);
	}
	vec.pop();
	
	String::from_utf8(vec).expect(&format!("llvm-config {} result is not a UTF-8 string", argumentName))
}
