// use ethers_solc::Project;
use foundry_common::compile::ProjectCompiler;

fn main() {
    let foundry_config = foundry_config::Config::load();
    match ProjectCompiler::new(true, false)
        .compile_with(&foundry_config.project().unwrap(), |prj| Ok(prj.compile()?))
    {
        Err(err) => {
            println!("failed to compile foundry project: {}", err);
            std::process::exit(1);
        }
        _ => {}
    }

    std::process::exit(0);
}
