use std::sync::OnceLock;

static CPU_FEATURES: OnceLock<CpuFeatures> = OnceLock::new();

#[derive(Clone, Copy)]
pub struct CpuFeatures {
    pub ssse3: bool,
    pub avx2: bool,
    pub avx512f: bool,
    pub avx512bw: bool,
    pub feature_simd_ssse3: bool,
    pub feature_simd_avx2: bool,
    pub feature_simd_avx512: bool,
}

impl CpuFeatures {
    fn init() -> Self {
        #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
        {
            let ssse3 = is_x86_feature_detected!("ssse3");
            let avx2 = is_x86_feature_detected!("avx2");
            let avx512f = is_x86_feature_detected!("avx512f");
            let avx512bw = is_x86_feature_detected!("avx512bw");
            Self {
                ssse3,
                avx2,
                avx512f,
                avx512bw,
                feature_simd_ssse3: ssse3,
                feature_simd_avx2: avx2,
                feature_simd_avx512: avx512f && avx512bw,
            }
        }
        #[cfg(not(any(target_arch = "x86", target_arch = "x86_64")))]
        {
            Self {
                ssse3: false,
                avx2: false,
                avx512f: false,
                avx512bw: false,
                feature_simd_ssse3: false,
                feature_simd_avx2: false,
                feature_simd_avx512: false,
            }
        }
    }
}

#[inline(always)]
pub fn has_ssse3() -> bool {
    CPU_FEATURES.get_or_init(CpuFeatures::init).ssse3
}

#[inline(always)]
pub fn has_avx2() -> bool {
    CPU_FEATURES.get_or_init(CpuFeatures::init).avx2
}

#[inline(always)]
pub fn has_avx512f() -> bool {
    CPU_FEATURES.get_or_init(CpuFeatures::init).avx512f
}

#[inline(always)]
pub fn has_avx512bw() -> bool {
    CPU_FEATURES.get_or_init(CpuFeatures::init).avx512bw
}

#[inline(always)]
pub fn is_available_feature_simd_ssse3() -> bool {
    CPU_FEATURES
        .get_or_init(CpuFeatures::init)
        .feature_simd_ssse3
}

#[inline(always)]
pub fn is_available_feature_simd_avx2() -> bool {
    CPU_FEATURES
        .get_or_init(CpuFeatures::init)
        .feature_simd_avx2
}

#[inline(always)]
pub fn is_available_feature_simd_avx512() -> bool {
    CPU_FEATURES
        .get_or_init(CpuFeatures::init)
        .feature_simd_avx512
}
