use tokio::runtime::{Builder, Runtime};

use vega_rust_sdk::datanode::api::v2::{
    trading_data_service_client::TradingDataServiceClient, AssetByIdRequest, AssetByIdResponse,
    GetNodesRequest, GetNodesResponse, GetProposalsRequest, GetProposalsResponse,
    PartyAccountsRequest, PartyAccountsResponse, PartyStakeRequest, PartyStakeResponse,
};
use vega_rust_sdk::vega::api::v1::submit_transaction_request::Type;
use vega_rust_sdk::vega::api::v1::{
    core_service_client::CoreServiceClient, LastBlockHeightRequest, LastBlockHeightResponse,
    SubmitTransactionRequest, SubmitTransactionResponse,
};
use vega_rust_sdk::vega::commands::v1::Transaction;
use vega_rust_sdk::vega::AccountType;

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

    pub fn get_nodes(&mut self) -> Result<GetNodesResponse, tonic::Status> {
        Ok(self
            .rt
            .block_on(self.client.get_nodes(GetNodesRequest {}))?
            .into_inner())
    }

    pub fn get_proposals(&mut self) -> Result<GetProposalsResponse, tonic::Status> {
        Ok(self
            .rt
            .block_on(self.client.get_proposals(GetProposalsRequest {
                select_in_state: None,
            }))?
            .into_inner())
    }

    pub fn get_account(&mut self, party_id: &str) -> Result<PartyAccountsResponse, tonic::Status> {
        Ok(self
            .rt
            .block_on(self.client.party_accounts(PartyAccountsRequest {
                party_id: party_id.into(),
                market_id: String::new(),
                asset: String::new(),
                r#type: AccountType::General.into(),
            }))?
            .into_inner())
    }

    pub fn get_asset(&mut self, asset_id: &str) -> Result<AssetByIdResponse, tonic::Status> {
        Ok(self
            .rt
            .block_on(self.client.asset_by_id(AssetByIdRequest {
                id: asset_id.to_string(),
            }))?
            .into_inner())
    }

    pub fn get_party_stake(&mut self, party_id: &str) -> Result<PartyStakeResponse, tonic::Status> {
        Ok(self
            .rt
            .block_on(self.client.party_stake(PartyStakeRequest {
                party: party_id.to_string(),
            }))?
            .into_inner())
    }
}
