

use std::collections::HashMap;
use std::time::Instant;

use tokenizer_iterative1::SingleIteratorTokenizer;

pub struct Shell {
    user_config: HashMap<String, String>,
    user_definitions: HashMap<String, String>,
}

impl Shell {
    pub fn new() -> Self {
        Self {
            user_config: HashMap::new(),
            user_definitions: HashMap::new(),
        }
    }

    pub fn process_command(&mut self, command: &str) {
        let mut unprocessed = command;
        match get_next_part(&mut unprocessed) {
            Some("") | None => {}
            Some("status") => println!(r#"Active Tasks: 0"#),
            Some("delete") => {
                while let Some(name) = get_next_part(&mut unprocessed) {
                    println!("Cleared definition: {}", name);
                    self.user_definitions.remove(name);
                }
            }
            Some("var") => {
                for (name, value) in &self.user_definitions {
                    println!("{} =  {}", name, value);
                }
            }
            Some("set") => {
                let config_name = get_next_part(&mut unprocessed);
                if config_name.is_none() {
                    println!("Error: missing name for set command");
                    return;
                }

                let config_name = config_name.unwrap();
                if unprocessed.len() > 0 {
                    self.user_config
                        .insert(config_name.into(), unprocessed.into());
                    println!("Config: Set {}", config_name);
                } else {
                    match self.user_config.remove(config_name) {
                        Some(_) => println!("Config: Removed {}", config_name),
                        None => println!("Config: {} was not set", config_name),
                    }
                }
            }
            Some("unset") => {
                let name = get_next_part(&mut unprocessed);
                if name.is_none() {
                    println!("Error: missing name for unset command");
                    return;
                }
                self.user_config
                    .insert(name.unwrap().into(), unprocessed.into());
            }
            Some("help") => println!(
                r#"
Available Commands:

  help                      Show this help message
  exit                      Exit the prompt
  delete [name]             Deletes a variable with the given name
  set [config] (value)      Set (or unsets) a configuration variable
  vars [name]               Show current definitions. Options:
                              --user   Show the user definitions (default)
                              --system Show the system definitions
                              --all    Show all definitions
  [name] = [expression]     Creates a definition with the given name
  [expression]              Start a task with the expression, display the results when ready.
"#
            ),
            Some(variable_name) => {
                if let Some("=") = get_next_part(&mut unprocessed) {
                    if unprocessed.is_empty() {
                        println!("Error: missing value for =");
                        return;
                    }

                    self.user_definitions
                        .insert(variable_name.into(), unprocessed.into());
                } else {
                    let collect_tokenization_statistics = true;

                    let tokens = if collect_tokenization_statistics {
                        let warmup_count = 500;
                        let measure_count = 2000;
                        let mut measurements = Vec::with_capacity(measure_count);

                        let tokens = {
                            let mut tokens_vec = Vec::with_capacity(measure_count + warmup_count);
                            for _ in 0..warmup_count {
                                tokens_vec.push(SingleIteratorTokenizer::from_slice(command.as_bytes())
                                    .map(|token| std::str::from_utf8(token).unwrap())
                                    .collect::<Vec<_>>());
                            }

                            for _ in 0..measure_count {
                                let start = Instant::now();
                                tokens_vec.push(SingleIteratorTokenizer::from_slice(command.as_bytes())
                                    .map(|token| std::str::from_utf8(token).unwrap())
                                    .collect::<Vec<_>>());
                                measurements.push(start.elapsed());
                            }
                            tokens_vec.pop().unwrap()
                        };

                        let token_count = tokens.len();
                        let byte_count = command.as_bytes().len();
                        let total_ns = measurements.iter()
                            .map(|d| d.as_nanos())
                            .sum::<u128>();
                        let min_ns = measurements.iter()
                            .map(|d| d.as_nanos())
                            .min()
                            .unwrap();
                        let max_ns = measurements.iter()
                            .map(|d| d.as_nanos())
                            .max()
                            .unwrap();
                        let average_ns: f64 = total_ns as f64 / measurements.len() as f64;

                        let ops_per_second: f64 = measure_count as f64 * 1_000_000_000f64 / total_ns as f64;

                        let average_tokens_ns = average_ns / token_count as f64;
                        let min_tokens_ns = min_ns as f64 / token_count as f64;
                        let max_tokens_ns = max_ns as f64 / token_count as f64;
                        let tokens_per_second: f64 = ops_per_second * token_count as f64;

                        let average_bytes_ns = average_ns / byte_count as f64;
                        let min_bytes_ns = min_ns as f64 / byte_count as f64;
                        let max_bytes_ns = max_ns as f64 / byte_count as f64;
                        let bytes_per_second: f64 = ops_per_second * byte_count as f64;

                        println!(
                            r#"
    /------------------ Tokenization Performance ------------------\
    |                 |          Ops |       Tokens |        Bytes |
    |--------------------------------------------------------------|
    |           Count |            1 | {:>12} | {:>12} |
    |        Min (ns) | {:>12.3} | {:>12.3} | {:>12.3} |
    |        Max (ns) | {:>12.3} | {:>12.3} | {:>12.3} |
    |--------------------------------------------------------------|
    |    Average (ns) | {:>12.3} | {:>12.3} | {:>12.3} |
    |    Rate (/sec)  | {:>12.1} | {:>12.1} | {:>12.1} |
    \--------------------------------------------------------------/
     "#,
                            token_count, byte_count,
                            min_ns, min_tokens_ns, min_bytes_ns,
                            max_ns, max_tokens_ns, max_bytes_ns,
                            average_ns, average_tokens_ns, average_bytes_ns,
                            ops_per_second, bytes_per_second, tokens_per_second
                        );
                        tokens
                    } else {
                        SingleIteratorTokenizer::from_slice(command.as_bytes())
                            .map(|token| std::str::from_utf8(token).unwrap())
                            .collect::<Vec<_>>()
                    };

                    println!("Tokens: {:?}", tokens)
                }
            }
        }
    }
}

fn get_next_part<'a, 'b>(unprocessed: &'a mut &'b str) -> Option<&'b str> {
    let mut unprocessed_iter = unprocessed.chars();
    let start = unprocessed_iter.position(|c| !c.is_whitespace())?;

    let end = unprocessed_iter
        .position(|c| c.is_whitespace())
        .map(|end| start + end + 1);

    let part_end = end.unwrap_or(unprocessed.len());
    let unprocessed_start = end.map(|i| i + 1).unwrap_or(unprocessed.len());

    let part = &unprocessed[start..part_end];
    *unprocessed = &unprocessed[unprocessed_start..];
    Some(part)
}
