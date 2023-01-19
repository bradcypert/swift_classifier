use std::{collections::{HashMap, HashSet}, fs::File};
use std::io::Read;

extern crate regex;
use regex::Regex;

struct Classifier {
    pub tokens: HashSet<String>,
    pub tokens_country: HashMap<String, i32>,
    pub tokens_pop: HashMap<String, i32>,
}

impl Classifier {

    pub fn new() -> Classifier {
        Classifier {
            tokens: HashSet::new(),
            tokens_country: HashMap::new(),
            tokens_pop: HashMap::new(),
        }                               
    }

    pub fn train(&mut self, file_name: &str) -> std::io::Result<()> {
        let file = File::open(file_name);

        match file {
            Ok(mut file) => {
                let mut contents = String::new();
                file.read_to_string(&mut contents)?;

                for token in tokenize(&contents) {
                    self.tokens.insert(token.to_string());

                    if file_name.ends_with("swift_country.txt") {
                        *self.tokens_country.entry(token.to_string()).or_insert(0) += 1;
                    } else {
                        *self.tokens_pop.entry(token.to_string()).or_insert(0) += 1;
                    }
                }
                Ok(())

        },
            Err(_) => panic!("Unable to open file")
        }
    }

    pub fn predict(&self, input: &str) -> String {
        let lower_input = input.to_lowercase();
        let input_tokens = tokenize(&lower_input);
        let (prob_pop, prob_country) = self.prob_of_tokens(input_tokens);
        
        let country_likeliness = prob_country / (prob_country + prob_pop);
        let pop_likeliness = prob_pop / (prob_pop + prob_country);

        println!("country: {:?} || pop: {:?}", country_likeliness, pop_likeliness);

        if pop_likeliness < country_likeliness {
            "Country".to_string()
        } else {
            "Pop".to_string()
        }
    }

    fn prob_of_tokens(&self, tokens: Vec<String>) -> (f64, f64) {
       
        let total_words_pop = self.tokens_pop.iter().count() as f64;
        let total_words_country = self.tokens_country.iter().count() as f64;
        let total_unique_words = total_words_country + total_words_pop;

        let mut word_is_pop = 1f64;
        let mut word_is_country = 1f64;

        for token in tokens.iter() {
            let token_pop_count = self.tokens_pop.get(token).unwrap_or(&0);
            let token_country_count = self.tokens_country.get(token).unwrap_or(&0);

            word_is_pop *= (token_pop_count+1) as f64 / (total_words_pop + total_unique_words);
            word_is_country *= (token_country_count+1) as f64 / (total_words_country + total_unique_words);
        }

        (word_is_pop, word_is_country)
    }
}

fn tokenize(input: &str) -> Vec<String> {
    let lowercase = &input.to_lowercase()[..];
    Regex::new(r"[a-z]+")
        .unwrap()
        .find_iter(lowercase)
        .map(|e| e.as_str().to_owned())
        .collect()
}

fn main() -> std::io::Result<()> {
    let mut classifier = Classifier::new(); 
    classifier.train("./src/swift_country.txt")?;
    classifier.train("./src/swift_pop.txt")?;

    // Garth Brooks
    println!("{}", classifier.predict("Blame it all on my roots, I showed up in boots And ruined your black tie affair. The last one to know, the last one to show. I was the last one you thought you'd see there"));

    // Taylor Swift
    println!("{}", classifier.predict("I wanna be your end game. I wanna be your first string. I wanna be your A-Team. I wanna be your end game, end game"));

    Ok(())
}
