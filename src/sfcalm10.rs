use std::error::Error;
use rppal::gpio::Gpio;
use std::string::String;

const GPIO_SENSOR: u8 = 12;

pub fn alm10 () -> Result<(), Box<dyn Error>>
{
  
 
 let alm_message10 = String::from ("Aviation");
   
 let pin = Gpio::new()?.get(GPIO_SENSOR)?.into_input();
 loop{

 if pin.is_low() == true {

    println!("{}",alm_message10);
  }
  
}}

