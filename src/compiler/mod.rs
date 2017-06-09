// This file is part of predicator. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/predicator/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of predicator. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/predicator/master/COPYRIGHT.


use ::rand::os::OsRng;
use ::rand::Rng;
use ::std::env::temp_dir;
use ::std::env::var_os;
use ::std::fs::create_dir_all;
use ::std::fs::File;
use ::std::io;
use ::std::io::prelude::*;
use ::std::path::Path;
use ::std::path::PathBuf;
use ::std::process::Command;


pub struct RustPluginCompiler
{
	pub namedTemporaryFilePathGenerator: NamedTemporaryFilePathGenerator,
}

impl RustPluginCompiler
{
	pub fn example(&mut self) -> Result<String, String>
	{
		let mut pluginSourceFilePath = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
		pluginSourceFilePath.push("src");
		pluginSourceFilePath.push("sample.plugin.rs");
		self.compile("sample_plugin", &pluginSourceFilePath)
	}
	
	pub fn new(temporaryFolderPath: TemporaryFolderPath) -> Self
	{
		Self
		{
			namedTemporaryFilePathGenerator: NamedTemporaryFilePathGenerator::new(temporaryFolderPath, UsefulRandomFileNameGenerator::newForRustBitCode())
		}
	}
	
	pub fn compileToBuffer(&mut self, crateName: &str, pluginSourceFilePath: &Path) -> Result<Vec<u8>, String>
	{
		let bitCodeFilePath = self.compile(crateName, pluginSourceFilePath)?;
		match Self::fileToBuffer(&bitCodeFilePath)
		{
			Err(error) => Err(format!("Could not read bit code file because '{}'", error)),
			Ok(buffer) => Ok(buffer),
		}
	}
	
	pub fn compile(&mut self, crateName: &str, pluginSourceFilePath: &Path) -> Result<String, String>
	{
		let plugin_source_file_path = pluginSourceFilePath.to_owned().into_os_string().into_string().unwrap();
		let plugin_bit_code_file_path = self.namedTemporaryFilePathGenerator.generateBitCodeFilePath(crateName).to_owned().into_os_string().into_string().unwrap();
		
		let mut command = match var_os("RUSTC")
		{
			None => Command::new("rustc"),
			Some(path) => Command::new(path),
		};
		
		let result = command
		.arg("--crate-name").arg(crateName)
		.arg(plugin_source_file_path)
		.arg("--crate-type").arg("bin")
		.arg("--emit").arg("llvm-bc")
		.arg("-C").arg("opt-level=3")
		.arg("-C").arg("panic=abort")
		.arg("-C").arg("lto")
		.arg("-C").arg("relocation-model=static")
		.arg("-o").arg(&plugin_bit_code_file_path)
		.status();
		
		if result.is_err()
		{
			Err("Could not compile crate".to_owned())
		}
		else
		{
			Ok(plugin_bit_code_file_path)
		}
	}
	
	fn fileToBuffer(filePath: &str) -> Result<Vec<u8>, io::Error>
	{
		let mut bitCodeFile = File::open(filePath)?;
		let mut buffer = Vec::with_capacity(1024 * 64);
		bitCodeFile.read_to_end(&mut buffer)?;
		Ok(buffer)
	}
}

/// Note: This isn't completely secure
pub struct NamedTemporaryFilePathGenerator
{
	temporaryFolderPath: TemporaryFolderPath,
	usefulRandomFileNameGenerator: UsefulRandomFileNameGenerator,
}

impl NamedTemporaryFilePathGenerator
{
	#[inline(always)]
	pub fn new(temporaryFolderPath: TemporaryFolderPath, usefulRandomFileNameGenerator: UsefulRandomFileNameGenerator) -> Self
	{
		Self
		{
			temporaryFolderPath,
			usefulRandomFileNameGenerator,
		}
	}
	
	#[inline(always)]
	pub fn generateBitCodeFilePath(&mut self, crateName: &str) -> PathBuf
	{
		let mut path = self.temporaryFolderPath.path();
		path.push(self.usefulRandomFileNameGenerator.generateRustFileName(crateName));
		path
	}
}

pub enum TemporaryFolderPath
{
	TempDir,
	HomeFallingBackToTempDir,
	PathUnderHomeFallingBackToTempDir(PathBuf),
	OriginalOurDir,
	Other(PathBuf),
}

impl TemporaryFolderPath
{
	#[inline(always)]
	pub fn path(&self) -> PathBuf
	{
		use self::TemporaryFolderPath::*;
		
		let path = match *self
		{
			TempDir => temp_dir(),
			HomeFallingBackToTempDir => match var_os("HOME")
			{
				Some(home) => PathBuf::from(home),
				None => temp_dir(),
			},
			PathUnderHomeFallingBackToTempDir(ref relativePath) => match var_os("HOME")
			{
				Some(home) =>
				{
					let path = PathBuf::from(home);
					path.join(relativePath.clone())
				},
				None => temp_dir(),
			},
			OriginalOurDir => PathBuf::from(env!("OUT_DIR")),
			Other(ref path) => path.clone(),
		};
		
		if !path.exists()
		{
			match create_dir_all(&path)
			{
				Err(_) => (),
				Ok(_) => (),
			}
		}
		
		path
	}
}

pub struct UsefulRandomFileNameGenerator
{
	randomFileNameGenerator: RandomFileNameGenerator,
	separator1: String,
	numberOfRandomAsciiCharacters: usize,
	separator2: String,
	suffix: String,
}

impl UsefulRandomFileNameGenerator
{
	#[inline(always)]
	pub fn newForRustBitCode() -> Self
	{
		Self::new("-".to_owned(), 16, ".".to_owned(), "bc".to_owned())
	}
	
	#[inline(always)]
	pub fn new(separator1: String, numberOfRandomAsciiCharacters: usize, separator2: String, suffix: String) -> Self
	{
		Self
		{
			randomFileNameGenerator: RandomFileNameGenerator::new(),
			separator1: separator1,
			numberOfRandomAsciiCharacters: numberOfRandomAsciiCharacters,
			separator2: separator2,
			suffix: suffix,
		}
	}
	
	#[inline(always)]
	pub fn generateRustFileName(&mut self, crateName: &str) -> String
	{
		self.randomFileNameGenerator.generateRandomFileName(crateName, &self.separator1, self.numberOfRandomAsciiCharacters, &self.separator2, &self.suffix)
	}
}

pub struct RandomFileNameGenerator
{
	randomNumberGenerator: OsRng,
}

impl RandomFileNameGenerator
{
	#[inline(always)]
	pub fn new() -> Self
	{
		Self
		{
			randomNumberGenerator: OsRng::new().expect("Could not create a random number generator"),
		}
	}
	
	#[inline(always)]
	pub fn generateRandomFileName(&mut self, prefix: &str, separator1: &str, numberOfRandomAsciiCharacters: usize, separator2: &str, suffix: &str) -> String
	{
		let mut randomFileName = String::with_capacity(prefix.len() + separator1.len() + numberOfRandomAsciiCharacters + separator2.len() + suffix.len());
		randomFileName.push_str(prefix);
		randomFileName.push_str(separator1);
		let randomPart: String = self.randomNumberGenerator.gen_ascii_chars().take(numberOfRandomAsciiCharacters).collect();
		randomFileName.push_str(&randomPart);
		randomFileName.push_str(separator2);
		randomFileName.push_str(suffix);
		randomFileName
	}
}
