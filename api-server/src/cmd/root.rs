use std::env;

pub struct RootCommand {
    args: Vec<String>,
}

impl RootCommand {
    pub fn new() -> Self {
        let args: Vec<String> = env::args().collect();
        RootCommand { args }
    }

    pub fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        if self.args.len() < 2 {
            self.print_help();
            return Ok(());
        }

        match self.args[1].as_str() {
            "cli" => {
                crate::cmd::cli::cli_command()?;
            }
            "ecocart-apis" => {
                let rt = tokio::runtime::Runtime::new()?;
                rt.block_on(crate::cmd::apis::apis_command())?;
            }
            "grpc" => {
                crate::cmd::grpc::grpc_command()?;
            }
            "migrate" => {
                let rt = tokio::runtime::Runtime::new()?;
                rt.block_on(crate::cmd::migrate::migrate_command())?;
            }
            "seed" => {
                let rt = tokio::runtime::Runtime::new()?;
                rt.block_on(crate::cmd::seed::seed_command())?;
            }
            "--help" | "-h" | "help" => {
                self.print_help();
            }
            _ => {
                eprintln!("Unknown command: {}", self.args[1]);
                self.print_help();
            }
        }

        Ok(())
    }

    fn print_help(&self) {
        println!("ecocart - A query processing tool");
        println!();
        println!("Usage: ecocart <command> [options]");
        println!();
        println!("Commands:");
        println!("  cli     - Run CLI interface");
        println!("  ecocart-apis    - Run API server");
        println!("  grpc    - Run gRPC server");
        println!("  migrate - Run SQLx database migrations (Postgres)");
        println!("  seed    - Seed default admin user (idempotent)");
        println!();
        println!("Options:");
        println!("  --help, -h, help  - Show this help message");
    }
}
