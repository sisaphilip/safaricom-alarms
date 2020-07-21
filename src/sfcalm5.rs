use std::error::Error;
use rppal::gpio::Gpio;
use std::string::String;


const GPIO_SENSOR: u8 = 23;

pub fn alm5 () -> Result<(), Box<dyn Error>>
{
  
 
 let alm_message5 = String::from ("Battery Providing");
   
 let pin = Gpio::new()?.get(GPIO_SENSOR)?.into_input();
 loop{

 if pin.is_low() == true {

    println!("{}",alm_message5);
  }
  
}}

