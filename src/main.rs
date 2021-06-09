fn main() {
    let mut args = std::env::args().skip(1);
    let expr = match args.next() {
        Some(e) => e,
        None => {
            eprintln!("Must supply expression");
            return;
        }
    };

    let source = format!("#version 450\nvoid main(){{}}\n{}", expr);
    
    let mut compiler = shaderc::Compiler::new().unwrap();
    let result = compiler.compile_into_spirv(
        &source, 
        shaderc::ShaderKind::Compute,
        "shader.glsl", 
        "main",
        None
    );

    match result {
        Ok(_) => println!("Compiled successfully."),
        Err(e) => eprintln!("Error: {}", e),
    }
}
