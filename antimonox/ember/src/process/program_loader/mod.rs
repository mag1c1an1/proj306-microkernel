// SPDX-License-Identifier: MPL-2.0

pub mod elf;
mod shebang;

use alloc::{ffi::CString, vec::Vec};

use crate::process::process_vm::ProcessVm;

use self::elf::{load_elf_to_vm, ElfLoadInfo};
use crate::Result;

/// Load an executable to root vmar, including loading programe image, preparing heap and stack,
/// initializing argv, envp and aux tables.
/// About recursion_limit: recursion limit is used to limit th recursion depth of shebang executables.
/// If the interpreter(the program behind #!) of shebang executable is also a shebang,
/// then it will trigger recursion. We will try to setup root vmar for the interpreter.
/// I guess for most cases, setting the recursion_limit as 1 should be enough.
/// because the interpreter is usually an elf binary(e.g., /bin/bash)
pub fn load_program_to_vm(
    process_vm: &ProcessVm,
    elf_binary: &[u8],
    argv: Vec<CString>,
    envp: Vec<CString>,
    is_root_server: bool,
) -> Result<ElfLoadInfo> {
    process_vm.clear_and_map();

    let elf_load_info = load_elf_to_vm(process_vm, elf_binary, argv, envp, is_root_server)?;

    Ok(elf_load_info)
}
