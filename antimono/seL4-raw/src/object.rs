use crate::structures::Cap;

mod cnode;
mod untyped;

fn is_cap_revocable(derived_cap: Cap, src_cap: Cap) -> bool {
    false
}
