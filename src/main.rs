use std::{collections::HashMap, time};

fn init_languages() -> HashMap<String, i32> {
    let mut languages = HashMap::new();
    languages.insert("JavaScript".to_string(), 1995);
    languages.insert("HTML/CSS".to_string(), 1990);
    languages.insert("Python".to_string(), 1991);
    languages.insert("SQL".to_string(), 1974);
    languages.insert("TypeScript".to_string(), 2012);
    languages.insert("Java".to_string(), 1995);
    languages.insert("C#".to_string(), 2000);
    languages.insert("PHP".to_string(), 1995);
    languages.insert("C++".to_string(), 1985);
    languages.insert("C".to_string(), 1972);
    languages.insert("Ruby".to_string(), 1995);
    languages.insert("Swift".to_string(), 2014);
    languages.insert("Objective-C".to_string(), 1984);
    languages.insert("Kotlin".to_string(), 2011);
    languages.insert("Rust".to_string(), 2010);
    languages.insert("Scala".to_string(), 2003);
    languages.insert("Go".to_string(), 2009);
    languages.insert("Perl".to_string(), 1987);
    languages.insert("Haskell".to_string(), 1990);
    languages.insert("Lua".to_string(), 1993);
    languages.insert("R".to_string(), 1993);
    languages.insert("Groovy".to_string(), 2003);
    languages.insert("Shell".to_string(), 1977);
    languages.insert("PowerShell".to_string(), 2006);
    languages.insert("Assembly".to_string(), 1949);
    languages.insert("Dart".to_string(), 2011);
    languages.insert("Clojure".to_string(), 2007);
    languages.insert("Elixir".to_string(), 2011);
    languages.insert("Erlang".to_string(), 1986);
    languages.insert("F#".to_string(), 2005);
    languages.insert("Julia".to_string(), 2012);
    languages.insert("Lisp".to_string(), 1958);
    languages.insert("Prolog".to_string(), 1972);
    languages.insert("Scheme".to_string(), 1975);
    languages.insert("Smalltalk".to_string(), 1972);
    languages.insert("VimL".to_string(), 1991);
    languages.insert("Ada".to_string(), 1980);
    languages.insert("Agda".to_string(), 1999);
    languages.insert("APL".to_string(), 1962);
    languages.insert("AutoHotkey".to_string(), 2003);
    languages.insert("AutoIt".to_string(), 1999);
    languages.insert("ATS".to_string(), 1996);
    languages.insert("Ballerina".to_string(), 2017);
    languages.insert("Batchfile".to_string(), 1980);
    languages.insert("BlitzMax".to_string(), 2001);
    languages.insert("Boo".to_string(), 2003);
    languages.insert("PHP".to_string(), 1995);

    languages
}

fn calculate_weights(years_active: &mut HashMap<String, i32>) -> HashMap<String, i32> {
    //get current year
    let current_year = time::SystemTime::now()
        .duration_since(time::UNIX_EPOCH)
        .unwrap()
        .as_secs() as i32;

    for year in years_active.values_mut() {
        *year = current_year - *year;
    }

    let min_year = years_active.values().min().unwrap_or(&0);
    let max_year = years_active.values().max().unwrap_or(&0);

    let mut weights = HashMap::new();

    for (language, &year) in years_active.iter() {
        let normalized_year = (year - min_year) as f64 / (max_year - min_year) as f64;
        let weight = (normalized_year * 99.0) as i32 + 1;
        weights.insert(language.to_string(), weight);
    }

    weights
}

fn main() {
    let mut languages = init_languages();
    let weights = calculate_weights(&mut languages);
    //sort weights by value
    let mut weights: Vec<_> = weights.into_iter().collect();
    weights.sort_by(|a, b| b.1.cmp(&a.1));

    println!("Language weighing from 1-100 by age (1 is newest, 100 is oldest):");
    for (language, weight) in &weights {
        println!("{}: {}", language, weight);
    }
}
