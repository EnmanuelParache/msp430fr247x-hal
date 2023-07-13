use super::Steal;
use msp430fr247x as pac;

pub enum Tbssel {
    Tbxclk,
    Aclk,
    Smclk,
    Inclk,
}

/// Timer clock divider
pub enum TimerDiv {
    /// No division
    _1,
    /// Divide by 2
    _2,
    /// Divide by 4
    _4,
    /// Divide by 8
    _8,
}

/// Timer expansion clock divider, applied on top of the normal clock divider
pub enum TimerExDiv {
    /// No division
    _1,
    /// Divide by 2
    _2,
    /// Divide by 3
    _3,
    /// Divide by 4
    _4,
    /// Divide by 5
    _5,
    /// Divide by 6
    _6,
    /// Divide by 7
    _7,
    /// Divide by 8
    _8,
}

pub enum Outmod {
    Out,
    Set,
    ToggleReset,
    SetReset,
    Toggle,
    Reset,
    ToggleSet,
    ResetSet,
}

pub enum Cm {
    NoCap,
    RisingEdge,
    FallingEdge,
    BothEdges,
}

pub enum Ccis {
    InputA,
    InputB,
    Gnd,
    Vcc,
}

pub trait TimerB: Steal {
    /// Reset timer countdown
    fn reset(&self);

    /// Set to upmode, reset timer, and clear interrupts
    fn upmode(&self);
    /// Set to continuous mode, reset timer, and clear interrupts
    fn continuous(&self);

    /// Apply clock select settings
    fn config_clock(&self, tbssel: Tbssel, div: TimerDiv);

    /// Check if timer is stopped
    fn is_stopped(&self) -> bool;

    /// Stop timer
    fn stop(&self);

    /// Set expansion register clock divider settings
    fn set_tbidex(&self, tbidex: TimerExDiv);

    fn tbifg_rd(&self) -> bool;
    fn tbifg_clr(&self);

    fn tbie_set(&self);
    fn tbie_clr(&self);

    fn tbxiv_rd(&self) -> u16;
}

pub trait CCRn<C>: Steal {
    fn set_ccrn(&self, count: u16);
    fn get_ccrn(&self) -> u16;

    fn config_outmod(&self, outmod: Outmod);
    fn config_cap_mode(&self, cm: Cm, ccis: Ccis);

    fn ccifg_rd(&self) -> bool;
    fn ccifg_clr(&self);

    fn ccie_set(&self);
    fn ccie_clr(&self);

    fn cov_ccifg_rd(&self) -> (bool, bool);
    fn cov_ccifg_clr(&self);
}

/// Label for capture-compare register 0
pub struct CCR0;
/// Label for capture-compare register 1
pub struct CCR1;
/// Label for capture-compare register 2
pub struct CCR2;
/// Label for capture-compare register 3
pub struct CCR3;
/// Label for capture-compare register 4
pub struct CCR4;
/// Label for capture-compare register 5
pub struct CCR5;
/// Label for capture-compare register 6
pub struct CCR6;

macro_rules! ccrn_impl {
    ($TBx:ident, $CCRn:ident, $tbxcctln:ident, $tbxccrn:ident) => {
        impl CCRn<$CCRn> for pac::$TBx {
            #[inline(always)]
            fn set_ccrn(&self, count: u16) {
                self.$tbxccrn.write(|w| unsafe { w.bits(count) });
            }

            #[inline(always)]
            fn get_ccrn(&self) -> u16 {
                self.$tbxccrn.read().bits()
            }

            #[inline(always)]
            fn config_outmod(&self, outmod: Outmod) {
                self.$tbxcctln.write(|w| w.outmod().bits(outmod as u8));
            }

            #[inline(always)]
            fn config_cap_mode(&self, cm: Cm, ccis: Ccis) {
                self.$tbxcctln.write(|w| {
                    w.cap()
                        .capture()
                        .scs()
                        .sync()
                        .cm()
                        .bits(cm as u8)
                        .ccis()
                        .bits(ccis as u8)
                });
            }

            #[inline(always)]
            fn ccifg_rd(&self) -> bool {
                self.$tbxcctln.read().ccifg().bit()
            }

            #[inline(always)]
            fn ccifg_clr(&self) {
                self.$tbxcctln.write(|w| w.ccifg().clear_bit());
            }

            #[inline(always)]
            fn ccie_set(&self) {
                self.$tbxcctln.write(|w| w.ccie().set_bit());
            }

            #[inline(always)]
            fn ccie_clr(&self) {
                self.$tbxcctln.write(|w| w.ccie().clear_bit());
            }

            #[inline(always)]
            fn cov_ccifg_rd(&self) -> (bool, bool) {
                let cctl = self.$tbxcctln.read();
                (cctl.cov().bit(), cctl.ccifg().bit())
            }

            #[inline(always)]
            fn cov_ccifg_clr(&self) {
                    self.$tbxcctln
                        .write(|w| w.ccifg().clear_bit().cov().clear_bit());
            }
        }
    };
}

macro_rules! timerb_impl {
    ($TBx:ident, $tbx:ident, $tbxctl:ident, $tbxex:ident, $tbxiv:ident, $([$CCRn:ident, $tbxcctln:ident, $tbxccrn:ident]),*) => {
        impl Steal for pac::$TBx {
            #[inline(always)]
            unsafe fn steal() -> Self {
                pac::Peripherals::steal().$TBx
            }
        }

        impl TimerB for pac::$TBx {
            #[inline(always)]
            fn reset(&self) {
                self.$tbxctl.write(|w| w.tbclr().set_bit());
            }

            #[inline(always)]
            fn upmode(&self) {
                self.$tbxctl.modify(|r, w| {
                    unsafe { w.bits(r.bits()) }
                        .tbclr()
                        .set_bit()
                        .tbifg()
                        .clear_bit()
                        .mc()
                        .up()
                });
            }

            #[inline(always)]
            fn continuous(&self) {
                self.$tbxctl.modify(|r, w| {
                    unsafe { w.bits(r.bits()) }
                        .tbclr()
                        .set_bit()
                        .tbifg()
                        .clear_bit()
                        .mc()
                        .continuous()
                });
            }

            #[inline(always)]
            fn config_clock(&self, tbssel: Tbssel, div: TimerDiv) {
                self.$tbxctl
                    .write(|w| w.tbssel().bits(tbssel as u8).id().bits(div as u8));
            }

            #[inline(always)]
            fn is_stopped(&self) -> bool {
                self.$tbxctl.read().mc().is_stop()
            }

            #[inline(always)]
            fn stop(&self) {
                self.$tbxctl.write(|w| w.mc().stop());
            }

            #[inline(always)]
            fn set_tbidex(&self, tbidex: TimerExDiv) {
                self.$tbxex.write(|w| w.tbidex().bits(tbidex as u8));
            }

            #[inline(always)]
            fn tbifg_rd(&self) -> bool {
                self.$tbxctl.read().tbifg().bit()
            }

            #[inline(always)]
            fn tbifg_clr(&self) {
                self.$tbxctl.write(|w| w.tbifg().clear_bit());
            }

            #[inline(always)]
            fn tbie_set(&self) {
                self.$tbxctl.write(|w| w.tbie().set_bit());
            }

            #[inline(always)]
            fn tbie_clr(&self) {
                self.$tbxctl.write(|w| w.tbie().clear_bit());
            }

            #[inline(always)]
            fn tbxiv_rd(&self) -> u16 {
                self.$tbxiv.read().bits()
            }
        }

        $(ccrn_impl!($TBx, $CCRn, $tbxcctln, $tbxccrn);)*
    };
}

timerb_impl!(
    TB0,
    tb0,
    tb0ctl,
    tb0ex0,
    tb0iv,
    [CCR0, tb0cctl0, tb0ccr0],
    [CCR1, tb0cctl1, tb0ccr1],
    [CCR2, tb0cctl2, tb0ccr2],
    [CCR3, tb0cctl3, tb0ccr3],
    [CCR4, tb0cctl4, tb0ccr4],
    [CCR5, tb0cctl5, tb0ccr5],
    [CCR6, tb0cctl6, tb0ccr6]
);
