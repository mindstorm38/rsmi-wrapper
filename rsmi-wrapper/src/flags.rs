use bitflags::bitflags;
use rsmi_wrapper_sys::*;


bitflags! {
    pub struct InitFlags: u64 {
        const ALL_GPUS = rsmi_init_flags_t_RSMI_INIT_FLAG_ALL_GPUS;
        const RESERVED_TEST1 = rsmi_init_flags_t_RSMI_INIT_FLAG_RESRV_TEST1;
    }
}
