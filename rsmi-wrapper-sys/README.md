# rsmi-wrapper-sys
Rust bindings for the [ROCm SMI](https://github.com/RadeonOpenCompute/rocm_smi_lib) library. This crate provides raw unsafe bindings.

## Generated
These bindings were generated from ROCm SMI version `rocm-5.2.0` headers, with the following command:
```console
bindgen include/rocm_smi/rocm_smi.h -o src/bindings.rs --no-layout-tests --no-doc-comments --dynamic-loading RsmiLib -- -Iinclude/
```
