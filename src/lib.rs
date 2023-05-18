#[cfg(test)]
mod tests {
    use cassandra_cpp::*;

    #[tokio::test]
    async fn test_insert_into_database() {
        let mut cluster = Cluster::default();
        cluster.set_contact_points("127.0.0.1").unwrap();

        let session = cluster.connect().await.unwrap();

        let query = format!("INSERT INTO test (key, value) VALUES (?, ?)");
        let prepared: PreparedStatement = session.prepare(&query).await.unwrap();
        let mut statement = prepared.bind();
        statement.bind(0, "key").unwrap();
        statement.bind(0, "value").unwrap();

        let result = statement.execute().await.unwrap();
        assert_eq!(result.row_count(), 1);

        // Now, verify the data

        let query = format!("SELECT value FROM test WHERE key = ?");
        let prepared: PreparedStatement = session.prepare(&query).await.unwrap();
        let mut statement = prepared.bind();
        statement.bind(0, "key").unwrap();

        let select_result = statement.execute().await.unwrap();
        assert_eq!(select_result.row_count(), 1);

        let row = select_result.first_row().unwrap();
        let value: String = row.get_column(0).unwrap().get_str().unwrap().to_string();
        assert_eq!(value, "value");
    }
}
