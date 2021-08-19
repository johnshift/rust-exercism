pub fn is_armstrong_number(num: u32) -> bool {
    if num == 0 {
        return true;
    }

    let number_of_digits: u32 = ((num as f64).log10()) as u32 + 1_u32;
    let mut digits: Vec<u32> = Vec::new();

    let mut ref_num: u32 = num;
    for d in (1..=number_of_digits).rev() {
        let mult: u32 = 10_u32.pow(d - 1);
        let digit = (ref_num / mult) as u32;
        digits.push(digit);
        if d != 1 {
            ref_num -= digit * mult;
        }
    }

    let sum = digits
        .into_iter()
        .map(|x| x.pow(number_of_digits))
        .sum::<u32>();

    sum == num
}
