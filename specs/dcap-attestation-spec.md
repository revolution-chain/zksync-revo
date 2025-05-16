# DCAP Attestation Wallet Specification

## Overview

This specification outlines the implementation of an optional DCAP (Data Center Attestation Primitives) attestation
wallet in the ZKSync Era `eth_sender` crate. This wallet will be used to create additional transactions during L1
commitments, enabling attestation verification for enhanced security in trusted environments.

## Functional Requirements

1. Add an optional DCAP attestation wallet to the `zksync_eth_sender` crate located in `core/node/eth_sender`
2. Create and send additional attestation transactions with this wallet during L1 commitment operations
3. Implement a configurable behavior to enable/disable the DCAP attestation feature
4. Ensure proper error handling and logging for the attestation process
5. Update metrics to track attestation transaction status and performance

## Technical Design

### 1. Configuration

Add DCAP attestation options to the `SenderConfig` in the `zksync_config` crate:

```rust
pub struct SenderConfig {
    // Existing fields...

    /// Optional DCAP attestation private key. If provided, will be used to send 
    /// attestation transactions with each L1 commitment
    pub dcap_attestation_private_key: Option<String>,

    /// Optional parameters for DCAP attestation
    pub dcap_attestation_params: Option<DcapAttestationParams>,
}

pub struct DcapAttestationParams {
    /// Gas limit for DCAP attestation transactions
    pub gas_limit: u64,

    /// Maximum number of retry attempts for failed attestation transactions
    pub max_retries: u32,

    /// Contract address for the DCAP attestation verification
    pub contract_address: Address,
}
```

### 2. Wallet Integration

Modify the `EthTxManager` to support an optional DCAP attestation wallet:

```rust
pub struct EthTxManager {
    // Existing fields...

    /// Optional wallet for DCAP attestation transactions
    dcap_attestation_wallet: Option<Wallet>,

    /// Optional DCAP attestation parameters
    dcap_attestation_params: Option<DcapAttestationParams>,
}
```

### 3. L1 Interface Extension

Extend the `AbstractL1Interface` trait to support DCAP attestation operations:

```rust
pub trait AbstractL1Interface: Debug + Send {
    // Existing methods...

    /// Sends a DCAP attestation transaction to attest to the L1 commitment
    async fn send_dcap_attestation(
        &self,
        commitment_data: &[u8],
        base_fee_per_gas: u64,
        priority_fee_per_gas: u64,
        gas_limit: U256,
    ) -> Result<SignedTxInfo, EthSenderError>;

    /// Gets the status of a sent DCAP attestation transaction
    async fn get_dcap_attestation_tx_status(
        &self,
        tx_hash: H256,
    ) -> Result<Option<ExecutedTxStatus>, EthSenderError>;
}
```

### 4. Implementation for Attestation Transaction Creation

Add functionality to create and send attestation transactions after each L1 commit:

```rust
impl Aggregator {
    // Existing methods...

    pub async fn send_dcap_attestation_for_commitment(
        &self,
        eth_tx_manager: &mut EthTxManager,
        storage: &mut Connection<'_, Core>,
        commitment_data: &[u8],
        l1_batch_number: L1BatchNumber,
    ) -> Result<Option<H256>, EthSenderError> {
        if eth_tx_manager.dcap_attestation_wallet.is_none() {
            return Ok(None);
        }

        // Implementation of sending the attestation transaction
        // ...

        Ok(Some(tx_hash))
    }
}
```

### 5. Integration with Commit Operation

Modify the existing commit operation flow to include the attestation transaction:

```rust
impl EthTxAggregator {
    // Existing methods...

    async fn process_aggregated_op(
        &self,
        aggregated_op: AggregatedOperation,
        status_updater: &dyn StatusUpdater,
    ) -> Result<ProcessedTxData, EthSenderError> {
        // Existing implementation...

        // If this is a commit operation and we have a DCAP attestation wallet
        if let AggregatedOperation::Commit(_, batches, _, _) = &aggregated_op {
            if self.eth_tx_manager.dcap_attestation_wallet.is_some() {
                for batch in batches {
                    let commitment_data = self.get_commitment_data_for_batch(batch)?;
                    let attestation_tx_hash = self.aggregator
                        .send_dcap_attestation_for_commitment(
                            &mut self.eth_tx_manager,
                            &mut storage,
                            &commitment_data,
                            batch.header.number,
                        )
                        .await?;

                    if let Some(tx_hash) = attestation_tx_hash {
                        tracing::info!(
                            "Sent DCAP attestation transaction for L1 batch {}. Tx hash: {:?}",
                            batch.header.number,
                            tx_hash
                        );
                    }
                }
            }
        }

        // Existing implementation...
    }
}
```

### 6. Metrics Tracking

Add metrics to track DCAP attestation operations:

```rust
impl Metrics {
    // Existing methods...

    pub fn new() -> Self {
        let dcap_attestation_transactions = register_counter_vec(
            "dcap_attestation_transactions",
            "Number of DCAP attestation transactions by status",
            &["status"],
        );

        let dcap_attestation_gas_used = register_histogram(
            "dcap_attestation_gas_used",
            "Amount of gas used by DCAP attestation transactions",
            exponential_buckets(1000.0, 1.5, 25).unwrap(),
        );

        // Include these metrics in the Metrics struct
        // ...
    }
}
```

## Error Handling

DCAP attestation transactions should be considered non-critical. If an attestation transaction fails, the system should
log the error but continue normal operation. This ensures that problems with the attestation process do not block the
main ZKSync functionality.

## Testing

1. Unit tests for the DCAP attestation wallet initialization and transaction creation
2. Integration tests to verify the attestation process with mock L1 interface
3. Test scenarios for error handling and recovery

## Implementation Timeline and Complexity

- **Estimated implementation time**: 2-3 weeks
- **Complexity**: Medium
- **Dependencies**: Requires understanding of Ethereum wallet management and DCAP attestation mechanisms
