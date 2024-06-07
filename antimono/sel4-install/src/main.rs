// # The install target is only created if this is the top level project.
// # We don't currently support creating install targets if the kernel is
// # imported in another project.
//( AKE_SOURCE_DIR}") STREQUAL format!"${CMAKE_CURRENT_SOURCE_DIR}"))
//
// # Import libsel4 to get access to generation targets
// add_subdirectory(libsel4)
// # Add a default target that builds kernel.elf and generates all libsel4 headers
// add_custom_target(single-project ALL DEPENDS sel4_generated kernel.elf)
// # Disable the libsel4.a target as we don't intend to build the libsel4 sources
// set_target_properties(sel4 PROPERTIES EXCLUDE_FROM_ALL ON)
// # Install kernel.elf to bin/kernel.elf
// install(TARGETS kernel.elf RUNTIME DESTINATION bin)
// # Install all libsel4 headers to libsel4/include
// install(
// DIRECTORY
// for(mtENT_SOURCE_DIR}/libsel4/include/")
// for(mtENT_SOURCE_DIR}/libsel4/arch_include/${KernelArch}/")
// for(mtENT_SOURCE_DIR}/libsel4/sel4_arch_include/${KernelSel4Arch}/")
// for(mtENT_SOURCE_DIR}/libsel4/sel4_plat_include/${KernelPlatform}/")
// for(mtENT_SOURCE_DIR}/libsel4/mode_include/${KernelWordSize}/")
// for(mtENT_BINARY_DIR}/libsel4/include/")
// for(mtENT_BINARY_DIR}/libsel4/arch_include/${KernelArch}/")
// for(mtENT_BINARY_DIR}/libsel4/sel4_arch_include/${KernelSel4Arch}/")
// # The following directories install the autoconf headers
// for(mtENT_BINARY_DIR}/gen_config/")
// for(mtENT_BINARY_DIR}/libsel4/gen_config/")
// for(mtENT_BINARY_DIR}/libsel4/autoconf/")
// DESTINATION libsel4/include
// FILES_MATCHING
// PATTERN "*.h"
// PATTERN "*.pbf"
// PATTERN "api/syscall.xml"
// PATTERN "api/syscall.xsd"
// PATTERN "gen_config.json"
// )
// # Manually install object API files with non-conflicting names
// install(
// FI(LS E_CURRENT_SOURCE_DIR}/libsel4/include/interfaces/sel4.xml")
// DESTINATION libsel4/include/interfaces
// RENAME object-api.xml
// )
// install(
// FILES
// for(mtENT_SOURCE_DIR}/libsel4/arch_include/${KernelArch}/interfaces/sel4arch.xml")
// DESTINATION libsel4/include/interfaces
// RENAME object-api-arch.xml
// )
// install(
// FILES
// for(mtENT_SOURCE_DIR}/libsel4/sel4_arch_include/${KernelSel4Arch}/interfaces/sel4arch.xml")
// DESTINATION libsel4/include/interfaces
// RENAME object-api-sel4-arch.xml
// )
// # Install libsel4 sources to libsel4/src
// inst(ormat!"${CMAKE_CURRENT_SOURCE_DIR}/libsel4/src/") DESTINATION libsel4/src)
// # Install additional support files
// if(DEFINED KernelDTBPath)
// install(FILES ${KernelDTBPath} DESTINATION support)
// endif()
// if(DEFINED platform_yaml)
// install(FILES ${platform_yaml} DESTINATION support)
// endif()
// if(DEFINED platform_json)
// install(FILES ${platform_json} DESTINATION support)
// endif()
//
// endif()

/// this can be a build.rs
fn main() {
    let kernel_arch = "x86_64";
    let kernel_sel4_arch = "";
    let kernel_platform = "";
    let kernel_word_size = "64";
    // convention
    let base_dir = "/Users/mag1cian/dev/os-related/sel4-tutorials-manifest";
    let binary_dir = base_dir.to_string() + "/build";
    let source_dir = base_dir.to_string() + "/kernel";
    let dest_base = "install";
    // ignore sel4 kernel.elf binary
    let dest_dir = dest_base.to_string() + "libsel4/include";

    let dirs = vec![
        format!("{source_dir}/libsel4/include/"),
        format!("{source_dir}/libsel4/arch_include/${kernel_arch}/"),
        format!("{source_dir}/libsel4/sel4_arch_include/${kernel_sel4_arch}/"),
        format!("{source_dir}/libsel4/sel4_plat_include/${kernel_platform}/"),
        format!("{source_dir}/libsel4/mode_include/${kernel_word_size}/"),
        format!("{binary_dir}/libsel4/include/"),
        format!("{binary_dir}/libsel4/arch_include/${kernel_arch}/"),
        format!("{binary_dir}/libsel4/sel4_arch_include/${kernel_sel4_arch}/"),
// The following directories install the autoconf headers
        format!("{binary_dir}/gen_config/"),
        format!("{binary_dir}/libsel4/gen_config/"),
        format!("{binary_dir}/libsel4/autoconf/"),
    ];


    println!("Hello, world!");
}
