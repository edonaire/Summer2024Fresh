
//assignment 1, esteban donaire

const FREEZING_POINT:f32 = 32.0;


fn celsius_to_farenheit(c:f64)->f64{
    c * 9.0/5.0 + 32.0


}

fn farenheit_to_celsius(f:f64)->f64{
    (f-32.0)*5.0/9.0


}

fn main(){
    println!("{}", FREEZING_POINT);
    let mut farenheit: f64 =100.0;

    let in_celsius =farenheit_to_celsius(farenheit);


    println!("100 farenheit is {}", in_celsius);

    let mut counter = 1;

    while counter != 6 {
        farenheit +=1.0;
        let incelsius = farenheit_to_celsius(farenheit);
        println!("{} farenheit is {} celsius", farenheit, incelsius);
        counter += 1;
    }
}