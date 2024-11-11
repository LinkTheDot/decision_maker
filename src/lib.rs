use clap::{Arg, Command};
use itertools::Itertools;
use rand_distr::{Distribution, WeightedIndex};
use std::fs;

const DEFAULT_PATH: &str = "./items";
const TEST_DATA: &str = "a = 100\nb = 50\nc = 25";

#[inline(always)]
pub fn run() -> anyhow::Result<()> {
  let args = get_args();
  let list = get_list(&args)?;

  if list.is_empty() {
    println!("Attempted to parse an empty list of items.");
  }

  if graph_check(&list, &args) {
    return Ok(());
  }

  let mut rng = rand::thread_rng();

  let weights: Vec<&u64> = list.iter().map(|(_, w)| w).collect();
  let dist = WeightedIndex::new(weights)?;
  let chosen_item_index = dist.sample(&mut rng);

  if let Some(item_name) = list.get(chosen_item_index).map(|(n, _)| n) {
    println!();
    println!("The chosen item was `{item_name}`");
    println!();
  }

  Ok(())
}

#[inline(always)]
fn get_list(args: &clap::ArgMatches) -> anyhow::Result<Vec<(String, u64)>> {
  let path = args
    .get_one::<String>("file")
    .cloned()
    .unwrap_or(DEFAULT_PATH.to_string());
  let file_data = if !cfg!(test) {
    fs::read_to_string(path)?
  } else {
    TEST_DATA.to_string()
  };

  // Turned into a Map because of how the data is formatted. (item = weight)
  let list: std::collections::HashMap<String, u64> = toml::from_str(&file_data)?;
  let list: Vec<(String, u64)> = list.into_iter().collect();
  let list_group = list.into_iter().chunk_by(|(key, _)| key.clone());
  let aggregated_list = list_group
    .into_iter()
    .map(|(key, group)| (key, group.map(|(_, value)| value).sum::<u64>()))
    .collect();

  Ok(aggregated_list)
}

fn graph_check(list: &[(String, u64)], args: &clap::ArgMatches) -> bool {
  if args.get_flag("graph") {
    let mut sum = 0;

    for (item_name, weight) in list.iter() {
      println!("`{item_name}`: `{weight}`");

      sum += weight;
    }

    println!();
    println!("Sum: {sum}");

    return true;
  }

  false
}

fn get_args() -> clap::ArgMatches {
  Command::new("Decision Maker")
    .arg(
      Arg::new("file")
        .short('f')
        .long("file")
        .action(clap::ArgAction::Set)
        .help("Tells the program which file to use."),
    )
    .arg(
      Arg::new("graph")
        .short('g')
        .long("graph")
        .action(clap::ArgAction::SetTrue)
        .help("Prints the values of the given data."),
    )
    .get_matches()
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn expected_list() {
    let data = get_list(&get_args()).unwrap();

    let expected_data = vec![("a".into(), 100), ("b".into(), 50), ("c".into(), 25)];

    assert_eq!(data, expected_data);
  }
}
