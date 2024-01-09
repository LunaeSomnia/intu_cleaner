use std::{
    io::{stderr, stdin, stdout, Read, Write},
    process::ExitCode,
};

use clap::{command, Parser, Subcommand};
use colored::Colorize;
use data::INTU_GRUPOS;
use question::Question;
use rand::Rng;

use crate::data::get_data_from_group;

const ALPHABETIC: &str = "{a}";
const BOLD: &str = "**";
const HIGHLIGHT: &str = "==";

pub mod data;
pub mod question;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Genera un numero de preguntas de cualquier grupo
    Test { preguntas: usize },
    /// Genera un numero de preguntas del grupo introducido
    TestGrupo { preguntas: usize, grupo: usize },
    /// Genera un numero de preguntas de los grupos introducidos
    TestGrupos {
        preguntas: usize,
        grupos: Vec<usize>,
    },
    /// Genera un markdown con las preguntas del grupo introducido
    Markdown { grupo: usize },
}

fn throw_error(error: &str) {
    println!("{}{} {error}", "ERROR".on_red().bold(), ":".black());
}

fn print_question(question: &Question) {
    println!("{}", question.desc().bold());
    for (i, option) in question.options().iter().enumerate() {
        println!(
            "{}{} {}",
            index_to_option(i).unwrap().to_string().black(),
            ")".black(),
            option
        );
    }
}

fn main() -> ExitCode {
    let args = Args::parse();

    let mut groups = INTU_GRUPOS.to_vec();
    let mut to_generate = 0;

    match &args.command {
        Some(Commands::Test { preguntas }) => {
            let max = 10 * INTU_GRUPOS.len();
            if *preguntas > 10 * INTU_GRUPOS.len() {
                throw_error(&format!(
                    "Eso de \"Solo 10 preguntas por grupo\" te lo has saltao no? MAX: {}",
                    max
                ));
                return ExitCode::FAILURE;
            }

            to_generate = *preguntas;
        }
        Some(Commands::TestGrupo { grupo, preguntas }) => {
            let max = 10;
            if *preguntas > 10 {
                throw_error(&format!(
                    "Eso de \"Solo 10 preguntas por grupo\" te lo has saltao no? MAX: {}",
                    max
                ));
                return ExitCode::FAILURE;
            }

            groups = vec![*grupo];
            to_generate = *preguntas;
        }
        Some(Commands::TestGrupos { preguntas, grupos }) => {
            let max = 10 * grupos.len();
            if *preguntas > 10 * grupos.len() {
                throw_error(&format!(
                    "Eso de \"Solo 10 preguntas por grupo\" te lo has saltao no? MAX: {}",
                    max
                ));
                return ExitCode::FAILURE;
            }

            groups = grupos.clone();
            to_generate = *preguntas;
        }
        Some(Commands::Markdown { grupo }) => {
            let data = match get_data_from_group(*grupo) {
                Ok(d) => d,
                Err(e) => {
                    throw_error(&e);
                    return ExitCode::FAILURE;
                }
            };

            let group_number = data.group_number();
            println!(
                "# INTU{}: {}",
                if group_number < 10 {
                    format!("0{}", group_number)
                } else {
                    group_number.to_string()
                },
                data.title(),
            );
            println!();

            for pregunta in data.questions().iter() {
                println!("1. {}{}{}", BOLD, pregunta.desc(), BOLD);
                for j in 0..4 {
                    println!(
                        "\t1. {}{}{}{}",
                        if j == 0 {
                            format!("{ALPHABETIC} ")
                        } else {
                            "".to_string()
                        },
                        if j == pregunta.correct() {
                            HIGHLIGHT
                        } else {
                            ""
                        },
                        pregunta.options()[j],
                        if j == pregunta.correct() {
                            HIGHLIGHT
                        } else {
                            ""
                        },
                    );
                }
            }
            println!();
        }
        None => {}
    }

    let mut question_pool = Vec::new();

    for group in groups {
        question_pool.append(&mut get_data_from_group(group).unwrap().questions().to_vec())
    }

    let mut rand_thread = rand::thread_rng();
    for i in 0..to_generate {
        let rand_index = rand_thread.gen_range(0..question_pool.len());

        let question = question_pool.remove(rand_index);

        print_question(&question);

        let mut answer = String::new();
        let mut answer_index = 4;
        let mut valid_answer = false;
        while !valid_answer {
            print!("{} {}", "Respuesta".bright_black(), "> ".yellow());
            stdout().flush().unwrap();

            stdin().read_line(&mut answer).unwrap();

            answer = answer.trim().to_string();

            if answer.is_empty() {
                answer = String::new();
                throw_error(
                    "Nunca has aprendido a hacer un test? Se responde con 'a', 'b', 'c' o 'd'.",
                );
                continue;
            }

            let char = answer.chars().next().unwrap();
            if let Ok(index) = option_to_index(char) {
                valid_answer = true;
                answer_index = index;
            } else {
                answer = String::new();
                throw_error(
                    "Nunca has aprendido a hacer un test? Se responde con 'a', 'b', 'c' o 'd'.",
                )
            }
        }

        println!();

        if answer_index == question.correct() {
            println!("{}", "Correcto".bold().on_green());
            println!("{}", "[Enter] para siguiente".black());
        } else {
            println!(
                "{} {} {}",
                "Incorrecto".bold().on_red(),
                "->".yellow(),
                index_to_option(question.correct())
                    .unwrap()
                    .to_string()
                    .green()
                    .bold()
            );
            println!("{}", "[Enter] para siguiente".black());
        }

        stdin().read_line(&mut answer).unwrap();
    }

    ExitCode::SUCCESS
}

pub fn option_to_index(char: char) -> Result<usize, String> {
    Ok(match char {
        'a' => 0,
        'b' => 1,
        'c' => 2,
        'd' => 3,
        _ => return Err("Invalid character".to_string()),
    })
}

pub fn index_to_option(index: usize) -> Result<char, String> {
    Ok(match index {
        0 => 'a',
        1 => 'b',
        2 => 'c',
        3 => 'd',
        _ => return Err("Invalid index".to_string()),
    })
}
