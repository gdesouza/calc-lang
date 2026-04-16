use rand::Rng;

struct Generator<R: Rng> {
    rng: R,
    declared_vars: Vec<String>,
    max_depth: usize,
}

impl<R: Rng> Generator<R> {
    fn new(rng: R, max_depth: usize) -> Self {
        Self {
            rng,
            declared_vars: Vec::new(),
            max_depth,
        }
    }

    fn gen_letter(&mut self) -> char {
        let letters = b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
        letters[self.rng.random_range(0..letters.len())] as char
    }

    fn gen_identifier(&mut self) -> String {
        let len = self.rng.random_range(1..=4);
        (0..len).map(|_| self.gen_letter()).collect()
    }

    fn gen_literal(&mut self) -> String {
        let len = self.rng.random_range(1..=3);
        let digits = b"0123456789";
        (0..len)
            .map(|_| digits[self.rng.random_range(0..digits.len())] as char)
            .collect()
    }

    fn gen_factor(&mut self, depth: usize) -> String {
        if depth >= self.max_depth {
            return if !self.declared_vars.is_empty() && self.rng.random_bool(0.5) {
                let idx = self.rng.random_range(0..self.declared_vars.len());
                self.declared_vars[idx].clone()
            } else {
                self.gen_literal()
            };
        }

        match self.rng.random_range(0..3) {
            0 if !self.declared_vars.is_empty() => {
                let idx = self.rng.random_range(0..self.declared_vars.len());
                self.declared_vars[idx].clone()
            }
            0 => self.gen_literal(),
            1 => self.gen_literal(),
            _ => format!("({})", self.gen_expr(depth + 1)),
        }
    }

    fn gen_term(&mut self, depth: usize) -> String {
        let mut result = self.gen_factor(depth);
        let extra = self.rng.random_range(0..3);
        for _ in 0..extra {
            let op = if self.rng.random_bool(0.5) { "*" } else { "/" };
            result = format!("{}{}{}", result, op, self.gen_factor(depth));
        }
        result
    }

    fn gen_expr(&mut self, depth: usize) -> String {
        let mut result = self.gen_term(depth);
        let extra = self.rng.random_range(0..3);
        for _ in 0..extra {
            let op = if self.rng.random_bool(0.5) { "+" } else { "-" };
            result = format!("{}{}{}", result, op, self.gen_term(depth));
        }
        result
    }

    fn gen_statement(&mut self) -> String {
        match self.rng.random_range(0..4) {
            0 => {
                let id = self.gen_identifier();
                self.declared_vars.push(id.clone());
                format!("@{}", id)
            }
            1 if !self.declared_vars.is_empty() => {
                let idx = self.rng.random_range(0..self.declared_vars.len());
                format!(">{}", self.declared_vars[idx].clone())
            }
            1 => {
                let id = self.gen_identifier();
                self.declared_vars.push(id.clone());
                format!("@{}", id)
            }
            2 => {
                format!("<{}", self.gen_expr(0))
            }
            _ => {
                let id = if !self.declared_vars.is_empty() && self.rng.random_bool(0.7) {
                    let idx = self.rng.random_range(0..self.declared_vars.len());
                    self.declared_vars[idx].clone()
                } else {
                    let id = self.gen_identifier();
                    self.declared_vars.push(id.clone());
                    id
                };
                format!("{}:={}", id, self.gen_expr(0))
            }
        }
    }

    fn gen_program(&mut self, num_statements: usize) -> String {
        let mut lines = Vec::new();
        for _ in 0..num_statements {
            lines.push(self.gen_statement());
        }
        lines.join("\n")
    }
}

fn main() {
    let mut args = std::env::args();
    let program_name = args.next().unwrap();
    let num_statements: usize = args
        .next()
        .and_then(|s| s.parse().ok())
        .unwrap_or_else(|| {
            eprintln!("Usage: {} <num_statements> [max_expr_depth]", program_name);
            std::process::exit(1);
        });
    let max_depth: usize = args.next().and_then(|s| s.parse().ok()).unwrap_or(3);

    let rng = rand::rng();
    let mut generator = Generator::new(rng, max_depth);
    println!("{}", generator.gen_program(num_statements));
}
