extern crate nalgebra as na;
mod inverse_kinematics;
mod serialization;
mod encoder_interface;
mod front_display;
mod state_server;
use state_server::*;
use front_display::{lcd_setup};
use encoder_interface::{setup_encoder};
use na::{Vector3};
use sqlx::sqlite::SqlitePool;
use std::env;
use structopt::StructOpt;
#[derive(StructOpt)]
struct Args {
    #[structopt(subcommand)]
    cmd: Option<Command>,
}

#[derive(StructOpt)]
enum Command {
    Add { description: String },
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {

    // The `<StateA>` is implied here. We don't need to add type annotations!
    let in_state_a = StateMachine::new("Booting up...".into());

    // This is okay here. But later once we've changed state it won't work anymore.
    in_state_a.some_unrelated_value;
    println!("Starting Value: {}", in_state_a.state.start_value);


    // Transition to the new state. This consumes the old state.
    // Here we need type annotations (since not all StateMachines are linear in their state).
    let in_state_b = StateMachine::<PreOperational>::from(in_state_a);

    // This doesn't work! The value is moved when we transition!
    // in_state_a.some_unrelated_value;
    // Instead, we can use the existing value.
    in_state_b.some_unrelated_value;

    println!("Interm Value: {:?}", in_state_b.state.interm_value);

    // And our final state.
    let in_state_c = StateMachine::<Operational>::from(in_state_b);

    // This doesn't work either! The state doesn't even contain this value.
    // in_state_c.state.start_value;

    println!("Final state: {}", in_state_c.state.final_value);

        let t2=Vector3::new(5.0, 5.0, 5.0);
        let v = inverse_kinematics::inverse_kinematics::simple_ik(t2);
        lcd_setup();
        if let Err(report) = setup_encoder().await{

        }
        let args = Args::from_args_safe()?;
    let pool = SqlitePool::connect(&env::var("DATABASE_URL")?).await?;

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

    Ok(())
}

async fn add_todo(pool: &SqlitePool, alias: String,vector:String,rotations:String) -> anyhow::Result<i64> {
    let mut conn = pool.acquire().await?;

    // Insert the task, then obtain the ID of this row
    let id = sqlx::query!(
        r#"
INSERT INTO nodes ( alias,vectors,rotations,following_node)
VALUES ( ?1,?2,?3,?4 )
        "#,
        alias,
        vector,
        rotations,
        1,
    )
    .execute(&mut conn)
    .await?
    .last_insert_rowid();

    Ok(id)
}

async fn list_todos(pool: &SqlitePool) -> anyhow::Result<()> {
    let recs = sqlx::query!(
        r#"
SELECT id, alias, vectors, rotations
FROM nodes
ORDER BY id
        "#
    )
    .fetch_all(pool)
    .await?;

    for rec in recs {
        println!(
            "- [{}] {} {} {}",
            rec.id,
            &rec.alias,
            &rec.vectors,
            &rec.rotations,
        );
    }
    Ok(())
}