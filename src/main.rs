use std::io;

fn main() {
    loop {
        println!("Type in the fahrenheight temperature");

        let mut fah = String::new();

        io::stdin()
            .read_line(&mut fah)
            .expect("Oops");

        if fah.trim() == "q" {
            println!("Bye!");
            break;
        }

        let fah: f32 = match fah.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        let cel = (fah - 32.0) / 1.8;

        println!("{}", cel);
    }
}
