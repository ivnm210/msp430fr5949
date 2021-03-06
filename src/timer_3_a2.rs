#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Timer3_A2 Control"]
    pub ta3ctl: TA3CTL,
    #[doc = "0x02 - Timer3_A2 Capture/Compare Control 0"]
    pub ta3cctl0: TA3CCTL0,
    #[doc = "0x04 - Timer3_A2 Capture/Compare Control 1"]
    pub ta3cctl1: TA3CCTL1,
    _reserved3: [u8; 10usize],
    #[doc = "0x10 - Timer3_A2"]
    pub ta3r: TA3R,
    #[doc = "0x12 - Timer3_A2 Capture/Compare 0"]
    pub ta3ccr0: TA3CCR0,
    #[doc = "0x14 - Timer3_A2 Capture/Compare 1"]
    pub ta3ccr1: TA3CCR1,
    _reserved6: [u8; 10usize],
    #[doc = "0x20 - Timer3_A2 Expansion Register 0"]
    pub ta3ex0: TA3EX0,
    _reserved7: [u8; 12usize],
    #[doc = "0x2e - Timer3_A2 Interrupt Vector Word"]
    pub ta3iv: TA3IV,
}
#[doc = "Timer3_A2 Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ta3ctl](ta3ctl) module"]
pub type TA3CTL = crate::Reg<u16, _TA3CTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TA3CTL;
#[doc = "`read()` method returns [ta3ctl::R](ta3ctl::R) reader structure"]
impl crate::Readable for TA3CTL {}
#[doc = "`write(|w| ..)` method takes [ta3ctl::W](ta3ctl::W) writer structure"]
impl crate::Writable for TA3CTL {}
#[doc = "Timer3_A2 Control"]
pub mod ta3ctl;
#[doc = "Timer3_A2 Capture/Compare Control 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ta3cctl0](ta3cctl0) module"]
pub type TA3CCTL0 = crate::Reg<u16, _TA3CCTL0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TA3CCTL0;
#[doc = "`read()` method returns [ta3cctl0::R](ta3cctl0::R) reader structure"]
impl crate::Readable for TA3CCTL0 {}
#[doc = "`write(|w| ..)` method takes [ta3cctl0::W](ta3cctl0::W) writer structure"]
impl crate::Writable for TA3CCTL0 {}
#[doc = "Timer3_A2 Capture/Compare Control 0"]
pub mod ta3cctl0;
#[doc = "Timer3_A2 Capture/Compare Control 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ta3cctl1](ta3cctl1) module"]
pub type TA3CCTL1 = crate::Reg<u16, _TA3CCTL1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TA3CCTL1;
#[doc = "`read()` method returns [ta3cctl1::R](ta3cctl1::R) reader structure"]
impl crate::Readable for TA3CCTL1 {}
#[doc = "`write(|w| ..)` method takes [ta3cctl1::W](ta3cctl1::W) writer structure"]
impl crate::Writable for TA3CCTL1 {}
#[doc = "Timer3_A2 Capture/Compare Control 1"]
pub mod ta3cctl1;
#[doc = "Timer3_A2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ta3r](ta3r) module"]
pub type TA3R = crate::Reg<u16, _TA3R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TA3R;
#[doc = "`read()` method returns [ta3r::R](ta3r::R) reader structure"]
impl crate::Readable for TA3R {}
#[doc = "`write(|w| ..)` method takes [ta3r::W](ta3r::W) writer structure"]
impl crate::Writable for TA3R {}
#[doc = "Timer3_A2"]
pub mod ta3r;
#[doc = "Timer3_A2 Capture/Compare 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ta3ccr0](ta3ccr0) module"]
pub type TA3CCR0 = crate::Reg<u16, _TA3CCR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TA3CCR0;
#[doc = "`read()` method returns [ta3ccr0::R](ta3ccr0::R) reader structure"]
impl crate::Readable for TA3CCR0 {}
#[doc = "`write(|w| ..)` method takes [ta3ccr0::W](ta3ccr0::W) writer structure"]
impl crate::Writable for TA3CCR0 {}
#[doc = "Timer3_A2 Capture/Compare 0"]
pub mod ta3ccr0;
#[doc = "Timer3_A2 Capture/Compare 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ta3ccr1](ta3ccr1) module"]
pub type TA3CCR1 = crate::Reg<u16, _TA3CCR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TA3CCR1;
#[doc = "`read()` method returns [ta3ccr1::R](ta3ccr1::R) reader structure"]
impl crate::Readable for TA3CCR1 {}
#[doc = "`write(|w| ..)` method takes [ta3ccr1::W](ta3ccr1::W) writer structure"]
impl crate::Writable for TA3CCR1 {}
#[doc = "Timer3_A2 Capture/Compare 1"]
pub mod ta3ccr1;
#[doc = "Timer3_A2 Expansion Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ta3ex0](ta3ex0) module"]
pub type TA3EX0 = crate::Reg<u16, _TA3EX0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TA3EX0;
#[doc = "`read()` method returns [ta3ex0::R](ta3ex0::R) reader structure"]
impl crate::Readable for TA3EX0 {}
#[doc = "`write(|w| ..)` method takes [ta3ex0::W](ta3ex0::W) writer structure"]
impl crate::Writable for TA3EX0 {}
#[doc = "Timer3_A2 Expansion Register 0"]
pub mod ta3ex0;
#[doc = "Timer3_A2 Interrupt Vector Word\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ta3iv](ta3iv) module"]
pub type TA3IV = crate::Reg<u16, _TA3IV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TA3IV;
#[doc = "`read()` method returns [ta3iv::R](ta3iv::R) reader structure"]
impl crate::Readable for TA3IV {}
#[doc = "`write(|w| ..)` method takes [ta3iv::W](ta3iv::W) writer structure"]
impl crate::Writable for TA3IV {}
#[doc = "Timer3_A2 Interrupt Vector Word"]
pub mod ta3iv;
