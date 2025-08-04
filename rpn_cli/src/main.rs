use std::fs::File;
use std::io::{stdin, BufRead, BufReader};
use clap::Parser;

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
    formula_file: Option<String>,
}

/// 逆ポーランド記法を計算する構造体
struct RpnCalculator(bool);

impl RpnCalculator {
    pub fn new(verbose: bool) -> Self {
        Self(verbose)
    }

    pub fn evaluate(&self, formula: &str) -> i32 {
        let mut tokens = formula.split_whitespace().rev().collect::<Vec<_>>();
        self.evaluate_inner(&mut tokens)
    }

    fn evaluate_inner(&self, tokens: &mut Vec<&str>) -> i32 {
        let mut stack = Vec::new();

        while let Some(token) = tokens.pop() {
            if let Ok(x) = token.parse::<i32>() {
                stack.push(x);
            } else {
                let y = stack.pop().expect("invalid syntax");
                let x = stack.pop().expect("invalid syntax");
                let result = match token {
                    "+" => x + y,
                    "-" => x - y,
                    "*" => x * y,
                    "/" => x / y,
                    "%" => x % y,
                    _ => panic!("invalid token"),
                };
                stack.push(result);
            }

            // -vオプションが指定されていたら、この時点でのトークンとスタックを出力
            if self.0 {
                println!("{:?} {:?}", tokens, stack);
            }
        }

        if stack.len() == 1 {
            stack[0]
        } else {
            panic!("invalid syntax")
        }
    }
}

fn main() {
    let opts = Opts::parse();

    if let Some(path) = opts.formula_file {
        let f = File::open(path).unwrap();
        let reader = BufReader::new(f);
        run(reader, opts.verbose);
    } else {
        let stdin = stdin();
        let reader = stdin.lock();
        run(reader, opts.verbose);
    }
}

/// Runs the RPN calculator with the provided reader
/// # Arguments:
/// * `reader`: A reader that provides lines of input
/// * `verbose`: A boolean indicating if verbose output is enabled
fn run<R: BufRead>(reader: R, verbose: bool) {
    let calc = RpnCalculator::new(verbose);

    for line in reader.lines() {
        let line = line.unwrap();
        let answer = calc.evaluate(&line);
        println!("{}", answer);
    }
}