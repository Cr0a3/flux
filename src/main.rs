#[forbid(missing_docs)]

use ygen::Support::Cli;

mod driver;
mod lexer;
mod parser;
mod codegen;
mod semnatic;
mod error;
mod ast;

fn driver_main() -> Result<(), Box<dyn std::error::Error>> {
    
    let mut cli = Cli::new("flux", "The flux programming language", "dev", "Cr0a3, SunkenPotato");

    // Input/Ouput file

    cli.add_unpositional("infile", true);
    cli.add_arg("o", "out", "The output file", false);

    // Backend related flags

    cli.add_opt("s", "asm", "Emit assembly code");
    cli.add_arg("target", "target", "The target triple", false);
    cli.add_arg("emit-ir", "emit-ir", "Emit ygen ir", false);

    // Optimization related flags

    cli.add_opt("O0", "Opt0", "Disable all optimizations");
    cli.add_opt("O3", "Opt3", "Enable all optimizations");
    cli.add_arg("passes", "passes", "Enable specific passes", false);

    cli.scan();

    // Create a new compiler and set it's values

    let file = cli.arg_val("infile").expect("no input file providet");

    let mut compiler = driver::FluxCompiler::new(&file)?;

    // There maybe is a output file specfied
    if let Some(out_file) = cli.arg_val("out") {
        compiler.set_out(&out_file);
    }

    // We now process the optimization level arguments
    if cli.opt("O0") { compiler.set_o0(); }
    if cli.opt("O0") { compiler.set_o3(); }
    if let Some(passes) = cli.arg_val("passes") {
        compiler.set_passes(&passes);
    }

    // Here the compilation specfic options/arguments
    if cli.opt("s") { compiler.emit_asm(); }
    if cli.opt("emit-ir") { compiler.emit_ir(); }
    if let Some(target) = cli.arg_val("target") {
        compiler.set_target_from_string(&target)?;
    }

    // Finally we run the compiler
    compiler.compile();

    Ok(())
}

fn main() {
    // run the driver main
    // check if it exited with an error, else return 
    let Err(err) = driver_main() else { return; };

    eprintln!("{}", err);
}
