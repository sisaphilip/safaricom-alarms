use std::error::Error;
use rppal::gpio::Gpio;
use std::string::String;


const GPIO_SENSOR: u8 = 18;
pub fn alm2 () -> Result<(), Box<dyn Error>>
{
 
 let alm_message2 = String::from ("G.M.F");
   
 let pin = Gpio::new()?.get(GPIO_SENSOR)?.into_input();
 loop{

 if pin.is_low() == true {

    println!("{}",alm_message2);
  }
  
}}

