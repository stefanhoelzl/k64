#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Flash Access Protection Register"]
    pub pfapr: PFAPR,
    #[doc = "0x04 - Flash Bank 0 Control Register"]
    pub pfb0cr: PFB0CR,
    #[doc = "0x08 - Flash Bank 1 Control Register"]
    pub pfb1cr: PFB1CR,
    _reserved3: [u8; 244usize],
    #[doc = "0x100 - Cache Tag Storage"]
    pub tagvdw0s: [TAGVDW0S; 4],
    #[doc = "0x110 - Cache Tag Storage"]
    pub tagvdw1s: [TAGVDW1S; 4],
    #[doc = "0x120 - Cache Tag Storage"]
    pub tagvdw2s: [TAGVDW2S; 4],
    #[doc = "0x130 - Cache Tag Storage"]
    pub tagvdw3s: [TAGVDW3S; 4],
    _reserved7: [u8; 192usize],
    #[doc = "0x200 - Cache Data Storage (upper word)"]
    pub dataw0s0u: DATAW0SU,
    #[doc = "0x204 - Cache Data Storage (lower word)"]
    pub dataw0s0l: DATAW0SL,
    #[doc = "0x208 - Cache Data Storage (upper word)"]
    pub dataw0s1u: DATAW0SU,
    #[doc = "0x20c - Cache Data Storage (lower word)"]
    pub dataw0s1l: DATAW0SL,
    #[doc = "0x210 - Cache Data Storage (upper word)"]
    pub dataw0s2u: DATAW0SU,
    #[doc = "0x214 - Cache Data Storage (lower word)"]
    pub dataw0s2l: DATAW0SL,
    #[doc = "0x218 - Cache Data Storage (upper word)"]
    pub dataw0s3u: DATAW0SU,
    #[doc = "0x21c - Cache Data Storage (lower word)"]
    pub dataw0s3l: DATAW0SL,
    #[doc = "0x220 - Cache Data Storage (upper word)"]
    pub dataw1s0u: DATAW1SU,
    #[doc = "0x224 - Cache Data Storage (lower word)"]
    pub dataw1s0l: DATAW1SL,
    #[doc = "0x228 - Cache Data Storage (upper word)"]
    pub dataw1s1u: DATAW1SU,
    #[doc = "0x22c - Cache Data Storage (lower word)"]
    pub dataw1s1l: DATAW1SL,
    #[doc = "0x230 - Cache Data Storage (upper word)"]
    pub dataw1s2u: DATAW1SU,
    #[doc = "0x234 - Cache Data Storage (lower word)"]
    pub dataw1s2l: DATAW1SL,
    #[doc = "0x238 - Cache Data Storage (upper word)"]
    pub dataw1s3u: DATAW1SU,
    #[doc = "0x23c - Cache Data Storage (lower word)"]
    pub dataw1s3l: DATAW1SL,
    #[doc = "0x240 - Cache Data Storage (upper word)"]
    pub dataw2s0u: DATAW2SU,
    #[doc = "0x244 - Cache Data Storage (lower word)"]
    pub dataw2s0l: DATAW2SL,
    #[doc = "0x248 - Cache Data Storage (upper word)"]
    pub dataw2s1u: DATAW2SU,
    #[doc = "0x24c - Cache Data Storage (lower word)"]
    pub dataw2s1l: DATAW2SL,
    #[doc = "0x250 - Cache Data Storage (upper word)"]
    pub dataw2s2u: DATAW2SU,
    #[doc = "0x254 - Cache Data Storage (lower word)"]
    pub dataw2s2l: DATAW2SL,
    #[doc = "0x258 - Cache Data Storage (upper word)"]
    pub dataw2s3u: DATAW2SU,
    #[doc = "0x25c - Cache Data Storage (lower word)"]
    pub dataw2s3l: DATAW2SL,
    #[doc = "0x260 - Cache Data Storage (upper word)"]
    pub dataw3s0u: DATAW3SU,
    #[doc = "0x264 - Cache Data Storage (lower word)"]
    pub dataw3s0l: DATAW3SL,
    #[doc = "0x268 - Cache Data Storage (upper word)"]
    pub dataw3s1u: DATAW3SU,
    #[doc = "0x26c - Cache Data Storage (lower word)"]
    pub dataw3s1l: DATAW3SL,
    #[doc = "0x270 - Cache Data Storage (upper word)"]
    pub dataw3s2u: DATAW3SU,
    #[doc = "0x274 - Cache Data Storage (lower word)"]
    pub dataw3s2l: DATAW3SL,
    #[doc = "0x278 - Cache Data Storage (upper word)"]
    pub dataw3s3u: DATAW3SU,
    #[doc = "0x27c - Cache Data Storage (lower word)"]
    pub dataw3s3l: DATAW3SL,
}
#[doc = "Flash Access Protection Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pfapr](pfapr) module"]
pub type PFAPR = crate::Reg<u32, _PFAPR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PFAPR;
#[doc = "`read()` method returns [pfapr::R](pfapr::R) reader structure"]
impl crate::Readable for PFAPR {}
#[doc = "`write(|w| ..)` method takes [pfapr::W](pfapr::W) writer structure"]
impl crate::Writable for PFAPR {}
#[doc = "Flash Access Protection Register"]
pub mod pfapr;
#[doc = "Flash Bank 0 Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pfb0cr](pfb0cr) module"]
pub type PFB0CR = crate::Reg<u32, _PFB0CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PFB0CR;
#[doc = "`read()` method returns [pfb0cr::R](pfb0cr::R) reader structure"]
impl crate::Readable for PFB0CR {}
#[doc = "`write(|w| ..)` method takes [pfb0cr::W](pfb0cr::W) writer structure"]
impl crate::Writable for PFB0CR {}
#[doc = "Flash Bank 0 Control Register"]
pub mod pfb0cr;
#[doc = "Flash Bank 1 Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pfb1cr](pfb1cr) module"]
pub type PFB1CR = crate::Reg<u32, _PFB1CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PFB1CR;
#[doc = "`read()` method returns [pfb1cr::R](pfb1cr::R) reader structure"]
impl crate::Readable for PFB1CR {}
#[doc = "`write(|w| ..)` method takes [pfb1cr::W](pfb1cr::W) writer structure"]
impl crate::Writable for PFB1CR {}
#[doc = "Flash Bank 1 Control Register"]
pub mod pfb1cr;
#[doc = "Cache Tag Storage\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [tagvdw0s](tagvdw0s) module"]
pub type TAGVDW0S = crate::Reg<u32, _TAGVDW0S>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TAGVDW0S;
#[doc = "`read()` method returns [tagvdw0s::R](tagvdw0s::R) reader structure"]
impl crate::Readable for TAGVDW0S {}
#[doc = "`write(|w| ..)` method takes [tagvdw0s::W](tagvdw0s::W) writer structure"]
impl crate::Writable for TAGVDW0S {}
#[doc = "Cache Tag Storage"]
pub mod tagvdw0s;
#[doc = "Cache Tag Storage\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [tagvdw1s](tagvdw1s) module"]
pub type TAGVDW1S = crate::Reg<u32, _TAGVDW1S>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TAGVDW1S;
#[doc = "`read()` method returns [tagvdw1s::R](tagvdw1s::R) reader structure"]
impl crate::Readable for TAGVDW1S {}
#[doc = "`write(|w| ..)` method takes [tagvdw1s::W](tagvdw1s::W) writer structure"]
impl crate::Writable for TAGVDW1S {}
#[doc = "Cache Tag Storage"]
pub mod tagvdw1s;
#[doc = "Cache Tag Storage\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [tagvdw2s](tagvdw2s) module"]
pub type TAGVDW2S = crate::Reg<u32, _TAGVDW2S>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TAGVDW2S;
#[doc = "`read()` method returns [tagvdw2s::R](tagvdw2s::R) reader structure"]
impl crate::Readable for TAGVDW2S {}
#[doc = "`write(|w| ..)` method takes [tagvdw2s::W](tagvdw2s::W) writer structure"]
impl crate::Writable for TAGVDW2S {}
#[doc = "Cache Tag Storage"]
pub mod tagvdw2s;
#[doc = "Cache Tag Storage\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [tagvdw3s](tagvdw3s) module"]
pub type TAGVDW3S = crate::Reg<u32, _TAGVDW3S>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TAGVDW3S;
#[doc = "`read()` method returns [tagvdw3s::R](tagvdw3s::R) reader structure"]
impl crate::Readable for TAGVDW3S {}
#[doc = "`write(|w| ..)` method takes [tagvdw3s::W](tagvdw3s::W) writer structure"]
impl crate::Writable for TAGVDW3S {}
#[doc = "Cache Tag Storage"]
pub mod tagvdw3s;
#[doc = "Cache Data Storage (upper word)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dataw0su](dataw0su) module"]
pub type DATAW0SU = crate::Reg<u32, _DATAW0SU>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DATAW0SU;
#[doc = "`read()` method returns [dataw0su::R](dataw0su::R) reader structure"]
impl crate::Readable for DATAW0SU {}
#[doc = "`write(|w| ..)` method takes [dataw0su::W](dataw0su::W) writer structure"]
impl crate::Writable for DATAW0SU {}
#[doc = "Cache Data Storage (upper word)"]
pub mod dataw0su;
#[doc = "Cache Data Storage (lower word)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dataw0sl](dataw0sl) module"]
pub type DATAW0SL = crate::Reg<u32, _DATAW0SL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DATAW0SL;
#[doc = "`read()` method returns [dataw0sl::R](dataw0sl::R) reader structure"]
impl crate::Readable for DATAW0SL {}
#[doc = "`write(|w| ..)` method takes [dataw0sl::W](dataw0sl::W) writer structure"]
impl crate::Writable for DATAW0SL {}
#[doc = "Cache Data Storage (lower word)"]
pub mod dataw0sl;
#[doc = "Cache Data Storage (upper word)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dataw1su](dataw1su) module"]
pub type DATAW1SU = crate::Reg<u32, _DATAW1SU>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DATAW1SU;
#[doc = "`read()` method returns [dataw1su::R](dataw1su::R) reader structure"]
impl crate::Readable for DATAW1SU {}
#[doc = "`write(|w| ..)` method takes [dataw1su::W](dataw1su::W) writer structure"]
impl crate::Writable for DATAW1SU {}
#[doc = "Cache Data Storage (upper word)"]
pub mod dataw1su;
#[doc = "Cache Data Storage (lower word)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dataw1sl](dataw1sl) module"]
pub type DATAW1SL = crate::Reg<u32, _DATAW1SL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DATAW1SL;
#[doc = "`read()` method returns [dataw1sl::R](dataw1sl::R) reader structure"]
impl crate::Readable for DATAW1SL {}
#[doc = "`write(|w| ..)` method takes [dataw1sl::W](dataw1sl::W) writer structure"]
impl crate::Writable for DATAW1SL {}
#[doc = "Cache Data Storage (lower word)"]
pub mod dataw1sl;
#[doc = "Cache Data Storage (upper word)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dataw2su](dataw2su) module"]
pub type DATAW2SU = crate::Reg<u32, _DATAW2SU>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DATAW2SU;
#[doc = "`read()` method returns [dataw2su::R](dataw2su::R) reader structure"]
impl crate::Readable for DATAW2SU {}
#[doc = "`write(|w| ..)` method takes [dataw2su::W](dataw2su::W) writer structure"]
impl crate::Writable for DATAW2SU {}
#[doc = "Cache Data Storage (upper word)"]
pub mod dataw2su;
#[doc = "Cache Data Storage (lower word)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dataw2sl](dataw2sl) module"]
pub type DATAW2SL = crate::Reg<u32, _DATAW2SL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DATAW2SL;
#[doc = "`read()` method returns [dataw2sl::R](dataw2sl::R) reader structure"]
impl crate::Readable for DATAW2SL {}
#[doc = "`write(|w| ..)` method takes [dataw2sl::W](dataw2sl::W) writer structure"]
impl crate::Writable for DATAW2SL {}
#[doc = "Cache Data Storage (lower word)"]
pub mod dataw2sl;
#[doc = "Cache Data Storage (upper word)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dataw3su](dataw3su) module"]
pub type DATAW3SU = crate::Reg<u32, _DATAW3SU>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DATAW3SU;
#[doc = "`read()` method returns [dataw3su::R](dataw3su::R) reader structure"]
impl crate::Readable for DATAW3SU {}
#[doc = "`write(|w| ..)` method takes [dataw3su::W](dataw3su::W) writer structure"]
impl crate::Writable for DATAW3SU {}
#[doc = "Cache Data Storage (upper word)"]
pub mod dataw3su;
#[doc = "Cache Data Storage (lower word)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dataw3sl](dataw3sl) module"]
pub type DATAW3SL = crate::Reg<u32, _DATAW3SL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DATAW3SL;
#[doc = "`read()` method returns [dataw3sl::R](dataw3sl::R) reader structure"]
impl crate::Readable for DATAW3SL {}
#[doc = "`write(|w| ..)` method takes [dataw3sl::W](dataw3sl::W) writer structure"]
impl crate::Writable for DATAW3SL {}
#[doc = "Cache Data Storage (lower word)"]
pub mod dataw3sl;
