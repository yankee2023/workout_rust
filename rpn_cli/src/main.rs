use std::fs::File;
use std::io::{stdin, BufRead, BufReader};
use std::path::PathBuf;
use clap::Parser;
use anyhow::{bail, ensure, Context, Result};

/// 逆ポーランド記法を計算する構造体
struct RpnCalculator(bool);

#[derive(Parser, Debug)]
#[clap(name = "My RPN program",
       version = "1.0.0",
       author = "Your Name",
       about = "Super awesome sample RPN calculator")]
struct Opts {
    /// Sets the level of verbosity
    #[clap(short, long)]
    verbose :bool,

    /// Formulas written in RPN
    #[clap(name = "FILE")]
    formula_file: Option<PathBuf>,
}

impl RpnCalculator {
    pub fn new(verbose: bool) -> Self {
        Self(verbose)
    }

    pub fn evaluate(&self, formula: &str) -> Result<i32> {
        let mut tokens = formula.split_whitespace().rev().collect::<Vec<_>>();
        self.evaluate_inner(&mut tokens)
    }

    fn evaluate_inner(&self, tokens: &mut Vec<&str>) -> Result<i32> {
        let mut stack = Vec::new();
        let mut pos = 0;

        while let Some(token) = tokens.pop() {
            pos += 1;

            if let Ok(x) = token.parse::<i32>() {
                stack.push(x);
            } else {
                let y = stack.pop().context(format!("invalid syntax at {}", pos))?;
                let x = stack.pop().context(format!("invalid syntax at {}", pos))?;
                let result = match token {
                    "+" => x + y,
                    "-" => x - y,
                    "*" => x * y,
                    "/" => x / y,
                    "%" => x % y,
                    _ => bail!("invalid token at {}", pos),
                };
                stack.push(result);
            }

            // -vオプションが指定されていたら、この時点でのトークンとスタックを出力
            if self.0 {
                println!("{:?} {:?}", tokens, stack);
            }
        }

        ensure!(stack.len() == 1, "invalid syntax");

        Ok(stack[0])
    }
}

fn main() -> Result<()>{
    let opts = Opts::parse();

    if let Some(path) = opts.formula_file {
        let f = File::open(path)?;
        let reader = BufReader::new(f);
        run(reader, opts.verbose)
    } else {
        let stdin = stdin();
        let reader = stdin.lock();
        run(reader, opts.verbose)
    }
}

/// Runs the RPN calculator with the provided reader
/// # Arguments:
/// * `reader`: A reader that provides lines of input
/// * `verbose`: A boolean indicating if verbose output is enabled
fn run<R: BufRead>(reader: R, verbose: bool) -> Result<()> {
    let calc = RpnCalculator::new(verbose);

    for line in reader.lines() {
        let line = line?;
        match calc.evaluate(&line) {
            Ok(answer) => println!("{}", answer),
            Err(e) => eprintln!("Error evaluating '{}': {}", line, e),
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ok() {
        let calc = RpnCalculator::new(false);
        assert_eq!(calc.evaluate(5).unwrap(), 5);
        assert_eq!(calc.evaluate("2 3 +").unwrap(), 5);
        assert_eq!(calc.evaluate("2 3 -").unwrap(), -1);
        assert_eq!(calc.evaluate("2 3 *").unwrap(), 6);
        assert_eq!(calc.evaluate("6 3 /").unwrap(), 2);
        assert_eq!(calc.evaluate("5 2 %").unwrap(), 1);
    }

    #[test]
    fn test_ng() {
        let calc = RpnCalculator::new(false);
        // 逆ポーランド記法の文法に違反している
        assert!(calc.evaluate(""), is_err());
        assert!(calc.evaluate("2 3 + -"), is_err());
        assert!(calc.evaluate("+ * 1 2"), is_err());
    }
}
