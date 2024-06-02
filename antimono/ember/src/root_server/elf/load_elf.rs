// SPDX-License-Identifier: MPL-2.0

//! This module is used to parse elf file content to get elf_load_info.
//! When create a process from elf file, we will use the elf_load_info to construct the VmSpace


use aster_frame::vm::{PAGE_SIZE, VmAllocOptions, VmMapOptions, VmSpace};
use xmas_elf::program;
use xmas_elf::program::ProgramHeader64;

use sel4::sys::seL4_PageBits;

use crate::{EmberResult, round_up};
use crate::common::region::VAddr;
use crate::error::EmberError;
use crate::root_server::elf::elf_file::Elf;

/// Loads elf to the process vm.
///
/// This function will map elf segments and
/// initialize process init stack.
pub fn load_elf(
    elf_binary: &[u8],
) -> EmberResult<ElfLoadInfo> {
    let parsed_elf = Elf::parse_elf(elf_binary)?;
    let range = parsed_elf.memory_bounds();
    error!("mem bounds 0x{:x}, 0x{:x}",range.start, range.end);
    let num = round_up!((range.end - range.start) as usize,seL4_PageBits) / 4096;
    let vm_seg = VmAllocOptions::new(num as usize).is_contiguous(true).alloc()?;
    for ph in &parsed_elf.program_headers {
        let typ = ph.get_type().map_err(|s| EmberError::Other(s))?;
        if typ == program::Type::Load {
            check_segment_align(ph)?;
            trace!("mem range = 0x{:x} ~ 0x{:x}, mem_size = 0x{:x}",ph.virtual_addr,ph.virtual_addr + ph.mem_size,ph.mem_size);
            trace!("file range = 0x{:x} ~ 0x{:x}, file_size = 0x{:x}",ph.offset,ph.offset + ph.file_size,ph.file_size);
            let file_offset = ph.offset as usize;
            let file_size = ph.file_size as usize;
            let virtual_addr = ph.virtual_addr as usize;
            debug_assert!(file_offset % PAGE_SIZE == virtual_addr % PAGE_SIZE);
        }
    }
    Ok(ElfLoadInfo::new(parsed_elf.entry_point()))
    // VmMapOptions::new()
    // vm_space.map()
}

pub struct ElfLoadInfo {
    entry_point: VAddr,
}

impl ElfLoadInfo {
    pub fn new(entry_point: VAddr) -> Self {
        Self {
            entry_point,
        }
    }

    pub fn entry_point(&self) -> VAddr {
        self.entry_point
    }
}

fn check_segment_align(program_header: &ProgramHeader64) -> EmberResult<()> {
    let align = program_header.align;
    if align == 0 || align == 1 {
        // no align requirement
        return Ok(());
    }
    debug_assert!(align.is_power_of_two());
    if !align.is_power_of_two() {
        return Err(EmberError::Other("segment align is invalid."));
    }
    debug_assert!(program_header.offset % align == program_header.virtual_addr % align);
    if program_header.offset % align != program_header.virtual_addr % align {
        return Err(EmberError::Other("segment align is not satisfied."));
    }
    Ok(())
}
