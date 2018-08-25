pub struct Transpiler {
    pub output: String,
}

impl Transpiler {
    pub fn new() -> Transpiler {
        let temp = String::from(r#"
            #include <iostream>
            int main(){auto result = "#);
        Transpiler {
            output: temp.clone(),
        }
    }

    fn push(&mut self, string: &str) {
        self.output.push_str(string);
    }

    pub fn pushn(&mut self, n: f64) {
        self.push(&format!("{}", n));
    }

    pub fn push_add(&mut self) {
        self.push(" + ");
    }
    pub fn push_sub(&mut self) {
        self.push(" - ");
    }
    pub fn push_mul(&mut self) {
        self.push(" * ");
    }
    pub fn push_div(&mut self) {
        self.push(" / ");
    }
    pub fn push_neg(&mut self) {
        self.push(" -");
    }

    pub fn push_bo(&mut self) {
        self.push("(");
    }
    pub fn push_bc(&mut self) {
        self.push(")");
    }

    pub fn end(&mut self) {
        self.output.push_str(";std::cout<<result;}");
        println!("{}", self.output);
    }
}