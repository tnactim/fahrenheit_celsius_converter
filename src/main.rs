use std::io;

// Convert temperature to fahrenheit and celsius, assuming both
// add menu?
// add rankine and kelvin
fn main() {
    println!( "Input temperature: " );

    let mut temp = String::new();

    io::stdin().read_line( &mut temp )
        .expect( "failed to read line" );

    let temp: f64 = temp.trim().parse().unwrap();

    f_to_c( temp );
    c_to_f( temp );
}

fn f_to_c( f: f64 ) {
    let c = (f - 32.0) * (5.0 / 9.0);
    println!("fahrenheit to celsius: {}", c );
}

fn c_to_f( c: f64 ) {
    let f = c * 1.8 + 32.0;
    println!("celsius to fahrenheit: {}", f );
}
