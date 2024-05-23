#[derive(Default)]
pub struct seL4_X86_Page_GetAddress_ret {
    pub error: seL4_Error::Type,
    pub paddr: seL4_Word,
}
#[derive(Default)]
pub struct seL4_X86_IOPort_In8_ret {
    pub error: seL4_Error::Type,
    pub result: seL4_Uint8,
}
#[derive(Default)]
pub struct seL4_X86_IOPort_In16_ret {
    pub error: seL4_Error::Type,
    pub result: seL4_Uint16,
}
#[derive(Default)]
pub struct seL4_X86_IOPort_In32_ret {
    pub error: seL4_Error::Type,
    pub result: seL4_Uint32,
}
impl seL4_IPCBuffer {
    #[allow(clippy::field_reassign_with_default)]
    pub fn seL4_Untyped_Retype(
        &mut self,
        service: seL4_Untyped,
        r#type: seL4_Word,
        r#size_bits: seL4_Word,
        r#root: seL4_CNode,
        r#node_index: seL4_Word,
        r#node_depth: seL4_Word,
        r#node_offset: seL4_Word,
        r#num_objects: seL4_Word,
    ) -> seL4_Error::Type {
        log :: trace ! ("seL4_Untyped_Retype(_service={:?}, type={:?}, size_bits={:?}, root={:?}, node_index={:?}, node_depth={:?}, node_offset={:?}, num_objects={:?})" , service , r#type , r#size_bits , r#root , r#node_index , r#node_depth , r#node_offset , r#num_objects ,);
        self.msg[..6usize].fill(0);
        self.set_mr_bits(0usize..64usize, r#type);
        self.set_mr_bits(64usize..128usize, r#size_bits);
        self.set_cap(0usize, r#root);
        self.set_mr_bits(128usize..192usize, r#node_index);
        self.set_mr_bits(192usize..256usize, r#node_depth);
        self.set_mr_bits(256usize..320usize, r#node_offset);
        self.set_mr_bits(320usize..384usize, r#num_objects);
        let info_in = seL4_MessageInfo::new(invocation_label::UntypedRetype.into(), 0, 1u64, 6u64);
        let info_out = self.seL4_Call(service, info_in);
        let err: seL4_Error::Type = info_out.get_label().try_into().unwrap();
        err
    }
    #[allow(clippy::field_reassign_with_default)]
    pub fn seL4_TCB_ReadRegisters(
        &mut self,
        service: seL4_TCB,
        r#suspend_source: seL4_Bool,
        r#arch_flags: seL4_Uint8,
        r#count: seL4_Word,
        r#regs: &mut seL4_UserContext,
    ) -> seL4_Error::Type {
        log :: trace ! ("seL4_TCB_ReadRegisters(_service={:?}, suspend_source={:?}, arch_flags={:?}, count={:?})" , service , r#suspend_source , r#arch_flags , r#count ,);
        self.msg[..2usize].fill(0);
        self.set_mr_bits(0usize..1usize, r#suspend_source);
        self.set_mr_bits(8usize..16usize, r#arch_flags);
        self.set_mr_bits(64usize..128usize, r#count);
        let info_in =
            seL4_MessageInfo::new(invocation_label::TCBReadRegisters.into(), 0, 0u64, 2u64);
        let info_out = self.seL4_Call(service, info_in);
        let err: seL4_Error::Type = info_out.get_label().try_into().unwrap();
        regs.rip = self.get_mr_bits(0usize..64usize);
        regs.rsp = self.get_mr_bits(64usize..128usize);
        regs.rflags = self.get_mr_bits(128usize..192usize);
        regs.rax = self.get_mr_bits(192usize..256usize);
        regs.rbx = self.get_mr_bits(256usize..320usize);
        regs.rcx = self.get_mr_bits(320usize..384usize);
        regs.rdx = self.get_mr_bits(384usize..448usize);
        regs.rsi = self.get_mr_bits(448usize..512usize);
        regs.rdi = self.get_mr_bits(512usize..576usize);
        regs.rbp = self.get_mr_bits(576usize..640usize);
        regs.r8 = self.get_mr_bits(640usize..704usize);
        regs.r9 = self.get_mr_bits(704usize..768usize);
        regs.r10 = self.get_mr_bits(768usize..832usize);
        regs.r11 = self.get_mr_bits(832usize..896usize);
        regs.r12 = self.get_mr_bits(896usize..960usize);
        regs.r13 = self.get_mr_bits(960usize..1024usize);
        regs.r14 = self.get_mr_bits(1024usize..1088usize);
        regs.r15 = self.get_mr_bits(1088usize..1152usize);
        regs.fs_base = self.get_mr_bits(1152usize..1216usize);
        regs.gs_base = self.get_mr_bits(1216usize..1280usize);
        err
    }
    #[allow(clippy::field_reassign_with_default)]
    pub fn seL4_TCB_WriteRegisters(
        &mut self,
        service: seL4_TCB,
        r#resume_target: seL4_Bool,
        r#arch_flags: seL4_Uint8,
        r#count: seL4_Word,
        r#regs: &seL4_UserContext,
    ) -> seL4_Error::Type {
        log :: trace ! ("seL4_TCB_WriteRegisters(_service={:?}, resume_target={:?}, arch_flags={:?}, count={:?}, regs={:?})" , service , r#resume_target , r#arch_flags , r#count , r#regs ,);
        self.msg[..22usize].fill(0);
        self.set_mr_bits(0usize..1usize, r#resume_target);
        self.set_mr_bits(8usize..16usize, r#arch_flags);
        self.set_mr_bits(64usize..128usize, r#count);
        self.set_mr_bits(128usize..192usize, regs.rip);
        self.set_mr_bits(192usize..256usize, regs.rsp);
        self.set_mr_bits(256usize..320usize, regs.rflags);
        self.set_mr_bits(320usize..384usize, regs.rax);
        self.set_mr_bits(384usize..448usize, regs.rbx);
        self.set_mr_bits(448usize..512usize, regs.rcx);
        self.set_mr_bits(512usize..576usize, regs.rdx);
        self.set_mr_bits(576usize..640usize, regs.rsi);
        self.set_mr_bits(640usize..704usize, regs.rdi);
        self.set_mr_bits(704usize..768usize, regs.rbp);
        self.set_mr_bits(768usize..832usize, regs.r8);
        self.set_mr_bits(832usize..896usize, regs.r9);
        self.set_mr_bits(896usize..960usize, regs.r10);
        self.set_mr_bits(960usize..1024usize, regs.r11);
        self.set_mr_bits(1024usize..1088usize, regs.r12);
        self.set_mr_bits(1088usize..1152usize, regs.r13);
        self.set_mr_bits(1152usize..1216usize, regs.r14);
        self.set_mr_bits(1216usize..1280usize, regs.r15);
        self.set_mr_bits(1280usize..1344usize, regs.fs_base);
        self.set_mr_bits(1344usize..1408usize, regs.gs_base);
        let info_in =
            seL4_MessageInfo::new(invocation_label::TCBWriteRegisters.into(), 0, 0u64, 22u64);
        let info_out = self.seL4_Call(service, info_in);
        let err: seL4_Error::Type = info_out.get_label().try_into().unwrap();
        err
    }
    #[allow(clippy::field_reassign_with_default)]
    pub fn seL4_TCB_CopyRegisters(
        &mut self,
        service: seL4_TCB,
        r#source: seL4_TCB,
        r#suspend_source: seL4_Bool,
        r#resume_target: seL4_Bool,
        r#transfer_frame: seL4_Bool,
        r#transfer_integer: seL4_Bool,
        r#arch_flags: seL4_Uint8,
    ) -> seL4_Error::Type {
        log :: trace ! ("seL4_TCB_CopyRegisters(_service={:?}, source={:?}, suspend_source={:?}, resume_target={:?}, transfer_frame={:?}, transfer_integer={:?}, arch_flags={:?})" , service , r#source , r#suspend_source , r#resume_target , r#transfer_frame , r#transfer_integer , r#arch_flags ,);
        self.msg[..1usize].fill(0);
        self.set_cap(0usize, r#source);
        self.set_mr_bits(0usize..1usize, r#suspend_source);
        self.set_mr_bits(1usize..2usize, r#resume_target);
        self.set_mr_bits(2usize..3usize, r#transfer_frame);
        self.set_mr_bits(3usize..4usize, r#transfer_integer);
        self.set_mr_bits(8usize..16usize, r#arch_flags);
        let info_in =
            seL4_MessageInfo::new(invocation_label::TCBCopyRegisters.into(), 0, 1u64, 1u64);
        let info_out = self.seL4_Call(service, info_in);
        let err: seL4_Error::Type = info_out.get_label().try_into().unwrap();
        err
    }
    #[allow(clippy::field_reassign_with_default)]
    pub fn seL4_TCB_Configure(
        &mut self,
        service: seL4_TCB,
        r#fault_ep: seL4_Word,
        r#cspace_root: seL4_CNode,
        r#cspace_root_data: seL4_Word,
        r#vspace_root: seL4_CPtr,
        r#vspace_root_data: seL4_Word,
        r#buffer: seL4_Word,
        r#bufferFrame: seL4_CPtr,
    ) -> seL4_Error::Type {
        log :: trace ! ("seL4_TCB_Configure(_service={:?}, fault_ep={:?}, cspace_root={:?}, cspace_root_data={:?}, vspace_root={:?}, vspace_root_data={:?}, buffer={:?}, bufferFrame={:?})" , service , r#fault_ep , r#cspace_root , r#cspace_root_data , r#vspace_root , r#vspace_root_data , r#buffer , r#bufferFrame ,);
        self.msg[..4usize].fill(0);
        self.set_mr_bits(0usize..64usize, r#fault_ep);
        self.set_cap(0usize, r#cspace_root);
        self.set_mr_bits(64usize..128usize, r#cspace_root_data);
        self.set_cap(1usize, r#vspace_root);
        self.set_mr_bits(128usize..192usize, r#vspace_root_data);
        self.set_mr_bits(192usize..256usize, r#buffer);
        self.set_cap(2usize, r#bufferFrame);
        let info_in = seL4_MessageInfo::new(invocation_label::TCBConfigure.into(), 0, 3u64, 4u64);
        let info_out = self.seL4_Call(service, info_in);
        let err: seL4_Error::Type = info_out.get_label().try_into().unwrap();
        err
    }
    #[allow(clippy::field_reassign_with_default)]
    pub fn seL4_TCB_SetPriority(
        &mut self,
        service: seL4_TCB,
        r#authority: seL4_TCB,
        r#priority: seL4_Word,
    ) -> seL4_Error::Type {
        log::trace!(
            "seL4_TCB_SetPriority(_service={:?}, authority={:?}, priority={:?})",
            service,
            r#authority,
            r#priority,
        );
        self.msg[..1usize].fill(0);
        self.set_cap(0usize, r#authority);
        self.set_mr_bits(0usize..64usize, r#priority);
        let info_in = seL4_MessageInfo::new(invocation_label::TCBSetPriority.into(), 0, 1u64, 1u64);
        let info_out = self.seL4_Call(service, info_in);
        let err: seL4_Error::Type = info_out.get_label().try_into().unwrap();
        err
    }
    #[allow(clippy::field_reassign_with_default)]
    pub fn seL4_TCB_SetMCPriority(
        &mut self,
        service: seL4_TCB,
        r#authority: seL4_TCB,
        r#mcp: seL4_Word,
    ) -> seL4_Error::Type {
        log::trace!(
            "seL4_TCB_SetMCPriority(_service={:?}, authority={:?}, mcp={:?})",
            service,
            r#authority,
            r#mcp,
        );
        self.msg[..1usize].fill(0);
        self.set_cap(0usize, r#authority);
        self.set_mr_bits(0usize..64usize, r#mcp);
        let info_in =
            seL4_MessageInfo::new(invocation_label::TCBSetMCPriority.into(), 0, 1u64, 1u64);
        let info_out = self.seL4_Call(service, info_in);
        let err: seL4_Error::Type = info_out.get_label().try_into().unwrap();
        err
    }
    #[allow(clippy::field_reassign_with_default)]
    pub fn seL4_TCB_SetSchedParams(
        &mut self,
        service: seL4_TCB,
        r#authority: seL4_TCB,
        r#mcp: seL4_Word,
        r#priority: seL4_Word,
    ) -> seL4_Error::Type {
        log::trace!(
            "seL4_TCB_SetSchedParams(_service={:?}, authority={:?}, mcp={:?}, priority={:?})",
            service,
            r#authority,
            r#mcp,
            r#priority,
        );
        self.msg[..2usize].fill(0);
        self.set_cap(0usize, r#authority);
        self.set_mr_bits(0usize..64usize, r#mcp);
        self.set_mr_bits(64usize..128usize, r#priority);
        let info_in =
            seL4_MessageInfo::new(invocation_label::TCBSetSchedParams.into(), 0, 1u64, 2u64);
        let info_out = self.seL4_Call(service, info_in);
        let err: seL4_Error::Type = info_out.get_label().try_into().unwrap();
        err
    }
    #[allow(clippy::field_reassign_with_default)]
    pub fn seL4_TCB_SetIPCBuffer(
        &mut self,
        service: seL4_TCB,
        r#buffer: seL4_Word,
        r#bufferFrame: seL4_CPtr,
    ) -> seL4_Error::Type {
        log::trace!(
            "seL4_TCB_SetIPCBuffer(_service={:?}, buffer={:?}, bufferFrame={:?})",
            service,
            r#buffer,
            r#bufferFrame,
        );
        self.msg[..1usize].fill(0);
        self.set_mr_bits(0usize..64usize, r#buffer);
        self.set_cap(0usize, r#bufferFrame);
        let info_in =
            seL4_MessageInfo::new(invocation_label::TCBSetIPCBuffer.into(), 0, 1u64, 1u64);
        let info_out = self.seL4_Call(service, info_in);
        let err: seL4_Error::Type = info_out.get_label().try_into().unwrap();
        err
    }
    #[allow(clippy::field_reassign_with_default)]
    pub fn seL4_TCB_SetSpace(
        &mut self,
        service: seL4_TCB,
        r#fault_ep: seL4_Word,
        r#cspace_root: seL4_CNode,
        r#cspace_root_data: seL4_Word,
        r#vspace_root: seL4_CPtr,
        r#vspace_root_data: seL4_Word,
    ) -> seL4_Error::Type {
        log :: trace ! ("seL4_TCB_SetSpace(_service={:?}, fault_ep={:?}, cspace_root={:?}, cspace_root_data={:?}, vspace_root={:?}, vspace_root_data={:?})" , service , r#fault_ep , r#cspace_root , r#cspace_root_data , r#vspace_root , r#vspace_root_data ,);
        self.msg[..3usize].fill(0);
        self.set_mr_bits(0usize..64usize, r#fault_ep);
        self.set_cap(0usize, r#cspace_root);
        self.set_mr_bits(64usize..128usize, r#cspace_root_data);
        self.set_cap(1usize, r#vspace_root);
        self.set_mr_bits(128usize..192usize, r#vspace_root_data);
        let info_in = seL4_MessageInfo::new(invocation_label::TCBSetSpace.into(), 0, 2u64, 3u64);
        let info_out = self.seL4_Call(service, info_in);
        let err: seL4_Error::Type = info_out.get_label().try_into().unwrap();
        err
    }
    #[allow(clippy::field_reassign_with_default)]
    pub fn seL4_TCB_Suspend(&mut self, service: seL4_TCB) -> seL4_Error::Type {
        log::trace!("seL4_TCB_Suspend(_service={:?})", service,);
        self.msg[..0usize].fill(0);
        let info_in = seL4_MessageInfo::new(invocation_label::TCBSuspend.into(), 0, 0u64, 0u64);
        let info_out = self.seL4_Call(service, info_in);
        let err: seL4_Error::Type = info_out.get_label().try_into().unwrap();
        err
    }
    #[allow(clippy::field_reassign_with_default)]
    pub fn seL4_TCB_Resume(&mut self, service: seL4_TCB) -> seL4_Error::Type {
        log::trace!("seL4_TCB_Resume(_service={:?})", service,);
        self.msg[..0usize].fill(0);
        let info_in = seL4_MessageInfo::new(invocation_label::TCBResume.into(), 0, 0u64, 0u64);
        let info_out = self.seL4_Call(service, info_in);
        let err: seL4_Error::Type = info_out.get_label().try_into().unwrap();
        err
    }
    #[allow(clippy::field_reassign_with_default)]
    pub fn seL4_TCB_BindNotification(
        &mut self,
        service: seL4_TCB,
        r#notification: seL4_CPtr,
    ) -> seL4_Error::Type {
        log::trace!(
            "seL4_TCB_BindNotification(_service={:?}, notification={:?})",
            service,
            r#notification,
        );
        self.msg[..0usize].fill(0);
        self.set_cap(0usize, r#notification);
        let info_in =
            seL4_MessageInfo::new(invocation_label::TCBBindNotification.into(), 0, 1u64, 0u64);
        let info_out = self.seL4_Call(service, info_in);
        let err: seL4_Error::Type = info_out.get_label().try_into().unwrap();
        err
    }
    #[allow(clippy::field_reassign_with_default)]
    pub fn seL4_TCB_UnbindNotification(&mut self, service: seL4_TCB) -> seL4_Error::Type {
        log::trace!("seL4_TCB_UnbindNotification(_service={:?})", service,);
        self.msg[..0usize].fill(0);
        let info_in = seL4_MessageInfo::new(
            invocation_label::TCBUnbindNotification.into(),
            0,
            0u64,
            0u64,
        );
        let info_out = self.seL4_Call(service, info_in);
        let err: seL4_Error::Type = info_out.get_label().try_into().unwrap();
        err
    }
    #[allow(clippy::field_reassign_with_default)]
    pub fn seL4_TCB_SetTLSBase(
        &mut self,
        service: seL4_TCB,
        r#tls_base: seL4_Word,
    ) -> seL4_Error::Type {
        log::trace!(
            "seL4_TCB_SetTLSBase(_service={:?}, tls_base={:?})",
            service,
            r#tls_base,
        );
        self.msg[..1usize].fill(0);
        self.set_mr_bits(0usize..64usize, r#tls_base);
        let info_in = seL4_MessageInfo::new(invocation_label::TCBSetTLSBase.into(), 0, 0u64, 1u64);
        let info_out = self.seL4_Call(service, info_in);
        let err: seL4_Error::Type = info_out.get_label().try_into().unwrap();
        err
    }
    #[allow(clippy::field_reassign_with_default)]
    pub fn seL4_CNode_Revoke(
        &mut self,
        service: seL4_CNode,
        r#index: seL4_Word,
        r#depth: seL4_Uint8,
    ) -> seL4_Error::Type {
        log::trace!(
            "seL4_CNode_Revoke(_service={:?}, index={:?}, depth={:?})",
            service,
            r#index,
            r#depth,
        );
        self.msg[..2usize].fill(0);
        self.set_mr_bits(0usize..64usize, r#index);
        self.set_mr_bits(64usize..72usize, r#depth);
        let info_in = seL4_MessageInfo::new(invocation_label::CNodeRevoke.into(), 0, 0u64, 2u64);
        let info_out = self.seL4_Call(service, info_in);
        let err: seL4_Error::Type = info_out.get_label().try_into().unwrap();
        err
    }
    #[allow(clippy::field_reassign_with_default)]
    pub fn seL4_CNode_Delete(
        &mut self,
        service: seL4_CNode,
        r#index: seL4_Word,
        r#depth: seL4_Uint8,
    ) -> seL4_Error::Type {
        log::trace!(
            "seL4_CNode_Delete(_service={:?}, index={:?}, depth={:?})",
            service,
            r#index,
            r#depth,
        );
        self.msg[..2usize].fill(0);
        self.set_mr_bits(0usize..64usize, r#index);
        self.set_mr_bits(64usize..72usize, r#depth);
        let info_in = seL4_MessageInfo::new(invocation_label::CNodeDelete.into(), 0, 0u64, 2u64);
        let info_out = self.seL4_Call(service, info_in);
        let err: seL4_Error::Type = info_out.get_label().try_into().unwrap();
        err
    }
    #[allow(clippy::field_reassign_with_default)]
    pub fn seL4_CNode_CancelBadgedSends(
        &mut self,
        service: seL4_CNode,
        r#index: seL4_Word,
        r#depth: seL4_Uint8,
    ) -> seL4_Error::Type {
        log::trace!(
            "seL4_CNode_CancelBadgedSends(_service={:?}, index={:?}, depth={:?})",
            service,
            r#index,
            r#depth,
        );
        self.msg[..2usize].fill(0);
        self.set_mr_bits(0usize..64usize, r#index);
        self.set_mr_bits(64usize..72usize, r#depth);
        let info_in = seL4_MessageInfo::new(
            invocation_label::CNodeCancelBadgedSends.into(),
            0,
            0u64,
            2u64,
        );
        let info_out = self.seL4_Call(service, info_in);
        let err: seL4_Error::Type = info_out.get_label().try_into().unwrap();
        err
    }
    #[allow(clippy::field_reassign_with_default)]
    pub fn seL4_CNode_Copy(
        &mut self,
        service: seL4_CNode,
        r#dest_index: seL4_Word,
        r#dest_depth: seL4_Uint8,
        r#src_root: seL4_CNode,
        r#src_index: seL4_Word,
        r#src_depth: seL4_Uint8,
        r#rights: seL4_CapRights_t,
    ) -> seL4_Error::Type {
        log :: trace ! ("seL4_CNode_Copy(_service={:?}, dest_index={:?}, dest_depth={:?}, src_root={:?}, src_index={:?}, src_depth={:?}, rights={:?})" , service , r#dest_index , r#dest_depth , r#src_root , r#src_index , r#src_depth , r#rights ,);
        self.msg[..5usize].fill(0);
        self.set_mr_bits(0usize..64usize, r#dest_index);
        self.set_mr_bits(64usize..72usize, r#dest_depth);
        self.set_cap(0usize, r#src_root);
        self.set_mr_bits(128usize..192usize, r#src_index);
        self.set_mr_bits(192usize..200usize, r#src_depth);
        self.set_mr_bits_from_slice(256usize..320usize, r#rights.0.inner());
        let info_in = seL4_MessageInfo::new(invocation_label::CNodeCopy.into(), 0, 1u64, 5u64);
        let info_out = self.seL4_Call(service, info_in);
        let err: seL4_Error::Type = info_out.get_label().try_into().unwrap();
        err
    }
    #[allow(clippy::field_reassign_with_default)]
    pub fn seL4_CNode_Mint(
        &mut self,
        service: seL4_CNode,
        r#dest_index: seL4_Word,
        r#dest_depth: seL4_Uint8,
        r#src_root: seL4_CNode,
        r#src_index: seL4_Word,
        r#src_depth: seL4_Uint8,
        r#rights: seL4_CapRights_t,
        r#badge: seL4_Word,
    ) -> seL4_Error::Type {
        log :: trace ! ("seL4_CNode_Mint(_service={:?}, dest_index={:?}, dest_depth={:?}, src_root={:?}, src_index={:?}, src_depth={:?}, rights={:?}, badge={:?})" , service , r#dest_index , r#dest_depth , r#src_root , r#src_index , r#src_depth , r#rights , r#badge ,);
        self.msg[..6usize].fill(0);
        self.set_mr_bits(0usize..64usize, r#dest_index);
        self.set_mr_bits(64usize..72usize, r#dest_depth);
        self.set_cap(0usize, r#src_root);
        self.set_mr_bits(128usize..192usize, r#src_index);
        self.set_mr_bits(192usize..200usize, r#src_depth);
        self.set_mr_bits_from_slice(256usize..320usize, r#rights.0.inner());
        self.set_mr_bits(320usize..384usize, r#badge);
        let info_in = seL4_MessageInfo::new(invocation_label::CNodeMint.into(), 0, 1u64, 6u64);
        let info_out = self.seL4_Call(service, info_in);
        let err: seL4_Error::Type = info_out.get_label().try_into().unwrap();
        err
    }
    #[allow(clippy::field_reassign_with_default)]
    pub fn seL4_CNode_Move(
        &mut self,
        service: seL4_CNode,
        r#dest_index: seL4_Word,
        r#dest_depth: seL4_Uint8,
        r#src_root: seL4_CNode,
        r#src_index: seL4_Word,
        r#src_depth: seL4_Uint8,
    ) -> seL4_Error::Type {
        log :: trace ! ("seL4_CNode_Move(_service={:?}, dest_index={:?}, dest_depth={:?}, src_root={:?}, src_index={:?}, src_depth={:?})" , service , r#dest_index , r#dest_depth , r#src_root , r#src_index , r#src_depth ,);
        self.msg[..4usize].fill(0);
        self.set_mr_bits(0usize..64usize, r#dest_index);
        self.set_mr_bits(64usize..72usize, r#dest_depth);
        self.set_cap(0usize, r#src_root);
        self.set_mr_bits(128usize..192usize, r#src_index);
        self.set_mr_bits(192usize..200usize, r#src_depth);
        let info_in = seL4_MessageInfo::new(invocation_label::CNodeMove.into(), 0, 1u64, 4u64);
        let info_out = self.seL4_Call(service, info_in);
        let err: seL4_Error::Type = info_out.get_label().try_into().unwrap();
        err
    }
    #[allow(clippy::field_reassign_with_default)]
    pub fn seL4_CNode_Mutate(
        &mut self,
        service: seL4_CNode,
        r#dest_index: seL4_Word,
        r#dest_depth: seL4_Uint8,
        r#src_root: seL4_CNode,
        r#src_index: seL4_Word,
        r#src_depth: seL4_Uint8,
        r#badge: seL4_Word,
    ) -> seL4_Error::Type {
        log :: trace ! ("seL4_CNode_Mutate(_service={:?}, dest_index={:?}, dest_depth={:?}, src_root={:?}, src_index={:?}, src_depth={:?}, badge={:?})" , service , r#dest_index , r#dest_depth , r#src_root , r#src_index , r#src_depth , r#badge ,);
        self.msg[..5usize].fill(0);
        self.set_mr_bits(0usize..64usize, r#dest_index);
        self.set_mr_bits(64usize..72usize, r#dest_depth);
        self.set_cap(0usize, r#src_root);
        self.set_mr_bits(128usize..192usize, r#src_index);
        self.set_mr_bits(192usize..200usize, r#src_depth);
        self.set_mr_bits(256usize..320usize, r#badge);
        let info_in = seL4_MessageInfo::new(invocation_label::CNodeMutate.into(), 0, 1u64, 5u64);
        let info_out = self.seL4_Call(service, info_in);
        let err: seL4_Error::Type = info_out.get_label().try_into().unwrap();
        err
    }
    #[allow(clippy::field_reassign_with_default)]
    pub fn seL4_CNode_Rotate(
        &mut self,
        service: seL4_CNode,
        r#dest_index: seL4_Word,
        r#dest_depth: seL4_Uint8,
        r#dest_badge: seL4_Word,
        r#pivot_root: seL4_CNode,
        r#pivot_index: seL4_Word,
        r#pivot_depth: seL4_Uint8,
        r#pivot_badge: seL4_Word,
        r#src_root: seL4_CNode,
        r#src_index: seL4_Word,
        r#src_depth: seL4_Uint8,
    ) -> seL4_Error::Type {
        log :: trace ! ("seL4_CNode_Rotate(_service={:?}, dest_index={:?}, dest_depth={:?}, dest_badge={:?}, pivot_root={:?}, pivot_index={:?}, pivot_depth={:?}, pivot_badge={:?}, src_root={:?}, src_index={:?}, src_depth={:?})" , service , r#dest_index , r#dest_depth , r#dest_badge , r#pivot_root , r#pivot_index , r#pivot_depth , r#pivot_badge , r#src_root , r#src_index , r#src_depth ,);
        self.msg[..8usize].fill(0);
        self.set_mr_bits(0usize..64usize, r#dest_index);
        self.set_mr_bits(64usize..72usize, r#dest_depth);
        self.set_mr_bits(128usize..192usize, r#dest_badge);
        self.set_cap(0usize, r#pivot_root);
        self.set_mr_bits(192usize..256usize, r#pivot_index);
        self.set_mr_bits(256usize..264usize, r#pivot_depth);
        self.set_mr_bits(320usize..384usize, r#pivot_badge);
        self.set_cap(1usize, r#src_root);
        self.set_mr_bits(384usize..448usize, r#src_index);
        self.set_mr_bits(448usize..456usize, r#src_depth);
        let info_in = seL4_MessageInfo::new(invocation_label::CNodeRotate.into(), 0, 2u64, 8u64);
        let info_out = self.seL4_Call(service, info_in);
        let err: seL4_Error::Type = info_out.get_label().try_into().unwrap();
        err
    }
    #[allow(clippy::field_reassign_with_default)]
    pub fn seL4_CNode_SaveCaller(
        &mut self,
        service: seL4_CNode,
        r#index: seL4_Word,
        r#depth: seL4_Uint8,
    ) -> seL4_Error::Type {
        log::trace!(
            "seL4_CNode_SaveCaller(_service={:?}, index={:?}, depth={:?})",
            service,
            r#index,
            r#depth,
        );
        self.msg[..2usize].fill(0);
        self.set_mr_bits(0usize..64usize, r#index);
        self.set_mr_bits(64usize..72usize, r#depth);
        let info_in =
            seL4_MessageInfo::new(invocation_label::CNodeSaveCaller.into(), 0, 0u64, 2u64);
        let info_out = self.seL4_Call(service, info_in);
        let err: seL4_Error::Type = info_out.get_label().try_into().unwrap();
        err
    }
    #[allow(clippy::field_reassign_with_default)]
    pub fn seL4_IRQControl_Get(
        &mut self,
        service: seL4_IRQControl,
        r#irq: seL4_Word,
        r#root: seL4_CNode,
        r#index: seL4_Word,
        r#depth: seL4_Uint8,
    ) -> seL4_Error::Type {
        log::trace!(
            "seL4_IRQControl_Get(_service={:?}, irq={:?}, root={:?}, index={:?}, depth={:?})",
            service,
            r#irq,
            r#root,
            r#index,
            r#depth,
        );
        self.msg[..3usize].fill(0);
        self.set_mr_bits(0usize..64usize, r#irq);
        self.set_cap(0usize, r#root);
        self.set_mr_bits(64usize..128usize, r#index);
        self.set_mr_bits(128usize..136usize, r#depth);
        let info_in =
            seL4_MessageInfo::new(invocation_label::IRQIssueIRQHandler.into(), 0, 1u64, 3u64);
        let info_out = self.seL4_Call(service, info_in);
        let err: seL4_Error::Type = info_out.get_label().try_into().unwrap();
        err
    }
    #[allow(clippy::field_reassign_with_default)]
    pub fn seL4_IRQHandler_Ack(&mut self, service: seL4_IRQHandler) -> seL4_Error::Type {
        log::trace!("seL4_IRQHandler_Ack(_service={:?})", service,);
        self.msg[..0usize].fill(0);
        let info_in = seL4_MessageInfo::new(invocation_label::IRQAckIRQ.into(), 0, 0u64, 0u64);
        let info_out = self.seL4_Call(service, info_in);
        let err: seL4_Error::Type = info_out.get_label().try_into().unwrap();
        err
    }
    #[allow(clippy::field_reassign_with_default)]
    pub fn seL4_IRQHandler_SetNotification(
        &mut self,
        service: seL4_IRQHandler,
        r#notification: seL4_CPtr,
    ) -> seL4_Error::Type {
        log::trace!(
            "seL4_IRQHandler_SetNotification(_service={:?}, notification={:?})",
            service,
            r#notification,
        );
        self.msg[..0usize].fill(0);
        self.set_cap(0usize, r#notification);
        let info_in =
            seL4_MessageInfo::new(invocation_label::IRQSetIRQHandler.into(), 0, 1u64, 0u64);
        let info_out = self.seL4_Call(service, info_in);
        let err: seL4_Error::Type = info_out.get_label().try_into().unwrap();
        err
    }
    #[allow(clippy::field_reassign_with_default)]
    pub fn seL4_IRQHandler_Clear(&mut self, service: seL4_IRQHandler) -> seL4_Error::Type {
        log::trace!("seL4_IRQHandler_Clear(_service={:?})", service,);
        self.msg[..0usize].fill(0);
        let info_in =
            seL4_MessageInfo::new(invocation_label::IRQClearIRQHandler.into(), 0, 0u64, 0u64);
        let info_out = self.seL4_Call(service, info_in);
        let err: seL4_Error::Type = info_out.get_label().try_into().unwrap();
        err
    }
    #[allow(clippy::field_reassign_with_default)]
    pub fn seL4_DomainSet_Set(
        &mut self,
        service: seL4_DomainSet,
        r#domain: seL4_Uint8,
        r#thread: seL4_TCB,
    ) -> seL4_Error::Type {
        log::trace!(
            "seL4_DomainSet_Set(_service={:?}, domain={:?}, thread={:?})",
            service,
            r#domain,
            r#thread,
        );
        self.msg[..1usize].fill(0);
        self.set_mr_bits(0usize..8usize, r#domain);
        self.set_cap(0usize, r#thread);
        let info_in = seL4_MessageInfo::new(invocation_label::DomainSetSet.into(), 0, 1u64, 1u64);
        let info_out = self.seL4_Call(service, info_in);
        let err: seL4_Error::Type = info_out.get_label().try_into().unwrap();
        err
    }
    #[allow(clippy::field_reassign_with_default)]
    pub fn seL4_X86_PDPT_Map(
        &mut self,
        service: seL4_X86_PDPT,
        r#pml4: seL4_X64_PML4,
        r#vaddr: seL4_Word,
        r#attr: seL4_X86_VMAttributes::Type,
    ) -> seL4_Error::Type {
        log::trace!(
            "seL4_X86_PDPT_Map(_service={:?}, pml4={:?}, vaddr={:?}, attr={:?})",
            service,
            r#pml4,
            r#vaddr,
            r#attr,
        );
        self.msg[..2usize].fill(0);
        self.set_cap(0usize, r#pml4);
        self.set_mr_bits(0usize..64usize, r#vaddr);
        self.set_mr_bits(64usize..128usize, r#attr);
        let info_in = seL4_MessageInfo::new(invocation_label::X86PDPTMap.into(), 0, 1u64, 2u64);
        let info_out = self.seL4_Call(service, info_in);
        let err: seL4_Error::Type = info_out.get_label().try_into().unwrap();
        err
    }
    #[allow(clippy::field_reassign_with_default)]
    pub fn seL4_X86_PDPT_Unmap(&mut self, service: seL4_X86_PDPT) -> seL4_Error::Type {
        log::trace!("seL4_X86_PDPT_Unmap(_service={:?})", service,);
        self.msg[..0usize].fill(0);
        let info_in = seL4_MessageInfo::new(invocation_label::X86PDPTUnmap.into(), 0, 0u64, 0u64);
        let info_out = self.seL4_Call(service, info_in);
        let err: seL4_Error::Type = info_out.get_label().try_into().unwrap();
        err
    }
    #[allow(clippy::field_reassign_with_default)]
    pub fn seL4_X86_PageDirectory_Map(
        &mut self,
        service: seL4_X86_PageDirectory,
        r#vspace: seL4_CPtr,
        r#vaddr: seL4_Word,
        r#attr: seL4_X86_VMAttributes::Type,
    ) -> seL4_Error::Type {
        log::trace!(
            "seL4_X86_PageDirectory_Map(_service={:?}, vspace={:?}, vaddr={:?}, attr={:?})",
            service,
            r#vspace,
            r#vaddr,
            r#attr,
        );
        self.msg[..2usize].fill(0);
        self.set_cap(0usize, r#vspace);
        self.set_mr_bits(0usize..64usize, r#vaddr);
        self.set_mr_bits(64usize..128usize, r#attr);
        let info_in =
            seL4_MessageInfo::new(invocation_label::X86PageDirectoryMap.into(), 0, 1u64, 2u64);
        let info_out = self.seL4_Call(service, info_in);
        let err: seL4_Error::Type = info_out.get_label().try_into().unwrap();
        err
    }
    #[allow(clippy::field_reassign_with_default)]
    pub fn seL4_X86_PageDirectory_Unmap(
        &mut self,
        service: seL4_X86_PageDirectory,
    ) -> seL4_Error::Type {
        log::trace!("seL4_X86_PageDirectory_Unmap(_service={:?})", service,);
        self.msg[..0usize].fill(0);
        let info_in = seL4_MessageInfo::new(
            invocation_label::X86PageDirectoryUnmap.into(),
            0,
            0u64,
            0u64,
        );
        let info_out = self.seL4_Call(service, info_in);
        let err: seL4_Error::Type = info_out.get_label().try_into().unwrap();
        err
    }
    #[allow(clippy::field_reassign_with_default)]
    pub fn seL4_X86_PageTable_Map(
        &mut self,
        service: seL4_X86_PageTable,
        r#vspace: seL4_CPtr,
        r#vaddr: seL4_Word,
        r#attr: seL4_X86_VMAttributes::Type,
    ) -> seL4_Error::Type {
        log::trace!(
            "seL4_X86_PageTable_Map(_service={:?}, vspace={:?}, vaddr={:?}, attr={:?})",
            service,
            r#vspace,
            r#vaddr,
            r#attr,
        );
        self.msg[..2usize].fill(0);
        self.set_cap(0usize, r#vspace);
        self.set_mr_bits(0usize..64usize, r#vaddr);
        self.set_mr_bits(64usize..128usize, r#attr);
        let info_in =
            seL4_MessageInfo::new(invocation_label::X86PageTableMap.into(), 0, 1u64, 2u64);
        let info_out = self.seL4_Call(service, info_in);
        let err: seL4_Error::Type = info_out.get_label().try_into().unwrap();
        err
    }
    #[allow(clippy::field_reassign_with_default)]
    pub fn seL4_X86_PageTable_Unmap(&mut self, service: seL4_X86_PageTable) -> seL4_Error::Type {
        log::trace!("seL4_X86_PageTable_Unmap(_service={:?})", service,);
        self.msg[..0usize].fill(0);
        let info_in =
            seL4_MessageInfo::new(invocation_label::X86PageTableUnmap.into(), 0, 0u64, 0u64);
        let info_out = self.seL4_Call(service, info_in);
        let err: seL4_Error::Type = info_out.get_label().try_into().unwrap();
        err
    }
    #[allow(clippy::field_reassign_with_default)]
    pub fn seL4_X86_Page_Map(
        &mut self,
        service: seL4_X86_Page,
        r#vspace: seL4_CPtr,
        r#vaddr: seL4_Word,
        r#rights: seL4_CapRights_t,
        r#attr: seL4_X86_VMAttributes::Type,
    ) -> seL4_Error::Type {
        log::trace!(
            "seL4_X86_Page_Map(_service={:?}, vspace={:?}, vaddr={:?}, rights={:?}, attr={:?})",
            service,
            r#vspace,
            r#vaddr,
            r#rights,
            r#attr,
        );
        self.msg[..3usize].fill(0);
        self.set_cap(0usize, r#vspace);
        self.set_mr_bits(0usize..64usize, r#vaddr);
        self.set_mr_bits_from_slice(64usize..128usize, r#rights.0.inner());
        self.set_mr_bits(128usize..192usize, r#attr);
        let info_in = seL4_MessageInfo::new(invocation_label::X86PageMap.into(), 0, 1u64, 3u64);
        let info_out = self.seL4_Call(service, info_in);
        let err: seL4_Error::Type = info_out.get_label().try_into().unwrap();
        err
    }
    #[allow(clippy::field_reassign_with_default)]
    pub fn seL4_X86_Page_Unmap(&mut self, service: seL4_X86_Page) -> seL4_Error::Type {
        log::trace!("seL4_X86_Page_Unmap(_service={:?})", service,);
        self.msg[..0usize].fill(0);
        let info_in = seL4_MessageInfo::new(invocation_label::X86PageUnmap.into(), 0, 0u64, 0u64);
        let info_out = self.seL4_Call(service, info_in);
        let err: seL4_Error::Type = info_out.get_label().try_into().unwrap();
        err
    }
    #[allow(clippy::field_reassign_with_default)]
    pub fn seL4_X86_Page_GetAddress(
        &mut self,
        service: seL4_X86_Page,
    ) -> seL4_X86_Page_GetAddress_ret {
        log::trace!("seL4_X86_Page_GetAddress(_service={:?})", service,);
        self.msg[..0usize].fill(0);
        let info_in =
            seL4_MessageInfo::new(invocation_label::X86PageGetAddress.into(), 0, 0u64, 0u64);
        let info_out = self.seL4_Call(service, info_in);
        let err: seL4_Error::Type = info_out.get_label().try_into().unwrap();
        let mut ret: seL4_X86_Page_GetAddress_ret = Default::default();
        ret.error = err;
        ret.paddr = self.get_mr_bits(0usize..64usize);
        ret
    }
    #[allow(clippy::field_reassign_with_default)]
    pub fn seL4_X86_ASIDControl_MakePool(
        &mut self,
        service: seL4_X86_ASIDControl,
        r#untyped: seL4_Untyped,
        r#root: seL4_CNode,
        r#index: seL4_Word,
        r#depth: seL4_Uint8,
    ) -> seL4_Error::Type {
        log :: trace ! ("seL4_X86_ASIDControl_MakePool(_service={:?}, untyped={:?}, root={:?}, index={:?}, depth={:?})" , service , r#untyped , r#root , r#index , r#depth ,);
        self.msg[..2usize].fill(0);
        self.set_cap(0usize, r#untyped);
        self.set_cap(1usize, r#root);
        self.set_mr_bits(0usize..64usize, r#index);
        self.set_mr_bits(64usize..72usize, r#depth);
        let info_in = seL4_MessageInfo::new(
            invocation_label::X86ASIDControlMakePool.into(),
            0,
            2u64,
            2u64,
        );
        let info_out = self.seL4_Call(service, info_in);
        let err: seL4_Error::Type = info_out.get_label().try_into().unwrap();
        err
    }
    #[allow(clippy::field_reassign_with_default)]
    pub fn seL4_X86_ASIDPool_Assign(
        &mut self,
        service: seL4_X86_ASIDPool,
        r#vspace: seL4_CPtr,
    ) -> seL4_Error::Type {
        log::trace!(
            "seL4_X86_ASIDPool_Assign(_service={:?}, vspace={:?})",
            service,
            r#vspace,
        );
        self.msg[..0usize].fill(0);
        self.set_cap(0usize, r#vspace);
        let info_in =
            seL4_MessageInfo::new(invocation_label::X86ASIDPoolAssign.into(), 0, 1u64, 0u64);
        let info_out = self.seL4_Call(service, info_in);
        let err: seL4_Error::Type = info_out.get_label().try_into().unwrap();
        err
    }
    #[allow(clippy::field_reassign_with_default)]
    pub fn seL4_X86_IOPortControl_Issue(
        &mut self,
        service: seL4_X86_IOPortControl,
        r#first_port: seL4_Word,
        r#last_port: seL4_Word,
        r#root: seL4_CNode,
        r#index: seL4_Word,
        r#depth: seL4_Uint8,
    ) -> seL4_Error::Type {
        log :: trace ! ("seL4_X86_IOPortControl_Issue(_service={:?}, first_port={:?}, last_port={:?}, root={:?}, index={:?}, depth={:?})" , service , r#first_port , r#last_port , r#root , r#index , r#depth ,);
        self.msg[..4usize].fill(0);
        self.set_mr_bits(0usize..64usize, r#first_port);
        self.set_mr_bits(64usize..128usize, r#last_port);
        self.set_cap(0usize, r#root);
        self.set_mr_bits(128usize..192usize, r#index);
        self.set_mr_bits(192usize..200usize, r#depth);
        let info_in = seL4_MessageInfo::new(
            invocation_label::X86IOPortControlIssue.into(),
            0,
            1u64,
            4u64,
        );
        let info_out = self.seL4_Call(service, info_in);
        let err: seL4_Error::Type = info_out.get_label().try_into().unwrap();
        err
    }
    #[allow(clippy::field_reassign_with_default)]
    pub fn seL4_X86_IOPort_In8(
        &mut self,
        service: seL4_X86_IOPort,
        r#port: seL4_Uint16,
    ) -> seL4_X86_IOPort_In8_ret {
        log::trace!(
            "seL4_X86_IOPort_In8(_service={:?}, port={:?})",
            service,
            r#port,
        );
        self.msg[..1usize].fill(0);
        self.set_mr_bits(0usize..16usize, r#port);
        let info_in = seL4_MessageInfo::new(invocation_label::X86IOPortIn8.into(), 0, 0u64, 1u64);
        let info_out = self.seL4_Call(service, info_in);
        let err: seL4_Error::Type = info_out.get_label().try_into().unwrap();
        let mut ret: seL4_X86_IOPort_In8_ret = Default::default();
        ret.error = err;
        ret.result = self.get_mr_bits(0usize..8usize);
        ret
    }
    #[allow(clippy::field_reassign_with_default)]
    pub fn seL4_X86_IOPort_In16(
        &mut self,
        service: seL4_X86_IOPort,
        r#port: seL4_Uint16,
    ) -> seL4_X86_IOPort_In16_ret {
        log::trace!(
            "seL4_X86_IOPort_In16(_service={:?}, port={:?})",
            service,
            r#port,
        );
        self.msg[..1usize].fill(0);
        self.set_mr_bits(0usize..16usize, r#port);
        let info_in = seL4_MessageInfo::new(invocation_label::X86IOPortIn16.into(), 0, 0u64, 1u64);
        let info_out = self.seL4_Call(service, info_in);
        let err: seL4_Error::Type = info_out.get_label().try_into().unwrap();
        let mut ret: seL4_X86_IOPort_In16_ret = Default::default();
        ret.error = err;
        ret.result = self.get_mr_bits(0usize..16usize);
        ret
    }
    #[allow(clippy::field_reassign_with_default)]
    pub fn seL4_X86_IOPort_In32(
        &mut self,
        service: seL4_X86_IOPort,
        r#port: seL4_Uint16,
    ) -> seL4_X86_IOPort_In32_ret {
        log::trace!(
            "seL4_X86_IOPort_In32(_service={:?}, port={:?})",
            service,
            r#port,
        );
        self.msg[..1usize].fill(0);
        self.set_mr_bits(0usize..16usize, r#port);
        let info_in = seL4_MessageInfo::new(invocation_label::X86IOPortIn32.into(), 0, 0u64, 1u64);
        let info_out = self.seL4_Call(service, info_in);
        let err: seL4_Error::Type = info_out.get_label().try_into().unwrap();
        let mut ret: seL4_X86_IOPort_In32_ret = Default::default();
        ret.error = err;
        ret.result = self.get_mr_bits(0usize..32usize);
        ret
    }
    #[allow(clippy::field_reassign_with_default)]
    pub fn seL4_X86_IOPort_Out8(
        &mut self,
        service: seL4_X86_IOPort,
        r#port: seL4_Word,
        r#data: seL4_Word,
    ) -> seL4_Error::Type {
        log::trace!(
            "seL4_X86_IOPort_Out8(_service={:?}, port={:?}, data={:?})",
            service,
            r#port,
            r#data,
        );
        self.msg[..2usize].fill(0);
        self.set_mr_bits(0usize..64usize, r#port);
        self.set_mr_bits(64usize..128usize, r#data);
        let info_in = seL4_MessageInfo::new(invocation_label::X86IOPortOut8.into(), 0, 0u64, 2u64);
        let info_out = self.seL4_Call(service, info_in);
        let err: seL4_Error::Type = info_out.get_label().try_into().unwrap();
        err
    }
    #[allow(clippy::field_reassign_with_default)]
    pub fn seL4_X86_IOPort_Out16(
        &mut self,
        service: seL4_X86_IOPort,
        r#port: seL4_Word,
        r#data: seL4_Word,
    ) -> seL4_Error::Type {
        log::trace!(
            "seL4_X86_IOPort_Out16(_service={:?}, port={:?}, data={:?})",
            service,
            r#port,
            r#data,
        );
        self.msg[..2usize].fill(0);
        self.set_mr_bits(0usize..64usize, r#port);
        self.set_mr_bits(64usize..128usize, r#data);
        let info_in = seL4_MessageInfo::new(invocation_label::X86IOPortOut16.into(), 0, 0u64, 2u64);
        let info_out = self.seL4_Call(service, info_in);
        let err: seL4_Error::Type = info_out.get_label().try_into().unwrap();
        err
    }
    #[allow(clippy::field_reassign_with_default)]
    pub fn seL4_X86_IOPort_Out32(
        &mut self,
        service: seL4_X86_IOPort,
        r#port: seL4_Word,
        r#data: seL4_Word,
    ) -> seL4_Error::Type {
        log::trace!(
            "seL4_X86_IOPort_Out32(_service={:?}, port={:?}, data={:?})",
            service,
            r#port,
            r#data,
        );
        self.msg[..2usize].fill(0);
        self.set_mr_bits(0usize..64usize, r#port);
        self.set_mr_bits(64usize..128usize, r#data);
        let info_in = seL4_MessageInfo::new(invocation_label::X86IOPortOut32.into(), 0, 0u64, 2u64);
        let info_out = self.seL4_Call(service, info_in);
        let err: seL4_Error::Type = info_out.get_label().try_into().unwrap();
        err
    }
    #[allow(clippy::field_reassign_with_default)]
    pub fn seL4_IRQControl_GetIOAPIC(
        &mut self,
        service: seL4_IRQControl,
        r#root: seL4_CNode,
        r#index: seL4_Word,
        r#depth: seL4_Uint8,
        r#ioapic: seL4_Word,
        r#pin: seL4_Word,
        r#level: seL4_Word,
        r#polarity: seL4_Word,
        r#vector: seL4_Word,
    ) -> seL4_Error::Type {
        log :: trace ! ("seL4_IRQControl_GetIOAPIC(_service={:?}, root={:?}, index={:?}, depth={:?}, ioapic={:?}, pin={:?}, level={:?}, polarity={:?}, vector={:?})" , service , r#root , r#index , r#depth , r#ioapic , r#pin , r#level , r#polarity , r#vector ,);
        self.msg[..7usize].fill(0);
        self.set_cap(0usize, r#root);
        self.set_mr_bits(0usize..64usize, r#index);
        self.set_mr_bits(64usize..72usize, r#depth);
        self.set_mr_bits(128usize..192usize, r#ioapic);
        self.set_mr_bits(192usize..256usize, r#pin);
        self.set_mr_bits(256usize..320usize, r#level);
        self.set_mr_bits(320usize..384usize, r#polarity);
        self.set_mr_bits(384usize..448usize, r#vector);
        let info_in = seL4_MessageInfo::new(
            invocation_label::X86IRQIssueIRQHandlerIOAPIC.into(),
            0,
            1u64,
            7u64,
        );
        let info_out = self.seL4_Call(service, info_in);
        let err: seL4_Error::Type = info_out.get_label().try_into().unwrap();
        err
    }
    #[allow(clippy::field_reassign_with_default)]
    pub fn seL4_IRQControl_GetMSI(
        &mut self,
        service: seL4_IRQControl,
        r#root: seL4_CNode,
        r#index: seL4_Word,
        r#depth: seL4_Uint8,
        r#pci_bus: seL4_Word,
        r#pci_dev: seL4_Word,
        r#pci_func: seL4_Word,
        r#handle: seL4_Word,
        r#vector: seL4_Word,
    ) -> seL4_Error::Type {
        log :: trace ! ("seL4_IRQControl_GetMSI(_service={:?}, root={:?}, index={:?}, depth={:?}, pci_bus={:?}, pci_dev={:?}, pci_func={:?}, handle={:?}, vector={:?})" , service , r#root , r#index , r#depth , r#pci_bus , r#pci_dev , r#pci_func , r#handle , r#vector ,);
        self.msg[..7usize].fill(0);
        self.set_cap(0usize, r#root);
        self.set_mr_bits(0usize..64usize, r#index);
        self.set_mr_bits(64usize..72usize, r#depth);
        self.set_mr_bits(128usize..192usize, r#pci_bus);
        self.set_mr_bits(192usize..256usize, r#pci_dev);
        self.set_mr_bits(256usize..320usize, r#pci_func);
        self.set_mr_bits(320usize..384usize, r#handle);
        self.set_mr_bits(384usize..448usize, r#vector);
        let info_in = seL4_MessageInfo::new(
            invocation_label::X86IRQIssueIRQHandlerMSI.into(),
            0,
            1u64,
            7u64,
        );
        let info_out = self.seL4_Call(service, info_in);
        let err: seL4_Error::Type = info_out.get_label().try_into().unwrap();
        err
    }
}
