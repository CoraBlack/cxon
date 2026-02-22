use crate::toolchain::ToolChainTrait;

pub struct LLVM {
    #[allow(dead_code)] data: ()
}

impl ToolChainTrait for LLVM {
    const CC:     &'static str = "clang";
    const CXX:    &'static str = "clang++";
    const LINKER: &'static str = "clang++";
    const DEBUG_FLAG:  &'static str = "-g";
    const OUTPUT_FLAG: &'static str = "-o";
    const ONLY_COMPILE_FLAG:    &'static str = "-c";
    const DEFINE_FLAG_PREFIX:   &'static str = "-D";
    const INCLUDE_FLAG_PREFIX:  &'static str = "-I";
    const LINK_DIR_FLAG_PREFIX: &'static str = "-L";
    const LINK_LIB_FLAG_PREFIX: &'static str = "-l";
}