static BIAS: i32 = 127;

pub fn split_floating_number(f: f32) {
    let bits = f.to_bits();
    let sign = bits >> 31;

    println!();
    println!("Number:   {}\n", f);

    println!(
        "          {:<32} | {:<32} | {:<32}",
        "Binary", "Decimal", "Converted"
    );
    println!(
        "{} | {} | {}",
        "-".repeat(10 + 32),
        "-".repeat(32),
        "-".repeat(32)
    );

    println!(
        "Sign:     {:<32b} | {:<32} | {:<32}",
        sign,
        sign,
        (-1_i32).pow(sign)
    );

    let exp = (bits >> 23 & 0xff) as i32 - BIAS;

    println!(
        "Exponent: {:<32b} | {:<32} | {:<32}",
        exp + BIAS,
        exp + BIAS,
        exp
    );

    let mut mantissa: f32 = 1.0;

    for i in 0..23 {
        let mask = 1 << i;
        let bit = bits & mask;

        if bit != 0 {
            mantissa += 2_f32.powf(i as f32 - 23.0);
        }
    }

    println!(
        "Mantissa: {:<32b} | {:<32} | {:<32}\n",
        bits & 0x7fffff,
        bits & 0x7fffff,
        mantissa
    );

    let sign = (-1_i32).pow(sign);

    println!(
        "Value:    {} (value = -1 ^ Sign * Mantissa * 2 ^ Exponent, Standard IEEE754)",
        sign as f32 * mantissa * 2_f32.powi(exp)
    );
}
