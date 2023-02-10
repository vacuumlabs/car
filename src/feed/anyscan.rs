use async_trait::async_trait;
use sea_orm::DatabaseConnection;

#[async_trait]
impl super::Feed for shared::AnyScan {
    async fn wait(&self, start: tokio::time::Instant) {
        tracing::info!("AnyScan waiting");
        let delay = tokio::time::Instant::now().duration_since(start);
        if let Some(duration) = tokio::time::Duration::from_millis(self.delay).checked_sub(delay) {
            tracing::info!("AnyScan sleeping for: {}", duration.as_millis());
            tokio::time::sleep(duration).await;
        } else {
            tracing::info!("Evil loop!!! {}", delay.as_millis());
        }
    }

    async fn process_block(&self, db: &DatabaseConnection, chain: &crate::entity::chain::Model) {
        tracing::info!("AsnyScan block");

        self.add_addresses(db, chain.id.clone(), vec![vec![8u8; 6], vec![10u8; 6]])
            .await;
        self.add_transactions(
            db,
            chain.id.clone(),
            vec![(vec![10u8, 4], Some(10), vec![1, 2], vec![1, 2])],
        )
        .await;
    }
}
