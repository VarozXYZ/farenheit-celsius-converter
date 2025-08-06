fn main() {
    let temp1: f32 = 34.0;
    println!("Converting {temp1} º to Farenheit...");
    let temp1 = ctof(temp1);
    println!("The converted amount is {temp1:.2} ºF");

    let temp2: f32 = 128.0;
    println!("Converting {temp2} F to Celsius...");
    let temp2 = ftoc(temp2);
    println!("The converted amount is {temp2:.2} ºC");
}

fn ftoc(temp: f32) -> f32 {
    (temp - 32.0) * 5.0 / 9.0
}

fn ctof(temp: f32) -> f32 {
    (temp * 9.0 / 5.0) + 32.0
}
