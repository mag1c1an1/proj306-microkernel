use crate::structures::cap::Cap;

mod cnode;
mod untyped;

fn is_cap_revocable(derived_cap: Cap, src_cap: Cap) -> bool {
    todo!()
}

pub mod arch;

type CPtrBits = u64;


struct CPtr {
    inner: CPtrBits,
}



// canbe method
// deriveCap_ret_t deriveCap(cte_t *slot, cap_t cap);
// finaliseCap_ret_t finaliseCap(cap_t cap, bool_t final, bool_t exposed);

// bool_t CONST hasCancelSendRights(cap_t cap);
// bool_t CONST sameRegionAs(cap_t cap_a, cap_t cap_b);
// bool_t CONST sameObjectAs(cap_t cap_a, cap_t cap_b);
// cap_t CONST updateCapData(bool_t preserve, word_t newData, cap_t cap);

// type?
pub fn mask_cap_rights(rights: u64, cap: Cap) -> Cap {
    todo!()
}
// cap_t createObject(object_t t, void *regionBase, word_t, bool_t deviceMemory);
pub fn create_object() {}
// void createNewObjects(object_t t, cte_t *parent,
//                       cte_t *destCNode, word_t destOffset, word_t destLength,
//                       void *regionBase, word_t userSize, bool_t deviceMemory);
// #ifdef CONFIG_KERNEL_MCS
// exception_t decodeInvocation(word_t invLabel, word_t length,
//                              cptr_t capIndex, cte_t *slot, cap_t cap,
//                              bool_t block, bool_t call,
//                              bool_t canDonate, bool_t firstPhase, word_t *buffer);
// exception_t performInvocation_Endpoint(endpoint_t *ep, word_t badge,
//                                        bool_t canGrant, bool_t canGrantReply,
//                                        bool_t block, bool_t call, bool_t canDonate);
// exception_t performInvocation_Notification(notification_t *ntfn,
//                                            word_t badge);
// exception_t performInvocation_Reply(tcb_t *thread, reply_t *reply, bool_t canGrant);
// #else
// exception_t decodeInvocation(word_t invLabel, word_t length,
//                              cptr_t capIndex, cte_t *slot, cap_t cap,
//                              bool_t block, bool_t call, word_t *buffer);
// exception_t performInvocation_Endpoint(endpoint_t *ep, word_t badge,
//                                        bool_t canGrant, bool_t canGrantReply,
//                                        bool_t block, bool_t call);
// exception_t performInvocation_Notification(notification_t *ntfn,
//                                            word_t badge);
// exception_t performInvocation_Reply(tcb_t *thread, cte_t *slot, bool_t canGrant);
// #endif
// word_t getObjectSize(word_t t, word_t userObjSize);

// static inline void postCapDeletion(cap_t cap)
// {
//     if (cap_get_capType(cap) == cap_irq_handler_cap) {
//         irq_t irq = IDX_TO_IRQT(cap_irq_handler_cap_get_capIRQ(cap));
//         deletedIRQHandler(irq);
//     } else if (isArchCap(cap)) {
//         Arch_postCapDeletion(cap);
//     }
// }

// method
// word_t CONST cap_get_capSizeBits(cap_t cap);
// method
// bool_t CONST cap_get_capIsPhysical(cap_t cap);
// method
// void *CONST cap_get_capPtr(cap_t cap);

#[derive(Debug, Clone,Copy,PartialEq, PartialOrd)]
pub enum ObjectType {

}