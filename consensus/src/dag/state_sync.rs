use super::{dag_fetcher::DagFetcher, dag_store::Dag, storage::DAGStorage, TDAGNetworkSender, types::{RemoteFetchRequest, NodeMetadata}, NodeId};
use crate::state_replication::StateComputer;
use aptos_infallible::RwLock;
use aptos_time_service::TimeService;
use aptos_types::{epoch_state::EpochState, ledger_info::LedgerInfoWithSignatures};
use std::sync::Arc;

pub const DAG_WINDOW: u64 = 10;

struct StateSyncAdapter {
    epoch_state: Arc<EpochState>,
    network: Arc<dyn TDAGNetworkSender>,
    time_service: TimeService,
    state_computer: Arc<dyn StateComputer>,
    storage: Arc<dyn DAGStorage>,
}

impl StateSyncAdapter {
    async fn sync_to_ledger_info(&self, target: LedgerInfoWithSignatures) -> anyhow::Result<()> {
        // Check whether to actually sync

        // TODO: there is a case where DAG fetches missing nodes in window, and a crash happens and when we restart,
        // we end up with a gap between the DAG and we need to be smart enough to clean up the DAG before the gap.

        // Create a new DAG store and Fetch blocks
        let target_round = target.ledger_info().round();
        let initial_round = target_round.saturating_sub(DAG_WINDOW);
        let dag = Arc::new(RwLock::new(Dag::new(
            self.epoch_state.clone(),
            self.storage.clone(),
            initial_round,
        )));
        let fetcher = DagFetcher::new(
            self.epoch_state.clone(),
            self.network.clone(),
            dag,
            self.time_service.clone(),
        );

        let target = 

        let request = RemoteFetchRequest::new(
            self.epoch_state.epoch,
            targets,
            dag.read().bitmask(target_round),
        );
        fetcher.fetch(request, responders, target);

        // State sync
        self.state_computer.sync_to(target).await?;

        Ok(())
    }
}
