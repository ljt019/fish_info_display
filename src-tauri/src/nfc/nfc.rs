use crate::simple_timer::SimpleTimer;
use pn532::{Pn532, Request, Response};
use rppal::i2c::I2c;
use std::error::Error;
use std::time::Duration;
use tauri::Manager;

pub struct NfcReader {
    pn532: Pn532<I2c>,
}

impl NfcReader {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        let i2c = I2c::new()?;
        let timer = SimpleTimer::new();
        let mut pn532 = Pn532::new(i2c, timer);
        pn532.process(
            &Request::sam_configuration(pn532::requests::SAMMode::Normal, false),
            100,
        )?;
        Ok(NfcReader { pn532 })
    }

    pub fn start_reading<F>(&mut self, mut callback: F) -> Result<(), Box<dyn Error>>
    where
        F: FnMut(Vec<u8>),
    {
        loop {
            match self.pn532.process(
                &Request::InListPassiveTarget {
                    max_targets: 1,
                    baud_rate: 106,
                },
                1000,
            ) {
                Ok(Response::InListPassiveTarget { targets }) if !targets.is_empty() => {
                    let uid = targets[0].identifier.clone();
                    callback(uid);
                }
                _ => (),
            }
            std::thread::sleep(Duration::from_millis(500));
        }
    }
}
