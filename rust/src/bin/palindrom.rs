fn is_palindrom(x: i32) -> bool {
    let parsed = x.to_string();
    let chars: Vec<char> = parsed.chars().collect();
    let mut r = 0;
    let mut l = parsed.len() - 1;

    print!("{}", parsed);

    while r < l {
        let val1 = chars.get(r).unwrap();
        let val2 = chars.get(l).unwrap();

        if val1 != val2 {
            return false;
        }
        r +=1;
        l -=1;

    }
    return true;
}

fn main() {
    print!("{:?}",is_palindrom(222));
}
