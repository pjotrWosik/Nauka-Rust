use std::io;

fn main() {
    let mut input_1 = String::new();        //zapisanie zmiennej
    let mut input_2 = String::new();

    println!("podaj numer 1: ");
    io::stdin()
        .read_line(&mut input_1)
        .expect("blad odczytu liczby 1");
    let liczba1: f32 = input_1.trim().parse().expect("to nie jest poprawna liczba");

    println!("podaj numer 2: ");                 //wypisanie textu na ekranie
    io::stdin()                                  //rust wczytuje dane
        .read_line(&mut input_2)    //rust czyta tekst z klawiatury
        .expect("blad odczytu liczby 2");   //zabezpieczenie jerzeli cos rust'owi nie pasuje to to wypisuje
    let liczba2: f32 = input_2.trim().parse().expect("to nie jest poprawna liczba");

    let liczba3 = liczba1 + liczba2;                                     //zmienna liczba 3 musi byc na tyle spowodu niezadeklarowania liczby 1 i 2
    println!("Wynik dodawania to: {} + {} = {}", liczba1, liczba2, liczba3);  //oblicza caly sys

    let liczba3 = liczba1 - liczba2;
    println!("Wynik odejmowania to: {} - {} = {}", liczba1, liczba2, liczba3);
}