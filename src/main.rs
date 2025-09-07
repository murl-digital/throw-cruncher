use std::{fs::File, iter::once, num::ParseFloatError, ops::Not, sync::LazyLock};

use csv::{Reader, Writer};
use regex::Regex;
use serde::Serialize;

#[derive(Debug, Serialize)]
struct Fruit {
    would_throw: bool,
    expected_rancidness: Option<f64>,
    desired_rancidness: Option<f64>,
    notes: String,
}

impl Fruit {
    fn from_iter<'r>(iter: &mut impl Iterator<Item = &'r str>) -> Result<Self, &'static str> {
        let mut notes = String::new();
        let would_throw = parse_bool(iter.next().ok_or("end of row")?)?;
        let expected_rancidness = match best_effort_parse_float(iter.next().ok_or("end of row")?) {
            Ok(FloatNote::Float(f)) => Some(f),
            Ok(FloatNote::FloatNote(f, note)) => {
                notes.push_str(note);
                Some(f)
            }
            Err(note) => {
                notes.push_str(note);

                if note.to_ascii_lowercase().contains("fresh") {
                    // some chuckleheads decided to use the word "fresh" instead of 1 on the scale.
                    // if we see fresh in the string, just assume they meant 1.
                    // it shouldn't mess the data up too bad :)
                    Some(1.0)
                } else {
                    None
                }
            }
        };
        let separator = if notes.is_empty() { "" } else { " | " };
        let desired_rancidness = match best_effort_parse_float(iter.next().ok_or("end of row")?) {
            Ok(FloatNote::Float(f)) => Some(f),
            Ok(FloatNote::FloatNote(f, note)) => {
                notes.push_str(separator);
                notes.push_str(note);
                Some(f)
            }
            Err(note) => {
                notes.push_str(separator);
                notes.push_str(note);

                if note.to_ascii_lowercase().contains("fresh") {
                    // some chuckleheads decided to use the word "fresh" instead of 1 on the scale.
                    // if we see fresh in the string, just assume they meant 1.
                    // it shouldn't mess the data up too bad :)
                    Some(1.0)
                } else {
                    None
                }
            }
        };
        Ok(Self {
            would_throw,
            expected_rancidness,
            desired_rancidness,
            notes,
        })
    }

    fn massage(self) -> Self {
        Self {
            would_throw: self.would_throw,
            expected_rancidness: self.expected_rancidness.map(|f| f.clamp(1.0, 5.0)),
            desired_rancidness: self.desired_rancidness.map(|f| f.clamp(1.0, 5.0)),
            notes: self.notes,
        }
    }
}

fn parse_bool(input: &str) -> Result<bool, &'static str> {
    match input {
        "Yes" => Ok(true),
        "No" => Ok(false),
        _ => Err(format!("malformed bool: {input}").leak()),
    }
}

enum FloatNote<'n> {
    Float(f64),
    FloatNote(f64, &'n str),
}

fn best_effort_parse_float<'n>(input: &'n str) -> Result<FloatNote<'n>, &'n str> {
    static REGEX: LazyLock<Regex> =
        LazyLock::new(|| Regex::new(r"([-]?[0-9]*\.?,?[0-9]+)").unwrap());

    if let Ok(result) = input.parse() {
        Ok(FloatNote::Float(result))
    } else if let Some(captures) = REGEX.captures(input) {
        let capture = captures.get(0).unwrap();
        Ok(FloatNote::FloatNote(
            capture.as_str().parse().unwrap(),
            input,
        ))
    } else {
        Err(input)
    }
}

#[derive(Debug, Serialize)]
struct Response {
    artichoke: Fruit,
    avocado: Fruit,
    banana: Fruit,
    brussels_sprout: Fruit,
    cantaloupe: Fruit,
    cauliflower: Fruit,
    chard: Fruit,
    crimini_mushroom: Fruit,
    golden_beet: Fruit,
    jalapeno: Fruit,
    kiwi: Fruit,
    korean_melon: Fruit,
    lime: Fruit,
    pear: Fruit,
    plucot: Fruit,
    red_grapefruit: Fruit,
    red_onion: Fruit,
    straightneck_squash: Fruit,
    strawberry: Fruit,
    tomatillo: Fruit,
}

#[derive(Debug, Serialize)]
struct FlattenedResponse {
    artichoke_would_throw: bool,
    artichoke_expected_rancidness: Option<f64>,
    artichoke_desired_rancidness: Option<f64>,
    avocado_would_throw: bool,
    avocado_expected_rancidness: Option<f64>,
    avocado_desired_rancidness: Option<f64>,
    banana_would_throw: bool,
    banana_expected_rancidness: Option<f64>,
    banana_desired_rancidness: Option<f64>,
    brussels_sprout_would_throw: bool,
    brussels_sprout_expected_rancidness: Option<f64>,
    brussels_sprout_desired_rancidness: Option<f64>,
    cantaloupe_would_throw: bool,
    cantaloupe_expected_rancidness: Option<f64>,
    cantaloupe_desired_rancidness: Option<f64>,
    cauliflower_would_throw: bool,
    cauliflower_expected_rancidness: Option<f64>,
    cauliflower_desired_rancidness: Option<f64>,
    chard_would_throw: bool,
    chard_expected_rancidness: Option<f64>,
    chard_desired_rancidness: Option<f64>,
    crimini_mushroom_would_throw: bool,
    crimini_mushroom_expected_rancidness: Option<f64>,
    crimini_mushroom_desired_rancidness: Option<f64>,
    golden_beet_would_throw: bool,
    golden_beet_expected_rancidness: Option<f64>,
    golden_beet_desired_rancidness: Option<f64>,
    jalapeno_would_throw: bool,
    jalapeno_expected_rancidness: Option<f64>,
    jalapeno_desired_rancidness: Option<f64>,
    kiwi_would_throw: bool,
    kiwi_expected_rancidness: Option<f64>,
    kiwi_desired_rancidness: Option<f64>,
    korean_melon_would_throw: bool,
    korean_melon_expected_rancidness: Option<f64>,
    korean_melon_desired_rancidness: Option<f64>,
    lime_would_throw: bool,
    lime_expected_rancidness: Option<f64>,
    lime_desired_rancidness: Option<f64>,
    pear_would_throw: bool,
    pear_expected_rancidness: Option<f64>,
    pear_desired_rancidness: Option<f64>,
    plucot_would_throw: bool,
    plucot_expected_rancidness: Option<f64>,
    plucot_desired_rancidness: Option<f64>,
    red_grapefruit_would_throw: bool,
    red_grapefruit_expected_rancidness: Option<f64>,
    red_grapefruit_desired_rancidness: Option<f64>,
    red_onion_would_throw: bool,
    red_onion_expected_rancidness: Option<f64>,
    red_onion_desired_rancidness: Option<f64>,
    straightneck_squash_would_throw: bool,
    straightneck_squash_expected_rancidness: Option<f64>,
    straightneck_squash_desired_rancidness: Option<f64>,
    strawberry_would_throw: bool,
    strawberry_expected_rancidness: Option<f64>,
    strawberry_desired_rancidness: Option<f64>,
    tomatillo_would_throw: bool,
    tomatillo_expected_rancidness: Option<f64>,
    tomatillo_desired_rancidness: Option<f64>,
}

impl FlattenedResponse {
    fn map(response: &Response) -> Self {
        Self {
            artichoke_would_throw: response.artichoke.would_throw,
            artichoke_expected_rancidness: response.artichoke.expected_rancidness,
            artichoke_desired_rancidness: response.artichoke.desired_rancidness,
            avocado_would_throw: response.avocado.would_throw,
            avocado_expected_rancidness: response.avocado.expected_rancidness,
            avocado_desired_rancidness: response.avocado.desired_rancidness,
            banana_would_throw: response.banana.would_throw,
            banana_expected_rancidness: response.banana.expected_rancidness,
            banana_desired_rancidness: response.banana.desired_rancidness,
            brussels_sprout_would_throw: response.brussels_sprout.would_throw,
            brussels_sprout_expected_rancidness: response.brussels_sprout.expected_rancidness,
            brussels_sprout_desired_rancidness: response.brussels_sprout.desired_rancidness,
            cantaloupe_would_throw: response.cantaloupe.would_throw,
            cantaloupe_expected_rancidness: response.cantaloupe.expected_rancidness,
            cantaloupe_desired_rancidness: response.cantaloupe.desired_rancidness,
            cauliflower_would_throw: response.cauliflower.would_throw,
            cauliflower_expected_rancidness: response.cauliflower.expected_rancidness,
            cauliflower_desired_rancidness: response.cauliflower.desired_rancidness,
            chard_would_throw: response.chard.would_throw,
            chard_expected_rancidness: response.chard.expected_rancidness,
            chard_desired_rancidness: response.chard.desired_rancidness,
            crimini_mushroom_would_throw: response.crimini_mushroom.would_throw,
            crimini_mushroom_expected_rancidness: response.crimini_mushroom.expected_rancidness,
            crimini_mushroom_desired_rancidness: response.crimini_mushroom.desired_rancidness,
            golden_beet_would_throw: response.golden_beet.would_throw,
            golden_beet_expected_rancidness: response.golden_beet.expected_rancidness,
            golden_beet_desired_rancidness: response.golden_beet.desired_rancidness,
            jalapeno_would_throw: response.jalapeno.would_throw,
            jalapeno_expected_rancidness: response.jalapeno.expected_rancidness,
            jalapeno_desired_rancidness: response.jalapeno.desired_rancidness,
            kiwi_would_throw: response.kiwi.would_throw,
            kiwi_expected_rancidness: response.kiwi.expected_rancidness,
            kiwi_desired_rancidness: response.kiwi.desired_rancidness,
            korean_melon_would_throw: response.korean_melon.would_throw,
            korean_melon_expected_rancidness: response.korean_melon.expected_rancidness,
            korean_melon_desired_rancidness: response.korean_melon.desired_rancidness,
            lime_would_throw: response.lime.would_throw,
            lime_expected_rancidness: response.lime.expected_rancidness,
            lime_desired_rancidness: response.lime.desired_rancidness,
            pear_would_throw: response.pear.would_throw,
            pear_expected_rancidness: response.pear.expected_rancidness,
            pear_desired_rancidness: response.pear.desired_rancidness,
            plucot_would_throw: response.plucot.would_throw,
            plucot_expected_rancidness: response.plucot.expected_rancidness,
            plucot_desired_rancidness: response.plucot.desired_rancidness,
            red_grapefruit_would_throw: response.red_grapefruit.would_throw,
            red_grapefruit_expected_rancidness: response.red_grapefruit.expected_rancidness,
            red_grapefruit_desired_rancidness: response.red_grapefruit.desired_rancidness,
            red_onion_would_throw: response.red_onion.would_throw,
            red_onion_expected_rancidness: response.red_onion.expected_rancidness,
            red_onion_desired_rancidness: response.red_onion.desired_rancidness,
            straightneck_squash_would_throw: response.straightneck_squash.would_throw,
            straightneck_squash_expected_rancidness: response
                .straightneck_squash
                .expected_rancidness,
            straightneck_squash_desired_rancidness: response.straightneck_squash.desired_rancidness,
            strawberry_would_throw: response.strawberry.would_throw,
            strawberry_expected_rancidness: response.strawberry.expected_rancidness,
            strawberry_desired_rancidness: response.strawberry.desired_rancidness,
            tomatillo_would_throw: response.tomatillo.would_throw,
            tomatillo_expected_rancidness: response.tomatillo.expected_rancidness,
            tomatillo_desired_rancidness: response.tomatillo.desired_rancidness,
        }
    }
}

#[derive(Debug, Serialize)]
struct FlattenedReport {
    artichoke_would_throw_count: usize,
    artichoke_would_not_throw_count: usize,
    artichoke_average_expected_rancidness: f64,
    artichoke_average_desired_rancidness: f64,
    avocado_would_throw_count: usize,
    avocado_would_not_throw_count: usize,
    avocado_average_expected_rancidness: f64,
    avocado_average_desired_rancidness: f64,
    banana_would_throw_count: usize,
    banana_would_not_throw_count: usize,
    banana_average_expected_rancidness: f64,
    banana_average_desired_rancidness: f64,
    brussels_sprout_would_throw_count: usize,
    brussels_sprout_would_not_throw_count: usize,
    brussels_sprout_average_expected_rancidness: f64,
    brussels_sprout_average_desired_rancidness: f64,
    cantaloupe_would_throw_count: usize,
    cantaloupe_would_not_throw_count: usize,
    cantaloupe_average_expected_rancidness: f64,
    cantaloupe_average_desired_rancidness: f64,
    cauliflower_would_throw_count: usize,
    cauliflower_would_not_throw_count: usize,
    cauliflower_average_expected_rancidness: f64,
    cauliflower_average_desired_rancidness: f64,
    chard_would_throw_count: usize,
    chard_would_not_throw_count: usize,
    chard_average_expected_rancidness: f64,
    chard_average_desired_rancidness: f64,
    crimini_mushroom_would_throw_count: usize,
    crimini_mushroom_would_not_throw_count: usize,
    crimini_mushroom_average_expected_rancidness: f64,
    crimini_mushroom_average_desired_rancidness: f64,
    golden_beet_would_throw_count: usize,
    golden_beet_would_not_throw_count: usize,
    golden_beet_average_expected_rancidness: f64,
    golden_beet_average_desired_rancidness: f64,
    jalapeno_would_throw_count: usize,
    jalapeno_would_not_throw_count: usize,
    jalapeno_average_expected_rancidness: f64,
    jalapeno_average_desired_rancidness: f64,
    kiwi_would_throw_count: usize,
    kiwi_would_not_throw_count: usize,
    kiwi_average_expected_rancidness: f64,
    kiwi_average_desired_rancidness: f64,
    korean_melon_would_throw_count: usize,
    korean_melon_would_not_throw_count: usize,
    korean_melon_average_expected_rancidness: f64,
    korean_melon_average_desired_rancidness: f64,
    lime_would_throw_count: usize,
    lime_would_not_throw_count: usize,
    lime_average_expected_rancidness: f64,
    lime_average_desired_rancidness: f64,
    pear_would_throw_count: usize,
    pear_would_not_throw_count: usize,
    pear_average_expected_rancidness: f64,
    pear_average_desired_rancidness: f64,
    plucot_would_throw_count: usize,
    plucot_would_not_throw_count: usize,
    plucot_average_expected_rancidness: f64,
    plucot_average_desired_rancidness: f64,
    red_grapefruit_would_throw_count: usize,
    red_grapefruit_would_not_throw_count: usize,
    red_grapefruit_average_expected_rancidness: f64,
    red_grapefruit_average_desired_rancidness: f64,
    red_onion_would_throw_count: usize,
    red_onion_would_not_throw_count: usize,
    red_onion_average_expected_rancidness: f64,
    red_onion_average_desired_rancidness: f64,
    straightneck_squash_would_throw_count: usize,
    straightneck_squash_would_not_throw_count: usize,
    straightneck_squash_average_expected_rancidness: f64,
    straightneck_squash_average_desired_rancidness: f64,
    strawberry_would_throw_count: usize,
    strawberry_would_not_throw_count: usize,
    strawberry_average_expected_rancidness: f64,
    strawberry_average_desired_rancidness: f64,
    tomatillo_would_throw_count: usize,
    tomatillo_would_not_throw_count: usize,
    tomatillo_average_expected_rancidness: f64,
    tomatillo_average_desired_rancidness: f64,
}

impl FlattenedReport {
    fn from_vec_response(vec_response: VecResponse) -> Self {
        let (
            artichoke_would_throw_count,
            artichoke_would_not_throw_count,
            artichoke_average_expected_rancidness,
            artichoke_average_desired_rancidness,
        ) = report(&vec_response.artichoke);
        let (
            avocado_would_throw_count,
            avocado_would_not_throw_count,
            avocado_average_expected_rancidness,
            avocado_average_desired_rancidness,
        ) = report(&vec_response.avocado);
        let (
            banana_would_throw_count,
            banana_would_not_throw_count,
            banana_average_expected_rancidness,
            banana_average_desired_rancidness,
        ) = report(&vec_response.banana);
        let (
            brussels_sprout_would_throw_count,
            brussels_sprout_would_not_throw_count,
            brussels_sprout_average_expected_rancidness,
            brussels_sprout_average_desired_rancidness,
        ) = report(&vec_response.brussels_sprout);
        let (
            cantaloupe_would_throw_count,
            cantaloupe_would_not_throw_count,
            cantaloupe_average_expected_rancidness,
            cantaloupe_average_desired_rancidness,
        ) = report(&vec_response.cantaloupe);
        let (
            cauliflower_would_throw_count,
            cauliflower_would_not_throw_count,
            cauliflower_average_expected_rancidness,
            cauliflower_average_desired_rancidness,
        ) = report(&vec_response.cauliflower);
        let (
            chard_would_throw_count,
            chard_would_not_throw_count,
            chard_average_expected_rancidness,
            chard_average_desired_rancidness,
        ) = report(&vec_response.chard);
        let (
            crimini_mushroom_would_throw_count,
            crimini_mushroom_would_not_throw_count,
            crimini_mushroom_average_expected_rancidness,
            crimini_mushroom_average_desired_rancidness,
        ) = report(&vec_response.crimini_mushroom);
        let (
            golden_beet_would_throw_count,
            golden_beet_would_not_throw_count,
            golden_beet_average_expected_rancidness,
            golden_beet_average_desired_rancidness,
        ) = report(&vec_response.golden_beet);
        let (
            jalapeno_would_throw_count,
            jalapeno_would_not_throw_count,
            jalapeno_average_expected_rancidness,
            jalapeno_average_desired_rancidness,
        ) = report(&vec_response.jalapeno);
        let (
            kiwi_would_throw_count,
            kiwi_would_not_throw_count,
            kiwi_average_expected_rancidness,
            kiwi_average_desired_rancidness,
        ) = report(&vec_response.kiwi);
        let (
            korean_melon_would_throw_count,
            korean_melon_would_not_throw_count,
            korean_melon_average_expected_rancidness,
            korean_melon_average_desired_rancidness,
        ) = report(&vec_response.korean_melon);
        let (
            lime_would_throw_count,
            lime_would_not_throw_count,
            lime_average_expected_rancidness,
            lime_average_desired_rancidness,
        ) = report(&vec_response.lime);
        let (
            pear_would_throw_count,
            pear_would_not_throw_count,
            pear_average_expected_rancidness,
            pear_average_desired_rancidness,
        ) = report(&vec_response.pear);
        let (
            plucot_would_throw_count,
            plucot_would_not_throw_count,
            plucot_average_expected_rancidness,
            plucot_average_desired_rancidness,
        ) = report(&vec_response.plucot);
        let (
            red_grapefruit_would_throw_count,
            red_grapefruit_would_not_throw_count,
            red_grapefruit_average_expected_rancidness,
            red_grapefruit_average_desired_rancidness,
        ) = report(&vec_response.red_grapefruit);
        let (
            red_onion_would_throw_count,
            red_onion_would_not_throw_count,
            red_onion_average_expected_rancidness,
            red_onion_average_desired_rancidness,
        ) = report(&vec_response.red_onion);
        let (
            straightneck_squash_would_throw_count,
            straightneck_squash_would_not_throw_count,
            straightneck_squash_average_expected_rancidness,
            straightneck_squash_average_desired_rancidness,
        ) = report(&vec_response.straightneck_squash);
        let (
            strawberry_would_throw_count,
            strawberry_would_not_throw_count,
            strawberry_average_expected_rancidness,
            strawberry_average_desired_rancidness,
        ) = report(&vec_response.strawberry);
        let (
            tomatillo_would_throw_count,
            tomatillo_would_not_throw_count,
            tomatillo_average_expected_rancidness,
            tomatillo_average_desired_rancidness,
        ) = report(&vec_response.tomatillo);

        Self {
            artichoke_would_throw_count,
            artichoke_would_not_throw_count,
            artichoke_average_expected_rancidness,
            artichoke_average_desired_rancidness,
            avocado_would_throw_count,
            avocado_would_not_throw_count,
            avocado_average_expected_rancidness,
            avocado_average_desired_rancidness,
            banana_would_throw_count,
            banana_would_not_throw_count,
            banana_average_expected_rancidness,
            banana_average_desired_rancidness,
            brussels_sprout_would_throw_count,
            brussels_sprout_would_not_throw_count,
            brussels_sprout_average_expected_rancidness,
            brussels_sprout_average_desired_rancidness,
            cantaloupe_would_throw_count,
            cantaloupe_would_not_throw_count,
            cantaloupe_average_expected_rancidness,
            cantaloupe_average_desired_rancidness,
            cauliflower_would_throw_count,
            cauliflower_would_not_throw_count,
            cauliflower_average_expected_rancidness,
            cauliflower_average_desired_rancidness,
            chard_would_throw_count,
            chard_would_not_throw_count,
            chard_average_expected_rancidness,
            chard_average_desired_rancidness,
            crimini_mushroom_would_throw_count,
            crimini_mushroom_would_not_throw_count,
            crimini_mushroom_average_expected_rancidness,
            crimini_mushroom_average_desired_rancidness,
            golden_beet_would_throw_count,
            golden_beet_would_not_throw_count,
            golden_beet_average_expected_rancidness,
            golden_beet_average_desired_rancidness,
            jalapeno_would_throw_count,
            jalapeno_would_not_throw_count,
            jalapeno_average_expected_rancidness,
            jalapeno_average_desired_rancidness,
            kiwi_would_throw_count,
            kiwi_would_not_throw_count,
            kiwi_average_expected_rancidness,
            kiwi_average_desired_rancidness,
            korean_melon_would_throw_count,
            korean_melon_would_not_throw_count,
            korean_melon_average_expected_rancidness,
            korean_melon_average_desired_rancidness,
            lime_would_throw_count,
            lime_would_not_throw_count,
            lime_average_expected_rancidness,
            lime_average_desired_rancidness,
            pear_would_throw_count,
            pear_would_not_throw_count,
            pear_average_expected_rancidness,
            pear_average_desired_rancidness,
            plucot_would_throw_count,
            plucot_would_not_throw_count,
            plucot_average_expected_rancidness,
            plucot_average_desired_rancidness,
            red_grapefruit_would_throw_count,
            red_grapefruit_would_not_throw_count,
            red_grapefruit_average_expected_rancidness,
            red_grapefruit_average_desired_rancidness,
            red_onion_would_throw_count,
            red_onion_would_not_throw_count,
            red_onion_average_expected_rancidness,
            red_onion_average_desired_rancidness,
            straightneck_squash_would_throw_count,
            straightneck_squash_would_not_throw_count,
            straightneck_squash_average_expected_rancidness,
            straightneck_squash_average_desired_rancidness,
            strawberry_would_throw_count,
            strawberry_would_not_throw_count,
            strawberry_average_expected_rancidness,
            strawberry_average_desired_rancidness,
            tomatillo_would_throw_count,
            tomatillo_would_not_throw_count,
            tomatillo_average_expected_rancidness,
            tomatillo_average_desired_rancidness,
        }
    }
}

struct VecResponse {
    artichoke: Vec<Fruit>,
    avocado: Vec<Fruit>,
    banana: Vec<Fruit>,
    brussels_sprout: Vec<Fruit>,
    cantaloupe: Vec<Fruit>,
    cauliflower: Vec<Fruit>,
    chard: Vec<Fruit>,
    crimini_mushroom: Vec<Fruit>,
    golden_beet: Vec<Fruit>,
    jalapeno: Vec<Fruit>,
    kiwi: Vec<Fruit>,
    korean_melon: Vec<Fruit>,
    lime: Vec<Fruit>,
    pear: Vec<Fruit>,
    plucot: Vec<Fruit>,
    red_grapefruit: Vec<Fruit>,
    red_onion: Vec<Fruit>,
    straightneck_squash: Vec<Fruit>,
    strawberry: Vec<Fruit>,
    tomatillo: Vec<Fruit>,
}

impl VecResponse {
    fn from_iter(iter: impl Iterator<Item = Response>) -> Self {
        let mut artichoke = Vec::default();
        let mut avocado = Vec::default();
        let mut banana = Vec::default();
        let mut brussels_sprout = Vec::default();
        let mut cantaloupe = Vec::default();
        let mut cauliflower = Vec::default();
        let mut chard = Vec::default();
        let mut crimini_mushroom = Vec::default();
        let mut golden_beet = Vec::default();
        let mut jalapeno = Vec::default();
        let mut kiwi = Vec::default();
        let mut korean_melon = Vec::default();
        let mut lime = Vec::default();
        let mut pear = Vec::default();
        let mut plucot = Vec::default();
        let mut red_grapefruit = Vec::default();
        let mut red_onion = Vec::default();
        let mut straightneck_squash = Vec::default();
        let mut strawberry = Vec::default();
        let mut tomatillo = Vec::default();

        for response in iter {
            artichoke.push(response.artichoke);
            avocado.push(response.avocado);
            banana.push(response.banana);
            brussels_sprout.push(response.brussels_sprout);
            cantaloupe.push(response.cantaloupe);
            cauliflower.push(response.cauliflower);
            chard.push(response.chard);
            crimini_mushroom.push(response.crimini_mushroom);
            golden_beet.push(response.golden_beet);
            jalapeno.push(response.jalapeno);
            kiwi.push(response.kiwi);
            korean_melon.push(response.korean_melon);
            lime.push(response.lime);
            pear.push(response.pear);
            plucot.push(response.plucot);
            red_grapefruit.push(response.red_grapefruit);
            red_onion.push(response.red_onion);
            straightneck_squash.push(response.straightneck_squash);
            strawberry.push(response.strawberry);
            tomatillo.push(response.tomatillo);
        }

        Self {
            artichoke,
            avocado,
            banana,
            brussels_sprout,
            cantaloupe,
            cauliflower,
            chard,
            crimini_mushroom,
            golden_beet,
            jalapeno,
            kiwi,
            korean_melon,
            lime,
            pear,
            plucot,
            red_grapefruit,
            red_onion,
            straightneck_squash,
            strawberry,
            tomatillo,
        }
    }
}

impl Response {
    fn from_iter<'r>(iter: &mut impl Iterator<Item = &'r str>) -> Result<Self, &'static str> {
        Ok(Self {
            artichoke: Fruit::from_iter(iter)?,
            avocado: Fruit::from_iter(iter)?,
            banana: Fruit::from_iter(iter)?,
            brussels_sprout: Fruit::from_iter(iter)?,
            cantaloupe: Fruit::from_iter(iter)?,
            cauliflower: Fruit::from_iter(iter)?,
            chard: Fruit::from_iter(iter)?,
            crimini_mushroom: Fruit::from_iter(iter)?,
            golden_beet: Fruit::from_iter(iter)?,
            jalapeno: Fruit::from_iter(iter)?,
            kiwi: Fruit::from_iter(iter)?,
            korean_melon: Fruit::from_iter(iter)?,
            lime: Fruit::from_iter(iter)?,
            pear: Fruit::from_iter(iter)?,
            plucot: Fruit::from_iter(iter)?,
            red_grapefruit: Fruit::from_iter(iter)?,
            red_onion: Fruit::from_iter(iter)?,
            straightneck_squash: Fruit::from_iter(iter)?,
            strawberry: Fruit::from_iter(iter)?,
            tomatillo: Fruit::from_iter(iter)?,
        })
    }

    fn massage(self) -> Self {
        Self {
            artichoke: self.artichoke.massage(),
            avocado: self.avocado.massage(),
            banana: self.banana.massage(),
            brussels_sprout: self.brussels_sprout.massage(),
            cantaloupe: self.cantaloupe.massage(),
            cauliflower: self.cauliflower.massage(),
            chard: self.chard.massage(),
            crimini_mushroom: self.crimini_mushroom.massage(),
            golden_beet: self.golden_beet.massage(),
            jalapeno: self.jalapeno.massage(),
            kiwi: self.kiwi.massage(),
            korean_melon: self.korean_melon.massage(),
            lime: self.lime.massage(),
            pear: self.pear.massage(),
            plucot: self.plucot.massage(),
            red_grapefruit: self.red_grapefruit.massage(),
            red_onion: self.red_onion.massage(),
            straightneck_squash: self.straightneck_squash.massage(),
            strawberry: self.strawberry.massage(),
            tomatillo: self.tomatillo.massage(),
        }
    }
}

fn main() {
    let mut reader = Reader::from_path("throwcsv.csv").unwrap();

    let responses = reader
        .records()
        .map(Result::unwrap)
        .map(|r| Response::from_iter(&mut r.iter().skip(3)))
        .collect::<Result<Vec<Response>, _>>()
        .expect("data ingest error");

    serde_json::to_writer_pretty(File::create("result_ingested.json").unwrap(), &responses)
        .unwrap();

    let massaged_responses = responses
        .into_iter()
        .map(Response::massage)
        .collect::<Vec<_>>();

    serde_json::to_writer_pretty(
        File::create("result_massaged.json").unwrap(),
        &massaged_responses,
    )
    .unwrap();

    let mut writer = Writer::from_path("result_massaged.csv").unwrap();
    massaged_responses
        .iter()
        .map(FlattenedResponse::map)
        .for_each(|r| writer.serialize(r).unwrap());

    let flattened_report =
        FlattenedReport::from_vec_response(VecResponse::from_iter(massaged_responses.into_iter()));

    Writer::from_path("result.csv")
        .unwrap()
        .serialize(flattened_report)
        .unwrap();
}

fn report(fruits: &[Fruit]) -> (usize, usize, f64, f64) {
    (
        fruits
            .iter()
            .filter_map(|f| f.would_throw.then_some(()))
            .count(),
        fruits
            .iter()
            .filter_map(|f| f.would_throw.not().then_some(()))
            .count(),
        fruits
            .iter()
            .filter_map(|f| f.expected_rancidness)
            .zip(1..)
            .fold(0.0, |s, (e, i)| (e as f64 + s * (i - 1) as f64) / i as f64),
        fruits
            .iter()
            .filter_map(|f| f.desired_rancidness)
            .zip(1..)
            .fold(0.0, |s, (e, i)| (e as f64 + s * (i - 1) as f64) / i as f64),
    )
}
