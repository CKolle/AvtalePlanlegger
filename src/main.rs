use core::fmt;
use std::io::Write;

use chrono::{Local, NaiveDateTime, TimeZone};
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::prelude::*;

#[derive(Serialize, Deserialize, PartialEq, Clone)]
struct Avtale {
    tittel: String,
    sted: String,
    varighet: i32,
    #[serde(with = "dato_format")]
    starttidspunkt: chrono::DateTime<Local>,
}

impl std::fmt::Display for Avtale {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Tittle: {}, Sted: {}, varighet: {}, Starttidspunkt: {}",
            self.tittel,
            self.sted,
            self.varighet,
            self.starttidspunkt.format("%Y-%m-%d %H:%M:%S")
        )
    }
}
mod dato_format {
    use chrono::{DateTime, Local, TimeZone};
    use serde::{self, Deserialize, Deserializer, Serializer};

    const FORMAT: &'static str = "%Y-%m-%d %H:%M:%S";

    pub fn serialize<S>(date: &DateTime<Local>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = format!("{}", date.format(FORMAT));
        serializer.serialize_str(&s)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<DateTime<Local>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Local
            .datetime_from_str(&s, FORMAT)
            .map_err(serde::de::Error::custom)
    }
}

fn lag_avtale() -> Avtale {
    let mut tittel = String::new();

    print!("Skriv inn tittelen på avtalen: ");
    std::io::stdout().flush().unwrap();
    std::io::stdin()
        .read_line(&mut tittel)
        .expect("Klarte ikke å lese in data");
    tittel.pop();

    let mut sted = String::new();

    print!("Skriv inn stedet til avtalen: ");
    std::io::stdout().flush().unwrap();
    std::io::stdin()
        .read_line(&mut sted)
        .expect("Klarte ikke å lese inn stedet");
    sted.pop();

    let varighet;
    loop {
        print!("Skriv inn varigheten til avtalen: ");
        std::io::stdout().flush().unwrap();
        let mut varighet_str = String::new();

        std::io::stdin()
            .read_line(&mut varighet_str)
            .expect("Kunne ikke lese in varigheten");

        varighet = match varighet_str.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Prøv igjen");
                continue;
            }
        };
        break;
    }

    let starttidspunkt: chrono::DateTime<Local>;
    loop {
        print!("Skriv inn startidspunktet for avtalen (YYYY-MM-DD HH:MM): ");
        std::io::stdout().flush().unwrap();
        let mut tid_dato = String::new();

        std::io::stdin()
            .read_line(&mut tid_dato)
            .expect("Kunne ikke lese inn dato og tid");
        tid_dato.pop();

        starttidspunkt = match NaiveDateTime::parse_from_str(&tid_dato, "%Y-%m-%d %H:%M") {
            Ok(time_date) => Local.from_local_datetime(&time_date).unwrap(),
            Err(_) => {
                println!("Prøv igjen");
                continue;
            }
        };
        break;
    }

    let avtale = Avtale {
        tittel: tittel,
        sted: sted,
        varighet: varighet,
        starttidspunkt: starttidspunkt,
    };
    return avtale;
}

fn print_avtaler(avtale_liste: &Vec<Avtale>) {
    for i in 0..avtale_liste.len() {
        println!("Index:{} Avtale: {}", i, avtale_liste[i]);
    }
}

fn les_avtaler() -> Result<Vec<Avtale>, std::io::Error> {
    let mut fil = File::open("avtale.json")?;
    let mut innhold = String::new();
    fil.read_to_string(&mut innhold)?;
    let avtale_liste: Vec<Avtale> = serde_json::from_str(&innhold)?;
    Ok(avtale_liste)
}

fn large_avtaler(avtale_liste: &Vec<Avtale>) {
    let json_data = serde_json::to_string_pretty(&avtale_liste).unwrap();
    let mut fil = File::create("avtale.json").unwrap();
    fil.write_all(&json_data.as_bytes()).unwrap();
}

fn skjekk_samme_dato(avtale_liste: &Vec<Avtale>, dato: &chrono::Date<Local>) -> Vec<Avtale> {
    let filtert = avtale_liste
        .iter()
        .filter(|&avtale| &avtale.starttidspunkt.date() == dato)
        .cloned()
        .collect();
    return filtert;
}

fn søk_avtaler(avtale_liste: &Vec<Avtale>, søke_ord: &str) -> Vec<Avtale> {
    let filtrert = avtale_liste
        .into_iter()
        .filter(|&avtale| avtale.tittel.contains(&søke_ord.to_lowercase()))
        .cloned()
        .collect();
    return filtrert;
}

fn meny() {
    let mut avtaler: Vec<Avtale> = Vec::new();
    loop {
        println!("Ny avtale [1]");
        println!("Lagre avtaler [2]");
        println!("Se avtaler [3]");
        println!("Les avtaler fra avtale.json fil [4]");
        println!("Avslutt programmet [5]");
        let mut svar_str = String::new();
        print!(": ");
        std::io::stdout().flush().unwrap();
        std::io::stdin().read_line(&mut svar_str).unwrap();
        let svar: u8;
        svar = match svar_str.trim().parse() {
            Ok(svar) => svar,
            Err(_) => {
                println!("Prøv igjen");
                continue;
            }
        };

        match svar {
            1 => {
                let avtale = lag_avtale();
                println!("La til ny avtale");
                avtaler.push(avtale);
            }
            2 => {
                large_avtaler(&avtaler);
                println!("Lagret avtaler")
            }
            3 => {
                print_avtaler(&avtaler);
            }
            4 => {
                let leste_avtaler = les_avtaler().expect("kunne ikke lese avtaler");
                println!("Leste avtaler");
                avtaler.extend(leste_avtaler);
            }
            5 => break,
            _ => {}
        }
        print!("Trykk enter for neste kommando ...");
        std::io::stdout().flush().unwrap();
        std::io::stdin().read(&mut [0]).unwrap();
        print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
        std::io::stdout().flush().unwrap();
    }
}
fn main() {
    // let mut avtaler: Vec<Avtale> = Vec::new();
    // for _ in 1..3 {
    //     let avtale = lag_avtale();
    //     avtaler.push(avtale);
    // }
    // // large_avtaler(&avtaler);
    // // let avtale_liste = les_avtaler().expect("Kunne ikke lese avtaler");
    // // let dato_str = "2002-12-2";
    // // let naive_date = NaiveDate::parse_from_str(dato_str, "%Y-%m-%d").unwrap();
    // // let dato = Local.from_local_date(&naive_date).unwrap();
    // // let samme_dato = skjekk_samme_dato(&avtale_liste, &dato);
    // // print_avtaler(&samme_dato);
    // let søke_ord = "heiSann";
    // let nye_avtaler = søk_avtaler(&avtaler, søke_ord);
    // print_avtaler(&nye_avtaler);
    meny()
}
