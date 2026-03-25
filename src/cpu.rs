use std::sync::OnceLock;

static CPU_FEATURES: OnceLock<CpuFeatures> = OnceLock::new();

#[derive(Clone, Copy)]
pub struct CpuFeatures {
    pub ssse3: bool,
    pub avx2: bool,
    pub avx512f: bool,
}

impl CpuFeatures {
    fn detect() -> Self {
        #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
        {
            Self {
                ssse3: is_x86_feature_detected!("ssse3"),
                avx2: is_x86_feature_detected!("avx2"),
                avx512f: is_x86_feature_detected!("avx512f"),
            }
        }
        #[cfg(not(any(target_arch = "x86", target_arch = "x86_64")))]
        {
            Self {
                ssse3: false,
                avx2: false,
                avx512f: false,
            }
        }
    }
}

#[inline(always)]
pub fn has_ssse3() -> bool {
    CPU_FEATURES.get_or_init(CpuFeatures::detect).ssse3
}

#[inline(always)]
pub fn has_avx2() -> bool {
    CPU_FEATURES.get_or_init(CpuFeatures::detect).avx2
}

#[inline(always)]
pub fn has_avx512f() -> bool {
    CPU_FEATURES.get_or_init(CpuFeatures::detect).avx512f
}
