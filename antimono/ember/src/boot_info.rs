use core::ops::Range;
use core::ptr;
use core::ptr::NonNull;

use pod::Pod;
use spin::Once;

use crate::sel4::sys::{seL4_BootInfo, seL4_Domain, seL4_IPCBuffer, seL4_NodeId, seL4_SlotPos, seL4_SlotRegion, seL4_UntypedDesc, seL4_Word};
use crate::vspace::MemType;

pub static BOOT_INFO: Once<MemType<BootInfo>> = Once::new();


#[derive(Debug)]
pub struct BootInfoBuilder {
    extra_len: seL4_Word,
    node_id: seL4_NodeId,
    num_nodes: seL4_Word,
    num_io_pt_levels: seL4_Word,
    ipc_buffer: *mut seL4_IPCBuffer,
    empty: seL4_SlotRegion,
    shared_frames: seL4_SlotRegion,
    user_image_frames: seL4_SlotRegion,
    user_image_paging: seL4_SlotRegion,
    io_space_caps: seL4_SlotRegion,
    extra_bi_pages: seL4_SlotRegion,
    init_thread_cnode_size_bits: seL4_Word,
    init_thread_domain: seL4_Domain,
    untyped: seL4_SlotRegion,
    untyped_list: [seL4_UntypedDesc; 230usize],
}


#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
pub struct BootInfo(pub seL4_BootInfo);


unsafe impl Pod for BootInfo {}


unsafe impl Send for BootInfo {}

unsafe impl Sync for BootInfo {}

impl Default for BootInfoBuilder {
    fn default() -> Self {
        Self {
            extra_len: 0,
            node_id: 0,
            num_nodes: 0,
            num_io_pt_levels: 0,
            ipc_buffer: ptr::null_mut(),
            empty: Default::default(),
            shared_frames: Default::default(),
            user_image_frames: Default::default(),
            user_image_paging: Default::default(),
            io_space_caps: Default::default(),
            extra_bi_pages: Default::default(),
            init_thread_cnode_size_bits: 0,
            init_thread_domain: 0,
            untyped: Default::default(),
            untyped_list: [seL4_UntypedDesc::default(); 230usize],
        }
    }
}


impl BootInfoBuilder {
    pub fn new() -> Self {
        BootInfoBuilder::default()
    }
    pub fn extra_len(&mut self, extra_len: usize) -> &mut Self {
        self.extra_len = extra_len as seL4_Word;
        self
    }
    pub fn node_id(&mut self, node_id: usize) -> &mut Self {
        self.node_id = node_id as seL4_NodeId;
        self
    }
    pub fn num_nodes(&mut self, num_nodes: usize) -> &mut Self {
        self.num_nodes = num_nodes as seL4_Word;
        self
    }
    pub fn num_io_pt_levels(&mut self, num_io_pt_levels: usize) -> &mut Self {
        self.num_io_pt_levels = num_io_pt_levels as seL4_Word;
        self
    }
    pub fn ipc_buffer(&mut self, ipc_buffer: NonNull<seL4_IPCBuffer>) -> &mut Self {
        self.ipc_buffer = ipc_buffer.as_ptr();
        self
    }
    pub fn empty(&mut self, empty: Range<usize>) -> &mut Self {
        self.empty = range_to_slot_region(empty);
        self
    }
    pub fn shared_frames(&mut self, shared_frames: Range<usize>) -> &mut Self {
        self.empty = range_to_slot_region(shared_frames);
        self
    }
    pub fn user_image_frames(&mut self, user_image_frames: Range<usize>) -> &mut Self {
        self.empty = range_to_slot_region(user_image_frames);
        self
    }
    pub fn user_image_paging(&mut self, user_image_paging: Range<usize>) -> &mut Self {
        self.empty = range_to_slot_region(user_image_paging);
        self
    }
    pub fn io_space_caps(&mut self, io_space_caps: Range<usize>) -> &mut Self {
        self.empty = range_to_slot_region(io_space_caps);
        self
    }
    pub fn extra_bi_pages(&mut self, extra_bi_pages: Range<usize>) -> &mut Self {
        self.empty = range_to_slot_region(extra_bi_pages);
        self
    }
    pub fn init_thread_cnode_size_bits(&mut self, init_thread_cnode_size_bits: usize) -> &mut Self {
        self.init_thread_cnode_size_bits = init_thread_cnode_size_bits as seL4_Word;
        self
    }
    pub fn init_thread_domain(&mut self, init_thread_domain: usize) -> &mut Self {
        self.init_thread_domain = init_thread_domain as seL4_Domain;
        self
    }
    pub fn untyped(&mut self, untyped: Range<usize>) -> &mut Self {
        self.empty = range_to_slot_region(untyped);
        self
    }
    pub fn untyped_list(&mut self, untyped_list: &[seL4_UntypedDesc]) -> &mut Self {
        for (i, desc) in untyped_list.iter().enumerate() {
            self.untyped_list[i] = desc.clone();
        }
        self
    }
    pub fn build(&self) -> seL4_BootInfo {
        seL4_BootInfo {
            extraLen: self.extra_len,
            nodeID: self.node_id,
            numNodes: self.num_nodes,
            numIOPTLevels: self.num_io_pt_levels,
            ipcBuffer: self.ipc_buffer,
            empty: self.empty,
            sharedFrames: self.shared_frames,
            userImageFrames: self.user_image_frames,
            userImagePaging: self.user_image_paging,
            ioSpaceCaps: self.io_space_caps,
            extraBIPages: self.extra_bi_pages,
            initThreadCNodeSizeBits: self.init_thread_cnode_size_bits,
            initThreadDomain: self.init_thread_domain,
            untyped: self.untyped,
            untypedList: self.untyped_list,
        }
    }
}

fn range_to_slot_region(range: Range<usize>) -> seL4_SlotRegion {
    seL4_SlotRegion { start: range.start as seL4_SlotPos, end: 0 }
}


mod tests {
    use core::mem::size_of;

    use ktest::ktest;

    use crate::bit;
    use crate::sel4::sys::{seL4_BootInfo, seL4_PageBits};

    #[ktest]
    fn size_test() {
        assert!(size_of::<seL4_BootInfo>() < bit!(seL4_PageBits));
    }
}