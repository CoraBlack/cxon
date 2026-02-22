use crate::toolchain::ToolChainTrait;

pub struct MSVC {
    #[allow(dead_code)] data: ()
}

impl ToolChainTrait for MSVC {
    const CC:     &'static str = "cl";
    const CXX:    &'static str = "cl";
    const LINKER: &'static str = "link";
    const DEBUG_FLAG:  &'static str = "/Zi";
    const OUTPUT_FLAG: &'static str = "/Fe:";
    const ONLY_COMPILE_FLAG:    &'static str = "/c";
    const DEFINE_FLAG_PREFIX:   &'static str = "/D";
    const INCLUDE_FLAG_PREFIX:  &'static str = "/I";
    const LINK_DIR_FLAG_PREFIX: &'static str = "/LIBPATH:";
    const LINK_LIB_FLAG_PREFIX: &'static str = "";
}