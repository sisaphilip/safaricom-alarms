use std::error::Error;
use rppal::gpio::Gpio;
use std::string::String;


const GPIO_SENSOR: u8 =25;

pub fn alm7 () -> Result<(), Box<dyn Error>>
{
  
 
 let alm_message7 = String::from("Gen Service");
   
 let pin = Gpio::new()?.get(GPIO_SENSOR)?.into_input();
 loop{

 if pin.is_low() == true {

    println!("{}",alm_message7);
  }
  
}}

