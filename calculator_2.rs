use std::io;

fn main() {
    let mut input1 = String::new();
    let mut input2 = String::new();
    let mut wybur = String::new();

    println!("podaj liczbe 1");
    io::stdin()
        .read_line(&mut input1)
        .expect("blad");
    let liczba1: f32 = input1.trim().parse().expect("to nie jest liczba");

    println!("podaj liczbe 2");
    io::stdin()
        .read_line(&mut input2)
        .expect("blad 2");
    let liczba2: f32 = input2.trim().parse().expect("to nie jest liczba");

    println!("wybierz dzialanie");
    println!("1 - dodawanie");
    println!("2 - odejmowanie");
    println!("3 - mnorzenie");
    println!("4 - dzielenie");
    io::stdin()
        .read_line(&mut wybur)
        .expect("blad przy wyborze");

    if wybur.trim() == "1" {
        let liczba3 = liczba1 + liczba2;
        println!("wynik dodawania to: {} + {} = {}", liczba1, liczba2, liczba3);
    }
    else if wybur.trim() == "2" {
        let liczba3 = liczba1 - liczba2;
        println!("wynik odejmowania to: {} - {} = {}", liczba1, liczba2, liczba3);
    }
    else if wybur.trim() == "3" {
        let liczba3 = liczba1 * liczba2;
        println!("wynik mnorzenia to: {} * {} = {}", liczba1, liczba2, liczba3);
    }
    else if wybur.trim() == "4" {
        let liczba3 = liczba1 / liczba2;
        println!("wynik dzielenia to: {} / {} = {}", liczba1, liczba2, liczba3);
    }
    else {
        println!("niepoprawna liczba");
    }

/*
    let liczba3 = liczba1 + liczba2;
    println!("wynik dodawania to: {} + {} = {}", liczba1, liczba2, liczba3);

    let liczba3 = liczba1 - liczba2;
    println!("wynik odejmowania to: {} - {} = {}", liczba1, liczba2, liczba3);

    let liczba3 = liczba1 * liczba2;
    println!("wynik mnorzenia to: {} * {} = {}", liczba1, liczba2, liczba3);

    let liczba3 = liczba1 / liczba2;
    println!("wynik dzielenia to: {} / {} = {}", liczba1, liczba2, liczba3);
 */
}
