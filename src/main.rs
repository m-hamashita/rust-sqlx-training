use sqlx::mysql::MySqlPoolOptions;

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    let pool = MySqlPoolOptions::new()
        .connect("mysql://user:password@localhost/test_db")
        .await?;

    // test query
    // let row: (i64,) = sqlx::query_as("SELECT 1")
    //     .fetch_one(&pool)
    //     .await?;

    // set transaction isolation level 
    let _  = sqlx::query("set transaction isolation level read committed")
        .execute(&pool)
        .await?;

    let row: (String, String) = sqlx::query_as("show variables like 'transaction_isolation'")
        .fetch_one(&pool)
        .await?;
    println!("{} = {}", row.0, row.1);
        

    Ok(())
}
