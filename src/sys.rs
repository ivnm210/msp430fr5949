#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - System control"]
    pub sysctl: SYSCTL,
    _reserved1: [u8; 4usize],
    #[doc = "0x06 - JTAG mailbox control"]
    pub sysjmbc: SYSJMBC,
    #[doc = "0x08 - JTAG mailbox input 0"]
    pub sysjmbi0: SYSJMBI0,
    #[doc = "0x0a - JTAG mailbox input 1"]
    pub sysjmbi1: SYSJMBI1,
    #[doc = "0x0c - JTAG mailbox output 0"]
    pub sysjmbo0: SYSJMBO0,
    #[doc = "0x0e - JTAG mailbox output 1"]
    pub sysjmbo1: SYSJMBO1,
    _reserved6: [u8; 10usize],
    #[doc = "0x1a - User NMI vector generator"]
    pub sysuniv: SYSUNIV,
    #[doc = "0x1c - System NMI vector generator"]
    pub syssniv: SYSSNIV,
    #[doc = "0x1e - Reset vector generator"]
    pub sysrstiv: SYSRSTIV,
}
#[doc = "System control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sysctl](sysctl) module"]
pub type SYSCTL = crate::Reg<u16, _SYSCTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SYSCTL;
#[doc = "`read()` method returns [sysctl::R](sysctl::R) reader structure"]
impl crate::Readable for SYSCTL {}
#[doc = "`write(|w| ..)` method takes [sysctl::W](sysctl::W) writer structure"]
impl crate::Writable for SYSCTL {}
#[doc = "System control"]
pub mod sysctl;
#[doc = "JTAG mailbox control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sysjmbc](sysjmbc) module"]
pub type SYSJMBC = crate::Reg<u16, _SYSJMBC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SYSJMBC;
#[doc = "`read()` method returns [sysjmbc::R](sysjmbc::R) reader structure"]
impl crate::Readable for SYSJMBC {}
#[doc = "`write(|w| ..)` method takes [sysjmbc::W](sysjmbc::W) writer structure"]
impl crate::Writable for SYSJMBC {}
#[doc = "JTAG mailbox control"]
pub mod sysjmbc;
#[doc = "JTAG mailbox input 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sysjmbi0](sysjmbi0) module"]
pub type SYSJMBI0 = crate::Reg<u16, _SYSJMBI0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SYSJMBI0;
#[doc = "`read()` method returns [sysjmbi0::R](sysjmbi0::R) reader structure"]
impl crate::Readable for SYSJMBI0 {}
#[doc = "`write(|w| ..)` method takes [sysjmbi0::W](sysjmbi0::W) writer structure"]
impl crate::Writable for SYSJMBI0 {}
#[doc = "JTAG mailbox input 0"]
pub mod sysjmbi0;
#[doc = "JTAG mailbox input 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sysjmbi1](sysjmbi1) module"]
pub type SYSJMBI1 = crate::Reg<u16, _SYSJMBI1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SYSJMBI1;
#[doc = "`read()` method returns [sysjmbi1::R](sysjmbi1::R) reader structure"]
impl crate::Readable for SYSJMBI1 {}
#[doc = "`write(|w| ..)` method takes [sysjmbi1::W](sysjmbi1::W) writer structure"]
impl crate::Writable for SYSJMBI1 {}
#[doc = "JTAG mailbox input 1"]
pub mod sysjmbi1;
#[doc = "JTAG mailbox output 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sysjmbo0](sysjmbo0) module"]
pub type SYSJMBO0 = crate::Reg<u16, _SYSJMBO0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SYSJMBO0;
#[doc = "`read()` method returns [sysjmbo0::R](sysjmbo0::R) reader structure"]
impl crate::Readable for SYSJMBO0 {}
#[doc = "`write(|w| ..)` method takes [sysjmbo0::W](sysjmbo0::W) writer structure"]
impl crate::Writable for SYSJMBO0 {}
#[doc = "JTAG mailbox output 0"]
pub mod sysjmbo0;
#[doc = "JTAG mailbox output 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sysjmbo1](sysjmbo1) module"]
pub type SYSJMBO1 = crate::Reg<u16, _SYSJMBO1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SYSJMBO1;
#[doc = "`read()` method returns [sysjmbo1::R](sysjmbo1::R) reader structure"]
impl crate::Readable for SYSJMBO1 {}
#[doc = "`write(|w| ..)` method takes [sysjmbo1::W](sysjmbo1::W) writer structure"]
impl crate::Writable for SYSJMBO1 {}
#[doc = "JTAG mailbox output 1"]
pub mod sysjmbo1;
#[doc = "User NMI vector generator\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sysuniv](sysuniv) module"]
pub type SYSUNIV = crate::Reg<u16, _SYSUNIV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SYSUNIV;
#[doc = "`read()` method returns [sysuniv::R](sysuniv::R) reader structure"]
impl crate::Readable for SYSUNIV {}
#[doc = "`write(|w| ..)` method takes [sysuniv::W](sysuniv::W) writer structure"]
impl crate::Writable for SYSUNIV {}
#[doc = "User NMI vector generator"]
pub mod sysuniv;
#[doc = "System NMI vector generator\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [syssniv](syssniv) module"]
pub type SYSSNIV = crate::Reg<u16, _SYSSNIV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SYSSNIV;
#[doc = "`read()` method returns [syssniv::R](syssniv::R) reader structure"]
impl crate::Readable for SYSSNIV {}
#[doc = "`write(|w| ..)` method takes [syssniv::W](syssniv::W) writer structure"]
impl crate::Writable for SYSSNIV {}
#[doc = "System NMI vector generator"]
pub mod syssniv;
#[doc = "Reset vector generator\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sysrstiv](sysrstiv) module"]
pub type SYSRSTIV = crate::Reg<u16, _SYSRSTIV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SYSRSTIV;
#[doc = "`read()` method returns [sysrstiv::R](sysrstiv::R) reader structure"]
impl crate::Readable for SYSRSTIV {}
#[doc = "`write(|w| ..)` method takes [sysrstiv::W](sysrstiv::W) writer structure"]
impl crate::Writable for SYSRSTIV {}
#[doc = "Reset vector generator"]
pub mod sysrstiv;
