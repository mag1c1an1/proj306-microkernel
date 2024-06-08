use alloc::boxed::Box;
use core::ops::Range;
use core::ptr::{NonNull, null_mut};

use pod::Pod;
use spin::Once;

use crate::sel4::sys::{seL4_BootInfo, seL4_Domain, seL4_IPCBuffer, seL4_NodeId, seL4_SlotPos, seL4_SlotRegion, seL4_UntypedDesc, seL4_Word};
use crate::vspace::MemType;

pub static BOOT_INFO: Once<MemType<BootInfo>> = Once::new();

#[derive(Debug, Default)]
pub struct BootState {
    pub bi_builder: BootInfoBuilder,
    pub cur_slot_pos: seL4_SlotPos,
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
pub struct BootInfo(pub seL4_BootInfo);


unsafe impl Pod for BootInfo {}

unsafe impl Send for BootInfo {}

unsafe impl Sync for BootInfo {}

#[derive(Debug)]
pub struct BootInfoBuilder {
    inner: Box<BootInfo>,
}

impl Default for BootInfoBuilder {
    fn default() -> Self {
        Self {
            inner: Box::new(BootInfo(seL4_BootInfo {
                extraLen: 0,
                nodeID: 0,
                numNodes: 0,
                numIOPTLevels: 0,
                ipcBuffer: null_mut(),
                empty: Default::default(),
                sharedFrames: Default::default(),
                userImageFrames: Default::default(),
                userImagePaging: Default::default(),
                ioSpaceCaps: Default::default(),
                extraBIPages: Default::default(),
                initThreadCNodeSizeBits: 0,
                initThreadDomain: 0,
                untyped: Default::default(),
                untypedList: [seL4_UntypedDesc::default(); 230usize],
            }))
        }
    }
}


impl BootInfoBuilder {
    pub fn new() -> Self {
        BootInfoBuilder::default()
    }
    pub fn extra_len(&mut self, extra_len: usize) -> &mut Self {
        self.inner.0.extraLen = extra_len as seL4_Word;
        self
    }
    pub fn node_id(&mut self, node_id: usize) -> &mut Self {
        self.inner.0.nodeID = node_id as seL4_NodeId;
        self
    }
    pub fn num_nodes(&mut self, num_nodes: usize) -> &mut Self {
        self.inner.0.numNodes = num_nodes as seL4_Word;
        self
    }
    pub fn num_io_pt_levels(&mut self, num_io_pt_levels: usize) -> &mut Self {
        self.inner.0.numIOPTLevels = num_io_pt_levels as seL4_Word;
        self
    }
    pub fn ipc_buffer(&mut self, ipc_buffer: NonNull<seL4_IPCBuffer>) -> &mut Self {
        self.inner.0.ipcBuffer = ipc_buffer.as_ptr();
        self
    }
    pub fn empty(&mut self, empty: Range<usize>) -> &mut Self {
        self.inner.0.empty = range_to_slot_region(empty);
        self
    }
    pub fn shared_frames(&mut self, shared_frames: Range<usize>) -> &mut Self {
        self.inner.0.sharedFrames = range_to_slot_region(shared_frames);
        self
    }
    pub fn user_image_frames(&mut self, user_image_frames: Range<usize>) -> &mut Self {
        self.inner.0.userImageFrames = range_to_slot_region(user_image_frames);
        self
    }
    pub fn user_image_paging(&mut self, user_image_paging: Range<usize>) -> &mut Self {
        self.inner.0.userImagePaging = range_to_slot_region(user_image_paging);
        self
    }
    pub fn io_space_caps(&mut self, io_space_caps: Range<usize>) -> &mut Self {
        self.inner.0.ioSpaceCaps = range_to_slot_region(io_space_caps);
        self
    }
    pub fn extra_bi_pages(&mut self, extra_bi_pages: Range<usize>) -> &mut Self {
        self.inner.0.extraBIPages = range_to_slot_region(extra_bi_pages);
        self
    }
    pub fn init_thread_cnode_size_bits(&mut self, init_thread_cnode_size_bits: usize) -> &mut Self {
        self.inner.0.initThreadCNodeSizeBits = init_thread_cnode_size_bits as seL4_Word;
        self
    }
    pub fn init_thread_domain(&mut self, init_thread_domain: usize) -> &mut Self {
        self.inner.0.initThreadDomain = init_thread_domain as seL4_Domain;
        self
    }
    pub fn untyped(&mut self, untyped: Range<usize>) -> &mut Self {
        self.inner.0.untyped = range_to_slot_region(untyped);
        self
    }
    pub fn untyped_list(&mut self, untyped_list: &[seL4_UntypedDesc]) -> &mut Self {
        for (i, desc) in untyped_list.iter().enumerate() {
            self.inner.0.untypedList[i] = desc.clone();
        }
        self
    }
    pub fn build(self) -> seL4_BootInfo {
        let ret = *self.inner;
        ret.0
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