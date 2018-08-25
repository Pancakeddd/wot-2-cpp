mod transpiler;

fn main() {
    let mut tp = transpiler::Transpiler::new();
    tp.pushn(234.0);
    tp.push_add();
    tp.pushn(23546.0);
    tp.end();
}
