extern crate nalgebra as na;
mod cli;
mod inverse_kinematics;
mod serialization;
mod tracker;

use clap::Parser;
use cli::DebugArgs;
use na::Vector3;
use sqlx::sqlite::SqlitePool;
use std::env;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let t2 = Vector3::new(5.0, 5.0, 5.0);
    let v = inverse_kinematics::inverse_kinematics::simple_ik(t2);
    let args = DebugArgs::parse(); //Args::from_args_safe()?;
    let pool = SqlitePool::connect(&env::var("DATABASE_URL")?).await?;

    /*
        match args.cmd {
            Some(Command::Add { description }) => {
                println!("Adding new todo with description '{}'", &description);
                let todo_id = add_todo(&pool, "Test".to_string(),"1".to_string(),"2".to_string()).await?;
                println!("Added new todo with id {}", todo_id);
            }

            None => {
                println!("Printing list of all todos");
                list_todos(&pool).await?;
            }
        }
    */
    Ok(())
}
