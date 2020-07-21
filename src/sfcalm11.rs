use std::error::Error;
use rppal::gpio::Gpio;

use std::string::String;


const GPIO_SENSOR: u8 = 13;

pub fn alm11 () -> Result<(), Box<dyn Error>>
{
  
 
 let alm_message11 = String::from ("High Temperature");
   
 let pin = Gpio::new()?.get(GPIO_SENSOR)?.into_input();
 loop{

 if pin.is_low() == true {

    println!("{}",alm_message11);
  }
  
}}

