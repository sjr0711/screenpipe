#[cfg(test)]
mod tests {
    use sqlx::sqlite::SqlitePool;

    #[tokio::test]
    async fn test_fts_match_verify() -> Result<(), sqlx::Error> {
        let pool = SqlitePool::connect("sqlite::memory:").await?;
        sqlx::query("CREATE VIRTUAL TABLE fts USING fts5(text_col, app_name);").execute(&pool).await?;
        sqlx::query("INSERT INTO fts (text_col, app_name) VALUES ('hello verify', 'my_app');").execute(&pool).await?;

        let q1 = "\"verify:something\"";
        let r1: Vec<(String,)> = sqlx::query_as("SELECT text_col FROM fts WHERE fts MATCH ?")
            .bind(q1).fetch_all(&pool).await?;
        println!("r1: {:?}", r1);

        let q2 = "\"-verify\"";
        let r2: Vec<(String,)> = sqlx::query_as("SELECT text_col FROM fts WHERE fts MATCH ?")
            .bind(q2).fetch_all(&pool).await?;
        println!("r2: {:?}", r2);

        let q3 = "verify:something";
        let r3: Result<Vec<(String,)>, _> = sqlx::query_as("SELECT text_col FROM fts WHERE fts MATCH ?")
            .bind(q3).fetch_all(&pool).await;
        println!("r3: {:?}", r3);

        Ok(())
    }
}
