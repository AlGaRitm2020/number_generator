
use std::env;
use std::io::Write;

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut file = std::fs::File::create("numbers.txt").expect("Create error");

    let ray  = vec!["0000", "1111", "2222", "3333", "4444", "5555", "6666", "7777", "8888", "9999"];
    
    for arg in &args[1..]{
        println!("{}\n\n\n", arg);
        
        let d = 0.0000001;
        let mut x = 0.0000001;
        if arg.chars().count() == 5 {
        
            let mut x = 0.000001;
            let d = 0.000001;
        }
        while x < 1.0 {
            x += d;
            
            let y = format!("{:.7}", x);
            if d == 0.000001 {
                let y = format!("{:.6}", x);
            }

            let c = &y[2..];

            let z = format!("{}{}\n", arg, c);
        
            let mut is_ok = true;
            for i in 0..10 {

                if z.contains(ray[i]) {
                    is_ok = false;
            
                }
            }

            if is_ok {
                println!("{}", z);
            
                file.write_all(z.as_bytes()).expect("Write error");
                println!("Number successfully writed");
        
            } 
        }
    }
}
