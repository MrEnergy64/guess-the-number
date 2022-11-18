
/// RUST guess-the-number
/// a little game to demostrate
/// first steps with RUS programming
/// language and shows some programming
/// how to do's

/// Version 1.8 by Norman Wöske



use std::io::{self, Write};
use std::fs::OpenOptions;
use std::fs;
use rand::Rng;
use chrono::prelude::*;
use std::{thread, time};

extern crate rand;

fn main() {

    willkommen();
    
} //end of main()


pub fn set_color(c: &str) {
        match c {
                "red"           => print!("\x1b[31m"),
                "green"         => print!("\x1b[32m"),
                "yellow"        => print!("\x1b[33m"),
                "cyan"          => print!("\x1b[36m"),
                "magenta"       => print!("\x1b[35m"),
                "reset"         => print!("\x1b[0m"),
                _           => print!("{} is an invalid color ", c),
        }

} // end of set_color()

pub fn clear_screen() {
        print!("{}[2J", 27 as char);

} // end of clear_screen()

pub fn mv_point(line: usize, col: usize) {
        print!("\x1b[{};{}H", col, line);

} // end of mv_point()

pub fn pause(p: u64) {

        let millis = time::Duration::from_millis(p);
        thread::sleep(millis);

} // end of pause()


fn uhrzeit() {
	let now: DateTime<Local> = Local::now();
	println!("	{}", now.format("%a - %e %b %Y  - %T"));

} // end of uhrzeit()

fn rahmen() {
	let str1 = "=";
	let str2 = "|";
	set_color("green");
	println!("\n{}{}{}\n", str2.repeat(1), str1.repeat(44), str2.repeat(1));
	set_color("reset");

}  // end of rahmen()

fn warten() {
	let warten = [".", ".", "."];
		for x in 0..3 {
			print!("{} ", warten[x]);
			io::stdout().flush().unwrap();
			pause(800);
		}

} // end of warten()

fn willkommen() {
	clear_screen();
	mv_point(0,0);

	const WILLKOMMEN: &str = " 	
   	*****************************
   	*    W I L L K O M M E N    *
   	*                           *
   	* (c) Norman Wöske     V1.8 *
   	*****************************";
	
	rahmen();
	set_color("yellow");
	println!("{}", WILLKOMMEN);
	set_color("reset");
	uhrzeit();

	eingabe_namen();

} // end of willkommen()

fn eingabe_namen() {
	let mut namen = String::new();
	rahmen();
	print!("     Hallo, \x1b[94mSpieler\x1b[0m! Wie ist dein Name?\n\n     Name: ");
	
	let _ = io::stdout().flush();	
	io::stdin()
		.read_line(&mut namen)
		.expect("Fehler beim Lesen der Zeile");
	
	while namen.ends_with('\n') || namen.ends_with('\r') {
		namen.pop();
	}

	
	print!("\nSchön das du hier bist \x1b[94m{namen}\x1b[0m, möchtest du die Protokolldatei einsehen (1=ja, [enter]=nein)? ");

	let _ = io::stdout().flush();	
	let mut protokoll = String::new();
	io::stdin()
		.read_line(&mut protokoll)
		.expect("Fehler beim Lesen der Zeile");
	
	while protokoll.ends_with('\n') || protokoll.ends_with('\r') {
			protokoll.pop();
	}

	if protokoll == "1" {
		let fp = "results.txt";
		let b = std::path::Path::new(fp).exists();

		if b == false {
			println!("\nKeine Protokolldatei Result.txt vorhanden, weiter im Programm..");
			pause(3000);
		} else {
			clear_screen();
			mv_point(0,0);
			rahmen();
			set_color("yellow");
			println!("\n    Letzter Access am ....\n");
    		let file_path = "results.txt";
    		println!("Lade Datei... {}\n", file_path);

    		let contents = fs::read_to_string(file_path)
        		.expect("Etwas ging beim Lesen der Datei schief");

   		 	println!("Protokoll:\n{contents}\n     weiter in 8 Sekunden....\n");
			rahmen();
			set_color("reset");
			uhrzeit();

			pause(8000);
		}

	}

	gaming_time(namen);

} // end of eingabe_namen()

fn gaming_time(namen: String) {
	clear_screen();
	mv_point(0,0);


	const GAMING: &str = " 	
   	*****************************
   	*    G A M I N G  T I M E   *
   	*                           *
   	*                           *
   	*****************************";

	rahmen();
	set_color("yellow");
	println!("{}", GAMING);
	set_color("reset");
	uhrzeit();
	rahmen();
 	println!("     Hallo \x1b[94m{}\x1b[0m, lass uns ein Spiel spielen...\n", namen);
 	
	print!("     Bei 3 geht es los.... ");
	let warten = ["1", "2", "3"];
	for x in 0..3 {
		print!("{} ", warten[x]);
		io::stdout().flush().unwrap();
		pause(1500);
	}
	

	 zahlenspiel(namen);
	
} //end of gaming_time()

fn zahlenspiel(namen: String) {
	let zaehler: i32 = 1;

	let secret_number = rand::thread_rng().gen_range(1..101);
	
	let secret_number2 = secret_number.to_string();

	zahlen_eingabe(secret_number2, namen, zaehler);

} // end of zahlenspiel()

fn zahlen_eingabe(secret_number2: String, namen: String, zaehler: i32) {
	clear_screen();
	mv_point(0,0);
	
	const RATEN: &str = "
	*****************************
 	*       Rate die Zahl!      *
 	*                           *
	*                           *
 	*****************************";

 	rahmen();
 	set_color("yellow");
	println!("{}", RATEN);
	set_color("reset");
	uhrzeit();
	set_color("magenta");
	println!("\n            Rateversuch {} von 10...", zaehler);
	set_color("reset");
	rahmen();
	let mut guess = String::new();
 	print!("\x1b[94m{}\x1b[0m, bitte gib deine Zahl zwischen 1-100 ein.\n\n           => : ", namen);
	let _ = io::stdout().flush();
 	io::stdin()
		.read_line(&mut guess)
	 	.expect("Fehler beim Lesen der Zeile");
	
	while guess.ends_with('\n') || guess.ends_with('\r') {
		guess.pop();
	}

	auswertung(secret_number2, guess, namen, zaehler);

} // end of zahlen_eingabe()

fn auswertung(secret_number2: String, guess: String, namen: String, zaehler: i32) {
	clear_screen();
	mv_point(0,0);

	const AUSWERTUNG: &str = "
	*****************************
 	*    A U S W E R T U N G    *
 	*                           *
	*                           *
 	*****************************";

	rahmen();
 	set_color("yellow");
	println!("{}", AUSWERTUNG);
	set_color("reset");
	uhrzeit();
	rahmen();

	if zaehler >= 10 {
		looser(secret_number2, guess, namen);
	} else {
		let secret_int: u32 = secret_number2
		.trim()
		.parse()
		.expect("Wanted a number");

		let guess_int: u32 = guess
		.trim()
		.parse()
		.expect("Wanted a number"); 
	
		if guess_int > secret_int {
			print!("\nDeine Zahl {} ist größer als die Geheimzahl, rate weiter ", guess);
			warten();
			let zaehler = zaehler + 1;
			zahlen_eingabe(secret_number2, namen, zaehler);
		} else if  guess_int < secret_int {
			print!("\nDeine Zahl {} ist kleiner als die Geheimzahl, rate weiter ", guess);
			warten();
			let zaehler = zaehler + 1;
			zahlen_eingabe(secret_number2, namen, zaehler);
		} else {
			winner(secret_number2, namen, zaehler);
		}
	}	
} // end of auswertung()

fn winner(secret_number2: String, namen: String, zaehler: i32) {
	clear_screen();
	mv_point(0,0);

	const WINNER: &str = "
	*****************************
	*    !!   GEWINNER   !!     *
	*                           *
	*           :-)             *
	*****************************";

	rahmen();
	set_color("yellow");
	println!("{}", WINNER);
	set_color("reset");
	uhrzeit();
	rahmen();
	
	let now: DateTime<Local> = Local::now();

	let fp = "results.txt";
	let b = std::path::Path::new(fp).exists();

	if b == false {
		let mut w = OpenOptions::new()
			.create_new(true)
			.write(true)
			.append(true)
			.open("results.txt")
			.unwrap();
		if let Err(e) = writeln!(&mut w, "Datum: {}, Name: {}, Gewonnen in {} versuchen.", now.format("%a - %e %b %Y  - %T"), namen, zaehler) {
			eprintln!("Couldn't write to file: {}", e);
		}
	
			
	} else {
		let mut w = OpenOptions::new()
			.write(true)
			.append(true)
			.open("results.txt")
			.unwrap();
		if let Err(e) = writeln!(&mut w, "Datum: {}, Name: {}, Gewonnen in {} versuchen.", now.format("%a - %e %b %Y  - %T"), namen, zaehler) {
			eprintln!("Couldn't write to file: {}", e);
		}
					
	}

	println!("\x1B[3m   *** Juhuu, \x1b[94m{}\x1b[0m\x1B[3m, du hast gewonnen!! ***\n\x1b[0m", namen);
	println!("Die zu erratende Zahl war: \x1b[93m{}\x1b[0m, und in \x1b[93m{}\x1b[0m Versuch(e) erraten.\n", secret_number2, zaehler);
	pause(1000);

	nochmal(namen);

} //end of winner

fn looser(secret_number2: String, x: String, namen: String) {
	clear_screen();
	mv_point(0,0);

	const LOSER: &str = "
	*****************************
	*    !!   VERLOREN   !!     *
	*                           *
	*           :-(             *
	*****************************";

	rahmen();
	set_color("cyan");
	println!("{}", LOSER);
	set_color("reset");
	uhrzeit();
	rahmen();

	let now: DateTime<Local> = Local::now();

	let fp = "results.txt";
	let b = std::path::Path::new(fp).exists();

	if b == false {
		let mut w = OpenOptions::new()
			.create_new(true)
			.write(true)
			.append(true)
			.open("results.txt")
			.unwrap();
		if let Err(e) = writeln!(&mut w, "Datum: {}, Name: {}, Verloren!!", now.format("%a - %e %b %Y  - %T"), namen) {
			eprintln!("Couldn't write to file: {}", e);
		}

	} else {
		let mut w = OpenOptions::new()
			.write(true)
			.append(true)
			.open("results.txt")
			.unwrap();
		if let Err(e) = writeln!(&mut w, "Datum: {}, Name: {}, Verloren!!", now.format("%a - %e %b %Y  - %T"), namen) {
			eprintln!("Couldn't write to file: {}", e);
		}
	}

	println!("\x1B[3m >> Schade, \x1b[94m{}\x1b[0m\x1B[3m, du hast leider verloren. <<\n\x1b[0m", namen);
	println!("Die zu erratende Zahl war: \x1b[93m{}\x1b[0m, deine letzte Zahl war: \x1b[93m{}\x1b[0m\n", secret_number2, x);
	pause(1000);

	nochmal(namen);

} //end of looser()

fn nochmal(namen: String) {
	let mut weiter = String::new();
	let mut neuer = String::new();

	println!("\n\x1b[94m{}\x1b[0m, möchtest Du nochmal spielen (schreibe 'nein' für beenden, ansonsten [enter] für weiter)?", namen);
	
	io::stdin()
	    .read_line(&mut weiter)
	    .expect("Fehler beim Lesen der Zeile");
   	
	if weiter.trim() == "nein" {
		clear_screen();
	 	mv_point(0,0);
		println!("\n\x1b[0mSchade, Goodbye \x1b[94m{}\x1b[0m\n", namen);   		   	
   	} else {
		println!("Spiel neu starten mit neuem Spieler (schreibe 'ja', ansonsten [enter] für gleichen Spieler) ?");
		io::stdin()
			.read_line(&mut neuer)
	    	.expect("Fehler beim Lesen der Zeile");

		if neuer.trim() == "ja" {
			main();
		} else {
			gaming_time(namen);
		}
   	}
} //end of nochmal()
