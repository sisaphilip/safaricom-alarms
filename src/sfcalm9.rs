use std::error::Error;
use rppal::gpio::Gpio;
use std::string::String;


const GPIO_SENSOR: u8 = 6;

pub fn alm9()-> Result<(), Box<dyn Error>>
{
   
 let alm_message9 = String::from ("Gate Open");
 let  pin = Gpio::new()?.get(GPIO_SENSOR)?.into_input();
 loop{

 if pin.is_low() == true {

    println!("{}",alm_message9);
  }
  
}}

