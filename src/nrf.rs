use nrf24_rs::{config::{DataRate, NrfConfig, PALevel, PayloadSize}, Nrf24l01};
use teensy4_bsp::hal;

const CE_PIN = 26;
const CSN_PIN = 27;
const INTERVAL_MS_SIGNAL_LOST = 1000;
const INTERVAL_MS_SIGNAL_RETRY = 250;

//unsigned long lastSignalMillis = 0;

#[derive(Debug, Error)]
pub enum Error {
}

#[derive(Clone, Copy, Debug, Default)]
pub struct RcDataPackage {
    pub joyLeft_X: u8,
    pub joyLeft_Y: u8,
    pub joyRight_X: u8,
    pub joyRight_Y: u8,
    pub potLeft: u8,
    pub potRight: u8,
    pub buttons_1: u8,
    pub buttons_2: u8,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct AckDataPackage {
    status: u8,
}

impl RcDataPackage {
    /// Test if the A button is pressed
    pub fn button_a(&self) -> bool {
        (self.buttons_1 & 0b00000001) != 0
    }

    /// Test if the B button is pressed
    pub fn button_b(&self) -> bool {
        (self.buttons_1 & 0b00000010) != 0
    }
    
    /// Test if the C button is pressed
    pub fn button_c(&self) -> bool {
        (self.buttons_1 & 0b00000100) != 0
    }

    /// Test if the D button is pressed
    pub fn button_d(&self) -> bool {
        (self.buttons_1 & 0b00001000) != 0
    }

    /// Test if the A toggle is set 
    pub fn toggle_a(&self) -> bool {
        (self.buttons_1 & 0b00010000) != 0
    }

    /// Test if the B toggle is set 
    pub fn toggle_b(&self) -> bool {
        (self.buttons_1 & 0b00100000) != 0
    }

    /// Test if the C toggle is set 
    pub fn toggle_c(&self) -> bool {
        (self.buttons_1 & 0b01000000) != 0
    }

    /// Test if the D toggle is set 
    pub fn toggle_d(&self) -> bool {
        (self.buttons_1 & 0b10000000) != 0
    }

    /// Test if the A bumper is pressed
    pub fn bumper_a(&self) -> bool {
        (self.buttons_2 & 0b00000001) != 0
    }

    /// Test if the B bumper is pressed
    pub fn bumper_b(&self) -> bool {
        (self.buttons_2 & 0b00000010) != 0
    }
    
    /// Test if the C bumper is pressed
    pub fn bumper_c(&self) -> bool {
        (self.buttons_2 & 0b00000100) != 0
    }

    /// Test if the D bumper is pressed
    pub fn bumper_d(&self) -> bool {
        (self.buttons_2 & 0b00001000) != 0
    }

    /// Test if the joystick left button is pressed
    pub fn joy_left_button(&self) -> bool {
        (self.buttons_2 & 0b00010000) != 0
    }

    /// Test if the joystick right button is pressed
    pub fn joy_right_button(&self) -> bool {
        (self.buttons_2 & 0b00100000) != 0
    }
}

impl AckDataPackage {
    /// Controller is connected
    pub fn connected(&self) -> bool {
        (self.status & 0b00000001) != 0
    }

    /// Controller is connected
    pub fn set_connected(&mut self, value: bool) {
        if value {
            self.status |= 0b00000001; // Set the bit
        } else {
            self.status &= !0b00000001; // Clear the bit
        }
    }
}

pub struct Nrf<SPI, CE> {
    address: [u8; 6],
    rc_data: RcDataPackage,
    ack_data: AckDataPackage,
    radio: Nrf24l01<SPI, CE>,
}

impl<SPI, CE> Nrf {
    pub fn new() -> Result<Nrf<SPI, CE>, Error> {
        let address = "HEX01";
        let rc_data = RcDataPackage::default(),
        let mut ack_data = RcDataPackage::default(),
        ack_data.set_connected(true);

        let gpio = hal::gpio`

        // Setup some configuration values using the builder pattern
        let config = NrfConfig::default()
            .ack_payloads_enabled(true)
            .channel(8)
            .data_rate(DataRate::R1Mbps)
            .pa_level(PALevel::Min)
            .payload_size(PayloadSize::Dynamic);

        let mut delay = setup_delay(); // create new delay
        let mut radio = Nrf24l01::new(spi, ce, &mut delay, config).unwrap();
        if !radio.is_connected().unwrap() {
            panic!("Chip is not connected.");
        }

        radio.start_listening()?;

        Ok(Nrf { address, rc_data, ack_data, radio })
    }
}

/*
void receiveNRFData()
{
    radio.writeAckPayload(1, &ack_data, sizeof(ack_data));

    unsigned long currentMillis = millis();
    if (radio.available() > 0)
    {
        radio.read(&rc_data, sizeof(rc_data));
        lastSignalMillis = currentMillis;
    }

    // RC_DisplayData();

    if (currentMillis - lastSignalMillis > INTERVAL_MS_SIGNAL_LOST)
    {
        Serial.println("We have lost connection, preventing unwanted behavior");
        delay(INTERVAL_MS_SIGNAL_RETRY);
    }
}

void RC_DisplayData()
{
    Serial.println("JLX: " + String(rc_data.joyLeft_X) +
                   "  JLY: " + String(rc_data.joyLeft_Y) +
                   "  JRX: " + String(rc_data.joyRight_X) +
                   "  JRY: " + String(rc_data.joyRight_Y) +
                   "  JLB: " + String(rc_data.joyLeft_Button) +
                   "  JRB: " + String(rc_data.joyRight_Button) +
                   "  PL: " + String(rc_data.potLeft) +
                   "  PR: " + String(rc_data.potRight) +
                   "  BA: " + String(rc_data.button_A) +
                   "  BB: " + String(rc_data.button_B) +
                   "  BC: " + String(rc_data.button_C) +
                   "  BD: " + String(rc_data.button_D) +
                   "  TA: " + String(rc_data.toggle_A) +
                   "  TB: " + String(rc_data.toggle_B) +
                   "  TC: " + String(rc_data.toggle_C) +
                   "  TD: " + String(rc_data.toggle_D) +
                   "  BA: " + String(rc_data.bumper_A) +
                   "  BB: " + String(rc_data.bumper_B) +
                   "  BC: " + String(rc_data.bumper_C) +
                   "  BD: " + String(rc_data.bumper_D));
}
*/
