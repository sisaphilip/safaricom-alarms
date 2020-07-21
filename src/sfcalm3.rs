use std::error::Error;
use rppal::gpio::Gpio;
use std::string::String;


const GPIO_SENSOR: u8 =27;

pub fn alm3 () -> Result<(), Box<dyn Error>>
{
  
 
 let alm_message3 = String::from ("Gen running");
   
 let pin = Gpio::new()?.get(GPIO_SENSOR)?.into_input();
 loop{

 if pin.is_low() == true {

    println!("{}",alm_message3);
  }
  
}}

