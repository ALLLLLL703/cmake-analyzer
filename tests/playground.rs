pub fn longer<'a, 'b>(str1: &'a str, str2: &'a str) -> &'a str {
    if str1.len() > str2.len() { str1 } else { str2 }
}

fn main() {
    let a = "114";
    let result;
    {
        let b = "514";
        result = longer(a, b);
    }
    println!("{result}");
}
