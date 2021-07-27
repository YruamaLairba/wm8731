//! Configuration for headphone outputs

use crate::EnableDisable;

#[derive(Debug, Copy, Clone)]
pub struct HeadphoneOut {
    pub(crate) data: u16,
}

impl HeadphoneOut {
    pub fn new() -> Self {
        HeadphoneOut {
            data: 0b0_0000_0000,
        }
    }

    /// Set headphone output volume control with raw bits
    ///
    /// Min: -73dB
    ///
    /// Max: +6dB
    ///
    /// Step: 1dB
    pub fn volume(&mut self, volume: u8) {
        self.data = self.data & !0b111_1111| (volume & 0b111_1111) as u16;
    }

    /// Zero cross detect
    pub fn zero_cross_detect(&mut self) -> EnableDisable {
        EnableDisable::new(7, &mut self.data)
    }

    /// Left to right channel headphone volume, mute, and zero cross data load
    /// When enabled, left and right channels will have the same values
    pub fn both(&mut self) -> EnableDisable {
        EnableDisable::new(8, &mut self.data)
    }
}
