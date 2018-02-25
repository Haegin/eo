extern crate clap;

use clap::{App, Arg};
// use std::env;

fn main() {
  let matches = app().get_matches();
  println!("{}", echo(matches));
}

fn app() -> clap::App<'static, 'static> {
  return App::new("eo")
    .version("0.1")
    .about("write arguments to the standard output")
    .author("Harry Mills")
    .arg(Arg::with_name("no-newline")
      .short("n")
      .long("no-newline")
      .help("Do not print the trailing newline character."))
    .arg(Arg::with_name("strings")
      .index(1)
      .multiple(true)
    );
}

fn echo(matches: clap::ArgMatches) -> String {
  let strings: Vec<&str> = matches
    .values_of("strings")
    .unwrap()
    .collect();
  return strings.join(" ") + "\n";
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test_print_basic_input() {
    let input = app().get_matches_from(vec!["eo", "Hello", "World"]);
    assert_eq!(echo(input), "Hello World\n".to_string());
  }

  #[test]
  fn test_print_longer_input() {
    let input = app().get_matches_from(vec!["eo", "Goodbye,", "Cruel World"]);
    assert_eq!(echo(input), "Goodbye, Cruel World\n".to_string());
  }

  // #[test]
  // fn test_print_longer_input() {
  //   let input = vec!["Goodbye,".to_string(), "Cruel World".to_string()];
  //   assert_eq!(echo(input), "Goodbye, Cruel World\n".to_string());
  // }
}
