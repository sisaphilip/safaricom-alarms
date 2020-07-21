use std::error::Error;
use rppal::gpio::Gpio;

use std::string::String;


const GPIO_SENSOR: u8 = 4;

 pub fn alm0 () -> Result<(), Box<dyn Error>> {
  
 
 let alm_message0 = String::from("Smoke in service rack");
   
 let pin = Gpio::new()?.get(GPIO_SENSOR)?.into_input();
 loop{

 if pin.is_low() == true {

    println!("{}",alm_message0);
  }
  
}}

