// Copyright 2018 Google Inc.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     https://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use failure::Error;
use home::home_dir;
use std::path::PathBuf;
use std::{env, fs};

pub(crate) fn install() -> Result<(), Error> {
    let kernel_dir = get_kernel_dir()?;
    fs::create_dir_all(&kernel_dir)?;
    let current_exe_path = env::current_exe()?;
    let current_exe = current_exe_path
        .to_str()
        .ok_or_else(|| format_err!("current exe path isn't valid UTF-8"))?;
    let kernel_json = object!{
        "argv" => array![current_exe, "--control_file", "{connection_file}"],
        "display_name" => "Rust",
        "language" => "rust",
        "interrupt_mode" => "message",
    };
    let kernel_json_filename = kernel_dir.join("kernel.json");
    println!("Writing {}", kernel_json_filename.to_string_lossy());
    kernel_json.write_pretty(&mut fs::File::create(kernel_json_filename)?, 2)?;
    println!("Installation complete");
    Ok(())
}

pub(crate) fn uninstall() -> Result<(), Error> {
    let kernel_dir = get_kernel_dir()?;
    println!("Deleting {}", kernel_dir.to_string_lossy());
    fs::remove_dir_all(kernel_dir)?;
    println!("Uninstall complete");
    Ok(())
}

fn get_kernel_dir() -> Result<PathBuf, Error> {
    let home_dir = home_dir().ok_or_else(|| format_err!("Couldn't get home directory"))?;
    Ok(home_dir.join(".ipython").join("kernels").join("rust"))
}
