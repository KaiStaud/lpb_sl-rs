extern crate nalgebra as na;
mod inverse_kinematics;
mod serialization;
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
        let t2=Vector3::new(5.0, 5.0, 5.0);
        let v = inverse_kinematics::inverse_kinematics::simple_ik(t2);
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