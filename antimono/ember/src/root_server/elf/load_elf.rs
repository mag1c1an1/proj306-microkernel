// SPDX-License-Identifier: MPL-2.0

//! This module is used to parse elf file content to get elf_load_info.
//! When create a process from elf file, we will use the elf_load_info to construct the VmSpace


use alloc::collections::VecDeque;
use alloc::vec;
use alloc::vec::Vec;
use core::ops::Range;

use aster_frame::vm::{PAGE_SIZE, PageFlags, VmAllocOptions, VmFrame, VmFrameVec, VmIo, VmMapOptions, VmSpace};
use xmas_elf::program;
use xmas_elf::program::ProgramHeader64;

use sel4::sys::seL4_PageBits;

use crate::{bit, EmberResult, round_down, round_up};
use crate::common::region::Vaddr;
use crate::error::EmberError;
use crate::root_server::{SegmentDesc, UserImage};
use crate::root_server::elf::elf_file::Elf;

pub fn create_user_image(
    elf_binary: &[u8],
    ui_bounds: Range<Vaddr>,
    parsed_elf: &Elf,
) -> EmberResult<UserImage> {
    let mut descs = Vec::new();
    let phys = VmAllocOptions::new(ui_bounds.len() / PAGE_SIZE).is_contiguous(true).alloc()?.0;
    let mut deque = VecDeque::from(phys);
    for ph in &parsed_elf.program_headers {
        let typ = ph.get_type().map_err(|s| EmberError::Other(s))?;
        if typ == program::Type::Load {
            check_segment_align(ph)?;
            // init segment vmo
            trace!("mem range = 0x{:x} ~ 0x{:x}, mem_size = 0x{:x}",ph.virtual_addr,ph.virtual_addr + ph.mem_size,ph.mem_size);
            trace!("file range = 0x{:x} ~ 0x{:x}, file_size = 0x{:x}",ph.offset,ph.offset + ph.file_size,ph.file_size);
            let file_offset = ph.offset as usize;
            let file_size = ph.file_size as usize;
            let virtual_addr = ph.virtual_addr as usize;
            let vmap_start =
                round_down!(virtual_addr);
            let vmap_end = round_up!((virtual_addr + ph.mem_size as usize));
            let vmo_size = vmap_end - vmap_start;
            // align 4096
            assert_eq!(vmo_size % PAGE_SIZE, 0);
            let n_frames = vmo_size / PAGE_SIZE;
            let vm_seg = VmFrameVec(deque.drain(0..n_frames).collect::<Vec<_>>());
            assert_eq!(vm_seg.nbytes(), vmo_size);

            // Write zero as paddings. There are head padding and tail padding.
            // Head padding: if the segment's virtual address is not page-aligned,
            // then the bytes in first page from start to virtual address should be padded zeros.
            // Tail padding: If the segment's mem_size is larger than file size,
            // then the bytes that are not backed up by file content should be zeros.(usually .data/.bss sections).

            // Head padding.
            let page_offset = file_offset % PAGE_SIZE;
            if page_offset != 0 {
                let buffer = vec![0u8; page_offset];
                vm_seg.write_bytes(0, &buffer)?;
            }
            // Tail padding.
            let vm_seg_bytes = vm_seg.nbytes();
            let tail_padding_offset = file_size + page_offset;
            if vm_seg_bytes > tail_padding_offset {
                let buffer = vec![0u8; (vm_seg_bytes - tail_padding_offset) % PAGE_SIZE];
                vm_seg.write_bytes(tail_padding_offset, &buffer)?;
            }

            let slice = if file_size == 0 {
                &[]
            } else {
                &elf_binary[file_offset..file_offset + file_size]
            };

            vm_seg.write_slice(page_offset, slice)?;

            debug_assert!(file_offset % PAGE_SIZE == virtual_addr % PAGE_SIZE);

            // build map info but do not map
            let flags = parse_segment_perm(ph.flags);
            descs.push(SegmentDesc {
                segment: vm_seg,
                start: vmap_start,
                pt_flags: flags,
            });
        }
    }
    Ok(UserImage {
        descs,
        ui_bounds,
        elf_load_info: ElfLoadInfo {
            entry_point: parsed_elf.entry_point()
        },
    })
}


fn parse_segment_perm(flags: program::Flags) -> PageFlags {
    let mut pt_flags = PageFlags::empty();
    if flags.is_read() {
        pt_flags |= PageFlags::R;
    }
    if flags.is_write() {
        pt_flags |= PageFlags::W;
    }
    if flags.is_execute() {
        pt_flags |= PageFlags::X;
    }
    pt_flags
}

#[derive(Debug)]
pub struct ElfLoadInfo {
    entry_point: Vaddr,
}

impl ElfLoadInfo {
    pub fn new(entry_point: Vaddr) -> Self {
        Self {
            entry_point,
        }
    }

    pub fn entry_point(&self) -> Vaddr {
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
