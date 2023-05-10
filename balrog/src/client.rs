use tokio::runtime::{Builder, Runtime};

use vega_protobufs::datanode::api::v2::{
    trading_data_service_client::TradingDataServiceClient, AccountFilter, GetAssetRequest,
    GetAssetResponse, GetStakeRequest, GetStakeResponse, ListAccountsRequest, ListAccountsResponse,
    ListGovernanceDataRequest, ListGovernanceDataResponse, ListNodesRequest, ListNodesResponse,
};
use vega_protobufs::vega::api::v1::submit_transaction_request::Type;
use vega_protobufs::vega::api::v1::{
    core_service_client::CoreServiceClient, LastBlockHeightRequest, LastBlockHeightResponse,
    SubmitTransactionRequest, SubmitTransactionResponse,
};
use vega_protobufs::vega::commands::v1::Transaction;
use vega_protobufs::vega::AccountType;

type StdError = Box<dyn std::error::Error + Send + Sync + 'static>;
// type Result<T, E = StdError> = ::std::result::Result<T, E>;

pub struct CoreBlockingClient {
    client: CoreServiceClient<tonic::transport::Channel>,
    rt: Runtime,
}

impl CoreBlockingClient {
    pub fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
    where
        D: std::convert::TryInto<tonic::transport::Endpoint>,
        D::Error: Into<StdError>,
    {
        let rt = Builder::new_multi_thread().enable_all().build().unwrap();
        let client = rt.block_on(CoreServiceClient::connect(dst))?;

        Ok(Self { client, rt })
    }

    pub fn last_block_height(&mut self) -> Result<LastBlockHeightResponse, tonic::Status> {
        Ok(self
            .rt
            .block_on(self.client.last_block_height(LastBlockHeightRequest {}))?
            .into_inner())
    }

    pub fn submit_transaction(
        &mut self,
        transaction: Transaction,
    ) -> Result<SubmitTransactionResponse, tonic::Status> {
        Ok(self
            .rt
            .block_on(self.client.submit_transaction(SubmitTransactionRequest {
                tx: Some(transaction),
                r#type: Type::Sync.into(),
            }))?
            .into_inner())
    }
}

pub struct DatanodeV2BlockingClient {
    client: TradingDataServiceClient<tonic::transport::Channel>,
    rt: Runtime,
}

impl DatanodeV2BlockingClient {
    pub fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
    where
        D: std::convert::TryInto<tonic::transport::Endpoint>,
        D::Error: Into<StdError>,
    {
        let rt = Builder::new_multi_thread().enable_all().build().unwrap();
        let client = rt.block_on(TradingDataServiceClient::connect(dst))?;

        Ok(Self { client, rt })
    }

    pub fn get_nodes(&mut self) -> Result<ListNodesResponse, tonic::Status> {
        Ok(self
            .rt
            .block_on(self.client.list_nodes(ListNodesRequest {
                epoch_seq: None,
                pagination: None,
            }))?
            .into_inner())
    }

    pub fn get_proposals(&mut self) -> Result<ListGovernanceDataResponse, tonic::Status> {
        Ok(self
            .rt
            .block_on(self.client.list_governance_data(ListGovernanceDataRequest {
                proposal_reference: None,
                proposal_state: None,
                proposal_type: None,
                proposer_party_id: None,
                pagination: None,
            }))?
            .into_inner())
    }

    pub fn get_account(&mut self, party_id: &str) -> Result<ListAccountsResponse, tonic::Status> {
        Ok(self
            .rt
            .block_on(self.client.list_accounts(ListAccountsRequest {
                filter: Some(AccountFilter {
                    party_ids: vec![party_id.into()],
                    market_ids: vec![],
                    asset_id: String::new(),
                    account_types: vec![AccountType::General.into()],
                }),
                pagination: None,
            }))?
            .into_inner())
    }

    pub fn get_asset(&mut self, asset_id: &str) -> Result<GetAssetResponse, tonic::Status> {
        Ok(self
            .rt
            .block_on(self.client.get_asset(GetAssetRequest {
                asset_id: asset_id.to_string(),
            }))?
            .into_inner())
    }

    pub fn get_party_stake(&mut self, party_id: &str) -> Result<GetStakeResponse, tonic::Status> {
        Ok(self
            .rt
            .block_on(self.client.get_stake(GetStakeRequest {
                party_id: party_id.to_string(),
                pagination: None,
            }))?
            .into_inner())
    }
}
