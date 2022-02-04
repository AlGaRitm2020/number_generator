fn main() {

    let ray  = vec!["0000", "1111", "2222", "3333", "4444", "5555", "6666", "7777", "8888", "9999", "0000"];
    let code = "8911";
    
    let mut x = 0.0000001;

    while x < 1.0 {
        x += 0.0000001;
        let y = format!("{:.7}", x);
        let c = &y[2..];

        let z = format!("{}{}", code, c);
        
        let mut is_ok = true;
        for i in 0..10 {

            if z.contains(ray[i]) {
                is_ok = false;
            }
        }
        if is_ok {
            println!("{}", z);
        }
    }
}
