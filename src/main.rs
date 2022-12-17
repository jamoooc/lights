use std::time::Duration;
use std::thread::sleep;
use std::error::Error;
use gpio_cdev::{Chip, LineHandle, LineRequestFlags};

const L1: u32 = 13; // GPIO13 PIN 33
const L2: u32 = 19; // GPIO19 PIN 35
const L3: u32 = 26; // GPIO26 PIN 37
const L4: u32 = 16; // GPIO16 PIN 36
const L5: u32 = 20; // GPIO20 PIN 38
const L6: u32 = 21; // GPIO21 PIN 40

fn main() -> Result<(), Box<dyn Error>>{

  // get the gpio adapater
  let mut chip = match Chip::new("/dev/gpiochip0") {
    Ok(c) => c,
    Err(e) => panic!("Oh no! No chip found!{:#?}", e)
  };
  println!("chip: {:#?}", chip);

  let light_pins = vec![ L1, L2, L3, L4, L5, L6 ];

  let mut lights = vec![];

  for (i, pin) in light_pins.iter().enumerate() {
    let str = format!("light{}", i.to_string());
    // Request a line to GPIO pins from the kernel. The default
    // value is set to 0 so all lines are off on initialisation.
    let light = chip
      .get_line(*pin)?
      .request(LineRequestFlags::OUTPUT, 0, &str)?;
    lights.push(light);
  }

  loop {
    match sequential_trigger(&lights) {
      Ok(c) => c,
      Err(e) => println!("Error running sequential trigger: {:#?}", e)
    }
    match split_trigger(&lights) {
      Ok(c) => c,
      Err(e) => println!("Error running sequential trigger: {:#?}", e)
    }
  }
}

fn clear_pins(lights: &Vec<LineHandle>) -> Result<(), gpio_cdev::Error>{
  for (i, light) in lights.iter().enumerate() {
    println!("Clearing: L{i}");
    light.set_value(0)?;
  }
  Ok(())
}

fn sequential_trigger(lights: &Vec<LineHandle>) -> Result<(), gpio_cdev::Error>{
  println!("Sequential trigger");
  for (i, light) in lights.iter().enumerate() {
    light.set_value(1)?;
    sleep(Duration::from_millis(200));
    println!("L{i}: ON");

    light.set_value(0)?;
    sleep(Duration::from_millis(200));
    println!("L{i}: OFF");

fn split_trigger(lights: &Vec<LineHandle>) -> Result<(), gpio_cdev::Error>{
  println!("Split trigger");
  for _ in 0..6 {
    for (i, light) in lights.iter().enumerate() {
      if i % 2 == 0 {
        light.set_value(1)?;
        println!("L{i}: ON");
      } else {
        light.set_value(0)?;
        println!("L{i}: OFF");
      }
    }
    sleep(Duration::from_millis(500));
    for (i, light) in lights.iter().enumerate() {
      if i % 2 != 0 {
        light.set_value(1)?;
        println!("L{i}: ON");
      } else {
        light.set_value(0)?;
        println!("L{i}: OFF");
      }
    }
    sleep(Duration::from_millis(500));
  }
  clear_pins(lights).unwrap();
  Ok(())
}
