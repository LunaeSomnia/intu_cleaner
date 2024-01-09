use crate::{option_to_index, question::Question};

pub const INTU_GRUPOS: &[usize] = &[
    1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 20, 21, 22, 23, 24, 25, 26, 27,
    28, 29, 30, 31, 32, 33, 34, 36,
];

pub const INTU01: &str = include_str!("../data/INTU01.txt");
pub const INTU02: &str = include_str!("../data/INTU02.txt");
pub const INTU03: &str = include_str!("../data/INTU03.txt");
pub const INTU04: &str = include_str!("../data/INTU04.txt");
pub const INTU05: &str = include_str!("../data/INTU05.txt");
pub const INTU06: &str = include_str!("../data/INTU06.txt");
pub const INTU07: &str = include_str!("../data/INTU07.txt");
pub const INTU08: &str = include_str!("../data/INTU08.txt");
pub const INTU09: &str = include_str!("../data/INTU09.txt");
pub const INTU10: &str = include_str!("../data/INTU10.txt");
pub const INTU11: &str = include_str!("../data/INTU11.txt");
pub const INTU12: &str = include_str!("../data/INTU12.txt");
pub const INTU13: &str = include_str!("../data/INTU13.txt");
pub const INTU14: &str = include_str!("../data/INTU14.txt");
pub const INTU15: &str = include_str!("../data/INTU15.txt");
pub const INTU16: &str = include_str!("../data/INTU16.txt");
pub const INTU17: &str = include_str!("../data/INTU17.txt");
pub const INTU18: &str = include_str!("../data/INTU18.txt");
pub const INTU20: &str = include_str!("../data/INTU20.txt");
pub const INTU21: &str = include_str!("../data/INTU21.txt");
pub const INTU22: &str = include_str!("../data/INTU22.txt");
pub const INTU23: &str = include_str!("../data/INTU23.txt");
pub const INTU24: &str = include_str!("../data/INTU24.txt");
pub const INTU25: &str = include_str!("../data/INTU25.txt");
pub const INTU26: &str = include_str!("../data/INTU26.txt");
pub const INTU27: &str = include_str!("../data/INTU27.txt");
pub const INTU28: &str = include_str!("../data/INTU28.txt");
pub const INTU29: &str = include_str!("../data/INTU29.txt");
pub const INTU30: &str = include_str!("../data/INTU30.txt");
pub const INTU31: &str = include_str!("../data/INTU31.txt");
pub const INTU32: &str = include_str!("../data/INTU32.txt");
pub const INTU33: &str = include_str!("../data/INTU33.txt");
pub const INTU34: &str = include_str!("../data/INTU34.txt");
pub const INTU36: &str = include_str!("../data/INTU36.txt");

pub fn get_data_from_group(group: usize) -> Result<IntuGroup, String> {
    let data = match group {
        1 => INTU01,
        2 => INTU02,
        3 => INTU03,
        4 => INTU04,
        5 => INTU05,
        6 => INTU06,
        7 => INTU07,
        8 => INTU08,
        9 => INTU09,
        10 => INTU10,
        11 => INTU11,
        12 => INTU12,
        13 => INTU13,
        14 => INTU14,
        15 => INTU15,
        16 => INTU16,
        17 => INTU17,
        18 => INTU18,
        20 => INTU20,
        21 => INTU21,
        22 => INTU22,
        23 => INTU23,
        24 => INTU24,
        25 => INTU25,
        26 => INTU26,
        27 => INTU27,
        28 => INTU28,
        29 => INTU29,
        30 => INTU30,
        31 => INTU31,
        32 => INTU32,
        33 => INTU33,
        34 => INTU34,
        36 => INTU36,
        _ => return Err("El grupo no existe".to_string()),
    };

    parse_file_data(data)
}

pub struct IntuGroup {
    title: String,
    group_number: usize,
    questions: Vec<Question>,
}

impl IntuGroup {
    pub fn title(&self) -> &str {
        self.title.as_ref()
    }

    pub fn group_number(&self) -> usize {
        self.group_number
    }

    pub fn questions(&self) -> &[Question] {
        self.questions.as_ref()
    }
}

pub fn parse_file_data(data: &str) -> Result<IntuGroup, String> {
    let mut lines = data.lines();

    let title = lines.next().unwrap().trim();
    let grupo = lines.next().unwrap().trim();
    let answers_unparsed = lines.next().unwrap().trim();

    let answers = {
        let mut vec: Vec<usize> = Vec::new();

        for c in answers_unparsed.chars() {
            vec.push(option_to_index(c).unwrap());
        }

        vec
    };

    let mut parsed = String::new();

    for line in lines {
        parsed.push_str(line);
        parsed.push(' ');
    }

    let parsed = parsed.replace("Pregunta", "\nPregunta");
    let parsed = parsed.trim();

    let mut preguntas = Vec::new();

    for line in parsed.lines() {
        #[allow(unused_assignments)]
        let mut resto = "";

        let mut iter = line.split("a)");
        let enunciado = iter.next().unwrap();
        let enunciado = enunciado.split(':').nth(1).unwrap().trim();
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

        let pregunta = Question::new(
            enunciado.to_string(),
            [
                a.trim().to_string(),
                b.trim().to_string(),
                c.trim().to_string(),
                d.trim().to_string(),
            ],
            4,
        );

        preguntas.push(pregunta);
    }

    for (i, preguntas) in preguntas.iter_mut().enumerate() {
        preguntas.set_correct(answers[i])
    }

    Ok(IntuGroup {
        title: title.to_string(),
        group_number: grupo.parse().unwrap(),
        questions: preguntas,
    })
}
