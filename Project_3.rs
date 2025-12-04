use std::io;
use phf::phf_map;

 //Converter jednostek

static dlugosci: phf::Map<&'static str, f64> = phf_map! {
    "km"    => 1000.0,
    "m"     => 1.0,
    "cm"    => 0.01,
    "mm"    => 0.001,
    "mila"  => 1609.344,
    "cal"   => 0.0254,
    "stopa" => 0.3048,
    "yard"  => 0.9144,
};

static waga: phf::Map<&'static str, f64> = phf_map! {
    "kg"      => 1.0,
    "g"       => 0.001,
    "mg"      => 0.000001,
    "tona"    => 1000.0,
    "funt"    => 0.45359237,
    "uncja"   => 0.028349523125,
};

static objetosci: phf::Map<&'static str, f64> = phf_map! {
    "litr"    => 0.001,
    "ml"      => 0.000001,
    "m3"      => 1.0,
    "galon_us"=> 0.003785411784,
    "galon_uk"=> 0.00454609,
};

fn main() {
    witam()
}

fn witam() {
    println!("WItaj w Programie Zmiana jednostek");
    println!("|=Wybur dzialania zmiany jednostek=|");
    println!("zmiana jednostki: ");
    println!("           1 - Długosci");
    println!("           2 - Wagi");
    println!("           3 - Objetosci");
    println!("           5 - Wyjdz z programu");
    println!("|==================================|");

    zapytanie()
}

fn zapytanie() {
    loop {
        let mut wybur_zmiany : String = String::new();
        io::stdin()
            .read_line(&mut wybur_zmiany)
            .unwrap()
        let wybur_zmiany = wybur_zmiany.trim();

        match wybur_zmiany {
            "1" => konwertuj(&dlugosci, &wybur_zmiany),
            "2" => konwertuj(&waga, &wybur_zmiany),
            "3" => konwertuj(&objetosci, &wybur_zmiany),
            "5" => {
                println!("Program sie zamknie");
                println!("dziekuje za skorzystanie z naszych uslug");
                break;
            }
            _ => println!("Nieotrzymalem numeru Wyboru."),
        }

    }
}

fn konwertuj() {

}
