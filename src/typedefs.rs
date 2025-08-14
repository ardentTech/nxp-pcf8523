#[derive(Debug, PartialEq)]
pub enum Meridiem {
    AM = 0x0,
    PM = 0x1,
}
// impl TryFrom<u8> for Meridiem {
//     type Error = ();
//     fn try_from(value: u8) -> Result<Self, Self::Error> {
//         match value {
//             0 => Ok(Meridiem::AM),
//             1 => Ok(Meridiem::PM),
//             _ => Err(()),
//         }
//     }
// }

#[derive(Debug, PartialEq)]
pub struct Hours {
    hours: u8,
    meridiem: Option<Meridiem>
}
impl Hours {
    pub fn new(hours: u8, meridiem: Option<Meridiem>) -> Self {
        // if meridiem.is_some() {
        //     // TODO validate < 13
        // }
        Self { hours, meridiem }
    }
}

#[derive(Debug, PartialEq)]
pub enum Pcf8523Error<E> {
    I2C(E),
}

#[derive(Debug, PartialEq)]
pub enum Pcf8523Interrupt {
    Alarm,
    BatterySwitchOver,
    CountdownTimerA,
    CountdownTimerB,
    Second,
    WatchdogTimerA,
}

#[derive(Debug, PartialEq)]
pub enum PowerManagement {
    SwitchOverStandardLowDetectionEnabled = 0x0,
    SwitchOverDirectSwitchingLowDetectionEnabled = 0x1,
    SwitchOverDisabledLowDetectionEnabled = 0x2,
    SwitchOverStandardLowDetectionDisabled = 0x4,
    SwitchOverDirectSwitchingLowDetectionDisabled = 0x5,
    SwitchOverDisabledLowDetectionDisabled = 0x7,
}