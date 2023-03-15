use rust_gpiozero::*;

trait TemperatureSensor {
    fn get_temp(&mut self) -> f32;
}

trait HumiditySensor {
    fn get_humidity(&mut self) -> f32;
}

trait LightController {
    fn turn_on_lights(&mut self);
    fn turn_off_lights(&mut self);
}

trait FanController {
    fn set_fan_speed(&mut self, percent: u8);
}

fn main() {
    println!("Hello, world!");
    let mut led = LED::new(17);
    led.set_blink_count(10);
    led.blink(2.0, 3.0);
}
