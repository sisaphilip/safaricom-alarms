use std::error::Error;
use rppal::gpio::Gpio;
use std::string::String;


const GPIO_SENSOR: u8 = 24;

pub fn alm6 () -> Result<(), Box<dyn Error>>
{
  
 
 let alm_message6 = String::from ("Rectifier Fail");
   
 let pin = Gpio::new()?.get(GPIO_SENSOR)?.into_input();
 loop{

 if pin.is_low() == true {

    println!("{}",alm_message6);
  }
  
}}

