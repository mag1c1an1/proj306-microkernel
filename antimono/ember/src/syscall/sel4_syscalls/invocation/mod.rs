use aster_frame::cpu::UserContext;

use crate::{current_thread, EmberResult};
use crate::common::raw::seL4_MessageInfo;
use crate::common::SeL4Bitfield;
use crate::sel4::SeL4ABI;
use crate::sel4::sys::invocation_label::CNodeCopy;

pub fn invoke_cnode_copy() {}


mod decode;

pub fn handle_invocation(is_call: bool, is_blocking: bool, user_context: &mut UserContext) -> EmberResult<()> {
    let w = user_context.msg_info_reg() as u64;
    let info = seL4_MessageInfo(SeL4Bitfield::new([w; 1]));
    let label = info.get_label();
    error!("{}",label==CNodeCopy.into());
    let len = info.get_length();
    let thread = current_thread!();

    Ok(())
}
