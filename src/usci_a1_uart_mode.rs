#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - USCI A1 Control Register 1"]
    pub uca1ctl0: UCA1CTL0,
    #[doc = "0x01 - USCI A1 Control Register 0"]
    pub uca1ctl1: UCA1CTL1,
    #[doc = "0x02 - USCI A1 Control Word Register 1"]
    pub uca1ctlw1: UCA1CTLW1,
    _reserved3: [u8; 2usize],
    #[doc = "0x06 - USCI A1 Baud Rate 0"]
    pub uca1br0: UCA1BR0,
    #[doc = "0x07 - USCI A1 Baud Rate 1"]
    pub uca1br1: UCA1BR1,
    #[doc = "0x08 - USCI A1 Modulation Control"]
    pub uca1mctlw: UCA1MCTLW,
    #[doc = "0x0a - USCI A1 Status Register"]
    pub uca1statw: UCA1STATW,
    _reserved7: [u8; 1usize],
    #[doc = "0x0c - USCI A1 Receive Buffer"]
    pub uca1rxbuf: UCA1RXBUF,
    #[doc = "0x0e - USCI A1 Transmit Buffer"]
    pub uca1txbuf: UCA1TXBUF,
    #[doc = "0x10 - USCI A1 LIN Control"]
    pub uca1abctl: UCA1ABCTL,
    _reserved10: [u8; 1usize],
    #[doc = "0x12 - USCI A1 IrDA Transmit Control"]
    pub uca1irtctl: UCA1IRTCTL,
    #[doc = "0x13 - USCI A1 IrDA Receive Control"]
    pub uca1irrctl: UCA1IRRCTL,
    _reserved12: [u8; 6usize],
    #[doc = "0x1a - USCI A0 Interrupt enable register"]
    pub uca1ie: UCA1IE,
    _reserved13: [u8; 1usize],
    #[doc = "0x1c - Interrupt flag register"]
    pub uca1ifg: UCA1IFG,
    #[doc = "0x1e - USCI A1 Interrupt Vector Register"]
    pub uca1iv: UCA1IV,
}
#[doc = "USCI A1 Control Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uca1ctl1](uca1ctl1) module"]
pub type UCA1CTL1 = crate::Reg<u8, _UCA1CTL1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCA1CTL1;
#[doc = "`read()` method returns [uca1ctl1::R](uca1ctl1::R) reader structure"]
impl crate::Readable for UCA1CTL1 {}
#[doc = "`write(|w| ..)` method takes [uca1ctl1::W](uca1ctl1::W) writer structure"]
impl crate::Writable for UCA1CTL1 {}
#[doc = "USCI A1 Control Register 1"]
pub mod uca1ctl1;
#[doc = "USCI A1 Control Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uca1ctl0](uca1ctl0) module"]
pub type UCA1CTL0 = crate::Reg<u8, _UCA1CTL0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCA1CTL0;
#[doc = "`read()` method returns [uca1ctl0::R](uca1ctl0::R) reader structure"]
impl crate::Readable for UCA1CTL0 {}
#[doc = "`write(|w| ..)` method takes [uca1ctl0::W](uca1ctl0::W) writer structure"]
impl crate::Writable for UCA1CTL0 {}
#[doc = "USCI A1 Control Register 0"]
pub mod uca1ctl0;
#[doc = "USCI A1 Baud Rate 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uca1br0](uca1br0) module"]
pub type UCA1BR0 = crate::Reg<u8, _UCA1BR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCA1BR0;
#[doc = "`read()` method returns [uca1br0::R](uca1br0::R) reader structure"]
impl crate::Readable for UCA1BR0 {}
#[doc = "`write(|w| ..)` method takes [uca1br0::W](uca1br0::W) writer structure"]
impl crate::Writable for UCA1BR0 {}
#[doc = "USCI A1 Baud Rate 0"]
pub mod uca1br0;
#[doc = "USCI A1 Baud Rate 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uca1br1](uca1br1) module"]
pub type UCA1BR1 = crate::Reg<u8, _UCA1BR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCA1BR1;
#[doc = "`read()` method returns [uca1br1::R](uca1br1::R) reader structure"]
impl crate::Readable for UCA1BR1 {}
#[doc = "`write(|w| ..)` method takes [uca1br1::W](uca1br1::W) writer structure"]
impl crate::Writable for UCA1BR1 {}
#[doc = "USCI A1 Baud Rate 1"]
pub mod uca1br1;
#[doc = "USCI A1 Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uca1statw](uca1statw) module"]
pub type UCA1STATW = crate::Reg<u8, _UCA1STATW>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCA1STATW;
#[doc = "`read()` method returns [uca1statw::R](uca1statw::R) reader structure"]
impl crate::Readable for UCA1STATW {}
#[doc = "`write(|w| ..)` method takes [uca1statw::W](uca1statw::W) writer structure"]
impl crate::Writable for UCA1STATW {}
#[doc = "USCI A1 Status Register"]
pub mod uca1statw;
#[doc = "USCI A1 LIN Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uca1abctl](uca1abctl) module"]
pub type UCA1ABCTL = crate::Reg<u8, _UCA1ABCTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCA1ABCTL;
#[doc = "`read()` method returns [uca1abctl::R](uca1abctl::R) reader structure"]
impl crate::Readable for UCA1ABCTL {}
#[doc = "`write(|w| ..)` method takes [uca1abctl::W](uca1abctl::W) writer structure"]
impl crate::Writable for UCA1ABCTL {}
#[doc = "USCI A1 LIN Control"]
pub mod uca1abctl;
#[doc = "USCI A1 IrDA Transmit Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uca1irtctl](uca1irtctl) module"]
pub type UCA1IRTCTL = crate::Reg<u8, _UCA1IRTCTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCA1IRTCTL;
#[doc = "`read()` method returns [uca1irtctl::R](uca1irtctl::R) reader structure"]
impl crate::Readable for UCA1IRTCTL {}
#[doc = "`write(|w| ..)` method takes [uca1irtctl::W](uca1irtctl::W) writer structure"]
impl crate::Writable for UCA1IRTCTL {}
#[doc = "USCI A1 IrDA Transmit Control"]
pub mod uca1irtctl;
#[doc = "USCI A1 IrDA Receive Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uca1irrctl](uca1irrctl) module"]
pub type UCA1IRRCTL = crate::Reg<u8, _UCA1IRRCTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCA1IRRCTL;
#[doc = "`read()` method returns [uca1irrctl::R](uca1irrctl::R) reader structure"]
impl crate::Readable for UCA1IRRCTL {}
#[doc = "`write(|w| ..)` method takes [uca1irrctl::W](uca1irrctl::W) writer structure"]
impl crate::Writable for UCA1IRRCTL {}
#[doc = "USCI A1 IrDA Receive Control"]
pub mod uca1irrctl;
#[doc = "USCI A1 Control Word Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uca1ctlw1](uca1ctlw1) module"]
pub type UCA1CTLW1 = crate::Reg<u16, _UCA1CTLW1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCA1CTLW1;
#[doc = "`read()` method returns [uca1ctlw1::R](uca1ctlw1::R) reader structure"]
impl crate::Readable for UCA1CTLW1 {}
#[doc = "`write(|w| ..)` method takes [uca1ctlw1::W](uca1ctlw1::W) writer structure"]
impl crate::Writable for UCA1CTLW1 {}
#[doc = "USCI A1 Control Word Register 1"]
pub mod uca1ctlw1;
#[doc = "USCI A1 Modulation Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uca1mctlw](uca1mctlw) module"]
pub type UCA1MCTLW = crate::Reg<u16, _UCA1MCTLW>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCA1MCTLW;
#[doc = "`read()` method returns [uca1mctlw::R](uca1mctlw::R) reader structure"]
impl crate::Readable for UCA1MCTLW {}
#[doc = "`write(|w| ..)` method takes [uca1mctlw::W](uca1mctlw::W) writer structure"]
impl crate::Writable for UCA1MCTLW {}
#[doc = "USCI A1 Modulation Control"]
pub mod uca1mctlw;
#[doc = "USCI A1 Receive Buffer\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uca1rxbuf](uca1rxbuf) module"]
pub type UCA1RXBUF = crate::Reg<u16, _UCA1RXBUF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCA1RXBUF;
#[doc = "`read()` method returns [uca1rxbuf::R](uca1rxbuf::R) reader structure"]
impl crate::Readable for UCA1RXBUF {}
#[doc = "`write(|w| ..)` method takes [uca1rxbuf::W](uca1rxbuf::W) writer structure"]
impl crate::Writable for UCA1RXBUF {}
#[doc = "USCI A1 Receive Buffer"]
pub mod uca1rxbuf;
#[doc = "USCI A1 Transmit Buffer\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uca1txbuf](uca1txbuf) module"]
pub type UCA1TXBUF = crate::Reg<u16, _UCA1TXBUF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCA1TXBUF;
#[doc = "`read()` method returns [uca1txbuf::R](uca1txbuf::R) reader structure"]
impl crate::Readable for UCA1TXBUF {}
#[doc = "`write(|w| ..)` method takes [uca1txbuf::W](uca1txbuf::W) writer structure"]
impl crate::Writable for UCA1TXBUF {}
#[doc = "USCI A1 Transmit Buffer"]
pub mod uca1txbuf;
#[doc = "USCI A1 Interrupt Vector Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uca1iv](uca1iv) module"]
pub type UCA1IV = crate::Reg<u16, _UCA1IV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCA1IV;
#[doc = "`read()` method returns [uca1iv::R](uca1iv::R) reader structure"]
impl crate::Readable for UCA1IV {}
#[doc = "`write(|w| ..)` method takes [uca1iv::W](uca1iv::W) writer structure"]
impl crate::Writable for UCA1IV {}
#[doc = "USCI A1 Interrupt Vector Register"]
pub mod uca1iv;
#[doc = "USCI A0 Interrupt enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uca1ie](uca1ie) module"]
pub type UCA1IE = crate::Reg<u8, _UCA1IE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCA1IE;
#[doc = "`read()` method returns [uca1ie::R](uca1ie::R) reader structure"]
impl crate::Readable for UCA1IE {}
#[doc = "`write(|w| ..)` method takes [uca1ie::W](uca1ie::W) writer structure"]
impl crate::Writable for UCA1IE {}
#[doc = "USCI A0 Interrupt enable register"]
pub mod uca1ie;
#[doc = "Interrupt flag register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uca1ifg](uca1ifg) module"]
pub type UCA1IFG = crate::Reg<u16, _UCA1IFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCA1IFG;
#[doc = "`read()` method returns [uca1ifg::R](uca1ifg::R) reader structure"]
impl crate::Readable for UCA1IFG {}
#[doc = "`write(|w| ..)` method takes [uca1ifg::W](uca1ifg::W) writer structure"]
impl crate::Writable for UCA1IFG {}
#[doc = "Interrupt flag register"]
pub mod uca1ifg;
