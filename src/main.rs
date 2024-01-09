use std::io::{stdin, Read};

use clap::{arg, command, Parser};

const ALPHABETIC: &str = "{a}";
const BOLD: &str = "**";
const HIGHLIGHT: &str = "==";

pub mod data;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    // #[arg(short, long)]
    // title: String,
    // #[arg(short, long)]
    // grupo: String,
    // #[arg(short, long)]
    // answers: String,
}

#[derive(Debug)]
struct Pregunta {
    enunciado: String,
    preguntas: [String; 4],
    correcta: usize,
}

// fn leer_entrada() -> Vec {}

fn main() {
    // let args = Args::parse();

    let mut parsed = String::new();

    let mut title = String::new();
    let mut grupo = String::new();
    let mut answers_unparsed = String::new();

    stdin().read_line(&mut title).unwrap();
    stdin().read_line(&mut grupo).unwrap();
    stdin().read_line(&mut answers_unparsed).unwrap();

    let title = title.trim();
    let grupo = grupo.trim();
    let answers = {
        let mut vec: Vec<usize> = Vec::new();

        for c in answers_unparsed.trim().chars() {
            let v = match c {
                'a' => 0,
                'b' => 1,
                'c' => 2,
                'd' => 3,
                _ => panic!("wtf is this? {}", c),
            };

            vec.push(v);
        }

        vec
    };

    // Read whole stdin
    for line in stdin().lines() {
        parsed.push_str(&line.unwrap());
        parsed.push(' ');
    }

    let parsed = parsed.replace("Pregunta", "\nPregunta");
    let parsed = parsed.trim();

    let mut preguntas = Vec::new();

    let mut lines = parsed.lines();

    for line in lines {
        #[allow(unused_assignments)]
        let mut resto = "";

        let mut iter = line.split("a)");
        let enunciado = iter.next().unwrap();
        let enunciado = enunciado.split(":").nth(1).unwrap().trim();
        resto = iter.next().unwrap();

        let mut iter = resto.split("b)");
        let a = iter.next().unwrap();
        resto = iter.next().unwrap();

        let mut iter = resto.split("c)");
        let b = iter.next().unwrap();
        resto = iter.next().unwrap();

        let mut iter = resto.split("d)");
        let c = iter.next().unwrap();
        let d = iter.next().unwrap();

        preguntas.push(Pregunta {
            enunciado: enunciado.to_string(),
            preguntas: [
                a.trim().to_string(),
                b.trim().to_string(),
                c.trim().to_string(),
                d.trim().to_string(),
            ],
            correcta: 4,
        })
    }

    let grupo: usize = grupo.parse::<usize>().unwrap();
    println!(
        "# INTU{}: {}",
        if grupo < 10 {
            format!("0{}", grupo)
        } else {
            grupo.to_string()
        },
        title,
    );
    println!("");

    for (i, pregunta) in preguntas.iter().enumerate() {
        println!("1. {}{}{}", BOLD, pregunta.enunciado, BOLD);
        let selected = answers[i];
        for j in 0..4 {
            println!(
                "\t1. {}{}{}{}",
                if j == 0 {
                    format!("{ALPHABETIC} ")
                } else {
                    "".to_string()
                },
                if j == selected { HIGHLIGHT } else { "" },
                pregunta.preguntas[j],
                if j == selected { HIGHLIGHT } else { "" },
            );
        }
    }
    println!("");
}
