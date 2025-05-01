use std::{sync::Arc, time::Duration};

use zksync_config::configs::TeeProofDataHandlerConfig;
use zksync_dal::{Connection, ConnectionPool, Core, CoreDal};
use zksync_object_store::ObjectStore;
use zksync_types::{commitment::L1BatchCommitmentMode, L2ChainId};
use tokio::{select, sync::watch};

pub(crate) async fn updater(
    blob_store: Arc<dyn ObjectStore>,
    mut connection_pool: ConnectionPool<Core>,
    config: TeeProofDataHandlerConfig,
    commitment_mode: L1BatchCommitmentMode,
    l2_chain_id: L2ChainId,
    mut stop_receiver: watch::Receiver<bool>,
) -> anyhow::Result<()> {
    let mut interval = tokio::time::interval(Duration::from_secs(config.dcap_collateral_refresh_seconds));
    let mut connection = connection_pool.connection_tagged("tee_dcap_collateral_updater").await?;
    loop {
        select! {
            _ = interval.tick() => {}
            signal = stop_receiver.changed() => {
                if signal.is_err() {
                    tracing::warn!("Stop signal sender for tee dcap collateral updater was dropped without sending a signal");
                }
                tracing::info!("Stop signal received, tee dcap collateral updater is shutting down");
                return Ok(());
            }
        }
        let mut dal = connection.tee_dcap_collateral_dal();
        // do work here
    } 
}
