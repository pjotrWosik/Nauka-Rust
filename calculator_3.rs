use std::io;

fn main() {
    loop {
        let mut input1 = String::new();
        let mut input2 = String::new();
        let mut wybur = String::new();

        println!("Wpisz numer 1:");
        io::stdin()
            .read_line(&mut input1)
            .expect("niewpisano inputu 1");
        let liczba1: f32 = input1.trim().parse().expect("problem z liczba 1");

        println!("Wpisz numer 2:");
        io::stdin()
            .read_line(&mut input2)
            .expect("niewpisano inputu 2");
        let liczba2: f32 = input2.trim().parse().expect("problem z liczba 2");

        println!("wybierz Dzialanie z wybranych:");
        println!("1 - dodawanie");
        println!("2 - odejmowanie");
        println!("3 - mnorzenie");
        println!("4 - dzielenie");
        io::stdin()
            .read_line(&mut wybur)
            .expect("niewpisano wyboru");

        if wybur.trim() == "1" {
            let liczba3 = liczba1 + liczba2;
            println!("wynik dodawania to {} + {} = {}", liczba1, liczba2, liczba3);
        }
        else if wybur.trim() == "2" {
            let liczba3 = liczba1 - liczba2;
            println!("wynik odejmowanie to {} - {} = {}", liczba1, liczba2, liczba3);
        }
        else if wybur.trim() == "3" {
            let liczba3 = liczba1 * liczba2;
            println!("wynik mnorzenie to {} * {} = {}", liczba1, liczba2, liczba3);
        }
        else if wybur.trim() == "4" {
            let liczba3 = liczba1 / liczba2;
            println!("wynik dzielenie to {} / {} = {}", liczba1, liczba2, liczba3);
        }
        let mut kontynuj = String::new();
        println!("Wpisz 1 jerzeli chcesz kontynuowac, lub cos innego jerzeli chcesz zakonczyczyc:");
        io::stdin()
            .read_line(&mut kontynuj)
            .expect("blad z kontynuj");

        if kontynuj.trim() != "1" {
            println!("koniec Programu");
            break;
        }
    }
}