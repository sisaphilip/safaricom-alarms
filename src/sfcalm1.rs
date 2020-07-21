use rppal::gpio::Gpio;
use std::string::String;
use std::error::Error;


const GPIO_SENSOR: u8 = 17;
pub fn alm1 () ->Result<(), Box<dyn Error>>
{
  
 
 let alm_message1 = String::from ("Mains Fail");

 let  pin= Gpio::new()?.get(GPIO_SENSOR)?.into_input();
 

 loop{

 if pin.is_low() == true {

    println!("{}",alm_message1);
  }
  
}}

