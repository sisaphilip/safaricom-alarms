use std::error::Error;
use rppal::gpio::Gpio;
use std::string::String;


const GPIO_SENSOR: u8 = 5;

pub fn alm8 () ->Result<(), Box<dyn Error>>
{
  
 
 let alm_message8 = String::from ("Fence Alarm");
   
 let pin = Gpio::new()?.get(GPIO_SENSOR)?.into_input();
 loop{

 if pin.is_low() == true {

    println!("{}",alm_message8);
  }
  
}}

