use gitlab_note_mr::app;
use std::process::ExitCode;

#[actix_web::main]
async fn server(config_file: &str) -> std::io::Result<()> {
    app(config_file).await
}

fn main() -> ExitCode {
    let config_file = match std::env::args().nth(1) {
        Some(x) => x,
        None => {
            eprintln!("usage: {:?} <config.yml>", std::env::current_exe());
            return ExitCode::FAILURE;
        }
    };
    match server(&config_file[..]) {
        Ok(_) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("fatal: {:?}", e);
            ExitCode::FAILURE
        }
    }
}
