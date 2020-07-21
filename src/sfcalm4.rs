use std::error::Error;
use rppal::gpio::Gpio;
use std::string::String;


const GPIO_SENSOR: u8 = 22;

pub fn alm4 () -> Result<(), Box<dyn Error>>
{
  
 
 let alm_message4 = String::from ("Low Fuel");
   
 let pin = Gpio::new()?.get(GPIO_SENSOR)?.into_input();
 loop{

 if pin.is_low() == true {

    println!("{}",alm_message4);
  }
  
}}

