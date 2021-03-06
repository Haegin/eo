extern crate clap;

use clap::{App, Arg};
use std::io::{self, Write};
// use std::env;

fn main() {
  let matches = app().get_matches();
  print!("{}", echo(matches));
  io::stdout().flush().unwrap();
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
  let mut print_newline: bool = !matches.is_present("no-newline");
  let strings: Vec<&str> = matches
    .values_of("strings")
    .unwrap()
    .collect();
  let mut output: String = strings.join(" ");
  if output.ends_with("\\c") {
    let c_pos = output.rfind("\\c");
    output.truncate(c_pos.unwrap());
    print_newline = false;
  }
  if print_newline {
    output.push_str("\n");
  }
  return output;
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

  #[test]
  fn test_n_command_line_arg() {
    let input = app().get_matches_from(vec!["eo", "-n", "Goodbye,", "Cruel World"]);
    assert_eq!(echo(input), "Goodbye, Cruel World".to_string());
  }

  #[test]
  fn test_backslash_c_at_end_removes_newline() {
    let input = app().get_matches_from(vec!["eo", "Hello\\c"]);
    assert_eq!(echo(input), "Hello".to_string());
  }

  #[test]
  fn test_backslash_c_in_middle_is_unchanged() {
    let input = app().get_matches_from(vec!["eo", "Hello\\c", "World"]);
    assert_eq!(echo(input), "Hello\\c World\n".to_string());
  }
}
