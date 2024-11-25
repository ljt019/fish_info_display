use pn532::requests::SAMMode;
use pn532::spi::SPIInterface;
use pn532::{Pn532, Request};
use rppal::gpio::{Gpio, OutputPin};
use rppal::hal::Timer;
use rppal::spi::Spi;
use std::error::Error;
use std::time::Duration;

pub struct NfcReader {
    pn532: Pn532<SPIInterface<Spi, OutputPin>, Timer>,
}

impl NfcReader {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        let spi = Spi::new(
            rppal::spi::Bus::Spi0,
            rppal::spi::SlaveSelect::Ss0,
            1000000,
            rppal::spi::Mode::Mode0,
        )?;

        let gpio = Gpio::new()?;
        let cs_pin = gpio.get(8)?.into_output();

        let interface = SPIInterface {
            spi: spi,
            cs: cs_pin,
        };

        let timer = Timer::new();

        let mut pn532: Pn532<_, _, 32> = Pn532::new(interface, timer);

        pn532.process(
            &Request::sam_configuration(SAMMode::Normal, false),
            0,
            Duration::from_millis(50),
        );

        Ok(NfcReader { pn532 })
    }

    pub fn start_reading<F>(&mut self, mut callback: F) -> Result<(), Box<dyn Error>>
    where
        F: FnMut(Vec<u8>),
    {
        loop {
            // Send the InListPassiveTarget request
            match self.pn532.process(
                &Request::INLIST_ONE_ISO_A_TARGET,
                16, // Adjust the response length as needed
                Duration::from_secs(1),
            ) {
                Ok(response_data) => {
                    // Parse the response data to extract the UID
                    if let Some(uid) = parse_in_list_passive_target_response(response_data) {
                        callback(uid);
                    }
                }
                Err(e) => {
                    eprintln!("Error during PN532 communication: {:?}", e);
                }
            }
            std::thread::sleep(Duration::from_millis(500));
        }
    }
}

// Function to parse the response data and extract the UID
fn parse_in_list_passive_target_response(data: &[u8]) -> Option<Vec<u8>> {
    // Ensure that the data length is at least the minimum expected
    if data.len() < 7 {
        // The minimum length for one target is 7 bytes
        return None;
    }

    let number_of_tags = data[0];
    if number_of_tags == 0 {
        // No tags found
        return None;
    }

    // Now, parse the target data
    let mut index = 1; // Start after number_of_tags

    // Target Number (1 byte)
    let _target_number = data[index];
    index += 1;

    // SENS_RES (2 bytes)
    let _sens_res = &data[index..index + 2];
    index += 2;

    // SEL_RES (1 byte)
    let _sel_res = data[index];
    index += 1;

    // NFCID Length (1 byte)
    let uid_length = data[index] as usize;
    index += 1;

    // NFCID (UID) (uid_length bytes)
    if data.len() < index + uid_length {
        // Not enough data for UID
        return None;
    }
    let uid = data[index..index + uid_length].to_vec();

    Some(uid)
}
