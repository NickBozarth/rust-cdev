// Force the compiler to embed the GNU BTI note property into the final object file
// TODO ensure that other architectures are supported
#[cfg(target_arch = "aarch64")]
core::arch::global_asm!(
    ".section .note.gnu.property, \"a\"",
    ".p2align 3",
    ".word 4",           // Name size (Length of "GNU\0")
    ".word 16",          // Data size
    ".word 5",           // Type (NT_GNU_PROPERTY_TYPE_0)
    ".asciz \"GNU\"",    // Owner Name
    
    // Property: GNU_PROPERTY_AARCH64_FEATURE_1_AND
    ".word 0xc0000000",  // pr_type
    ".word 4",           // pr_datasz
    ".word 1",           // pr_data (GNU_PROPERTY_AARCH64_FEATURE_1_BTI = 0x1)
    ".p2align 3"
);
