fn main() {
    let temp_in_c: f32 = 34.0;
    println!("Converting {temp_in_c} º to Farenheit...");
    let conv_temp_in_f = ctof(temp_in_c);
    println!("The converted amount is {conv_temp_in_f}ºC");

    let temp_in_f: f32 = 128.0;
    println!("Converting {temp_in_f} F to Celsius...");
    let conv_temp_in_c = ftoc(temp_in_f);
    println!("The converted amount is {conv_temp_in_c} F");
}

fn ftoc(temp: f32) -> f32 {
    let temp_in_f = (temp - 32.0) * 5.0 / 9.0;
    return temp_in_f;
}

fn ctof(temp: f32) -> f32 {
    let temp_in_c = (temp * 9.0 / 5.0) + 32.0;
    return temp_in_c;
}
