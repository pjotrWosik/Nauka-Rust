use std::io;


fn main() {
    witam();
    gra();

}

fn witam() {
    println!("ようこそ！");
    print!("U will now play A game:");
    println!("U will need to guess a number betwen 100 - 1");
    println!("U choose wisely u hawe only 5 tries");
    println!(" ");
    println!("write /yes/ if you want to start or /stop/ if you want to stop game: ");

    loop {
        let mut start_inp = String::new();
        io::stdin()
            .read_line(&mut start_inp)
            .unwrap();
        let start_inp = start_inp.trim();

        if start_inp == "yes" || start_inp == "true" {
            println!("lets start the game:");
            println!("write an number between 1 and 100:");
            break;
        }
        else if start_inp == "stop" {
            println!("the game hawe ben stoped C ya later:");
            break;
        } else {
            println!("please write /yes/ or /stop/");
        }
    }
}
fn gra() {
    println!("U will need to guess a number betwen 100 - 1\n");
    let magiczna = 45;
    loop {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .unwrap();
        let user_liczba: i32  = input.trim().parse().expect("u forgot i need an number!");

        if user_liczba < magiczna {
            println!("liczba jest za mala");
        }
        else if user_liczba > magiczna {
            println!("liczba jest za durza");
        }
        else if user_liczba == magiczna {
            println!("udalo sie kotku");
            break;
        }
    }

}
