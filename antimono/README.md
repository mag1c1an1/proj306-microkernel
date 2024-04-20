thanks
 Nick Spinale
 nick@nickspinable.com
 github.com/nspin


 cap: 代表某种能力和权限, 指向一个能力对象.
 cte: 代表一个 slot, 里面放一个能力, 并且形成一种链表

x86_64 only

# ![feature(alloc_error_handler)]
# ![feature(const_mut_refs)]
# ![feature(const_ptr_sub_ptr)]
# ![feature(const_trait_impl)]
# ![feature(coroutines)]
# ![feature(fn_traits)]
# ![feature(iter_from_coroutine)]
# ![feature(let_chains)]
# ![feature(negative_impls)]
# ![feature(new_uninit)]
# ![feature(panic_info_message)]
# ![feature(ptr_sub_ptr)]
# ![feature(strict_provenance)]
# ![allow(dead_code)]
# ![allow(unused_variables)]
# ![no_std]

# ![no_std]
# ![forbid(unsafe_code)]
# ![allow(dead_code)]
# ![allow(incomplete_features)]
# ![allow(unused_variables)]
# ![feature(btree_cursors)]
# ![feature(btree_extract_if)]
# ![feature(const_option)]
# ![feature(exclusive_range_pattern)]
# ![feature(extend_one)]
# ![feature(fn_traits)]
# ![feature(format_args_nl)]
# ![feature(int_roundings)]
# ![feature(let_chains)]
# ![feature(linked_list_remove)]
# ![feature(register_tool)]
// FIXME: This feature is used to support vm capbility now as a work around.
// Since this is an incomplete feature, use this feature is unsafe.
// We should find a proper method to replace this feature with min_specialization, which is a sound feature.
# ![feature(specialization)]
# ![feature(step_trait)]
# ![feature(trait_alias)]
# ![register_tool(component_access_control)]

# ![no_std]
# ![forbid(unsafe_code)]
# ![feature(fn_traits)]
# ![feature(step_trait)]
# ![allow(dead_code)]

# ![no_std]
# ![forbid(unsafe_code)]
# ![feature(strict_provenance)]

[package]
name = "aster-nix"
version = "0.1.0"
edition = "2021"

# See more keys and their definitions at <https://doc.rust-lang.org/cargo/reference/manifest.html>

[dependencies]
aster-frame = { path = "../../framework/aster-frame" }
align_ext = { path = "../../framework/libs/align_ext" }
pod = { git = "<https://github.com/asterinas/pod>", rev = "d7dba56" }
aster-input = { path = "../comps/input" }
aster-block = { path = "../comps/block" }
aster-network = { path = "../comps/network" }
aster-console = { path = "../comps/console" }
aster-time = { path = "../comps/time" }
aster-virtio = { path = "../comps/virtio" }
aster-rights = { path = "../libs/aster-rights" }
controlled = { path = "../libs/comp-sys/controlled" }
typeflags = { path = "../libs/typeflags" }
typeflags-util = { path = "../libs/typeflags-util" }
aster-rights-proc = { path = "../libs/aster-rights-proc" }
aster-util = { path = "../libs/aster-util" }
int-to-c-enum = { path = "../libs/int-to-c-enum" }
cpio-decoder = { path = "../libs/cpio-decoder" }
ascii = { version = "1.1", default-features = false, features = ["alloc"] }
intrusive-collections = "0.9.5"
time = { version = "0.3", default-features = false, features = ["alloc"] }
smoltcp = { version = "0.9.1", default-features = false, features = [
    "alloc",
    "log",
    "medium-ethernet",
    "medium-ip",
    "proto-dhcpv4",
    "proto-ipv4",
    "proto-igmp",
    "socket-icmp",
    "socket-udp",
    "socket-tcp",
    "socket-raw",
    "socket-dhcpv4",
] }
ktest = { path = "../../framework/libs/ktest" }
tdx-guest = { path = "../../framework/libs/tdx-guest", optional = true }

# parse elf file

xmas-elf = "0.8.0"

# goblin = {version= "0.5.3", default-features = false, features = ["elf64"]}

# data-structures

bitflags = "1.3"
ringbuf = { version = "0.3.2", default-features = false, features = ["alloc"] }
keyable-arc = { path = "../libs/keyable-arc" }

# unzip initramfs

libflate = { version ="2", default-features = false }
core2 = { version = "0.4", default_features = false, features = ["alloc"] }
lending-iterator = "0.1.7"
spin = "0.9.4"
vte = "0.10"
lru = "0.12.3"
log = "0.4"
getrandom = { version = "0.2.10", default-features = false, features = [
    "rdrand",
] }
bitvec = { version = "1.0", default-features = false, features = ["alloc"] }
hashbrown = "0.14"
rand = {version = "0.8.5", default-features = false, features = ["small_rng"]}
static_assertions = "1.1.0"
inherit-methods-macro = { git = "<https://github.com/asterinas/inherit-methods-macro>", rev = "98f7e3e" }
getset = "0.1.2"
atomic = "0.6"
bytemuck = "1.14.3"
bytemuck_derive = "1.5.0"
takeable = "0.2.2"

[dependencies.lazy_static]
version = "1.0"
features = ["spin_no_std"]

[features]
intel_tdx = ["dep:tdx-guest"]
