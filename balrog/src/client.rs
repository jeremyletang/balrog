use serde::de::IntoDeserializer;
use tokio::runtime::{Builder, Runtime};

use vega_rust_sdk::datanode::api::v2::{
    trading_data_service_client::TradingDataServiceClient, AssetByIdRequest, AssetByIdResponse,
    PartyAccountsRequest, PartyAccountsResponse,
};
use vega_rust_sdk::vega::api::v1::{
    core_service_client::CoreServiceClient, LastBlockHeightRequest, LastBlockHeightResponse,
};

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

    pub fn last_block_height(
        &mut self,
    ) -> Result<tonic::Response<LastBlockHeightResponse>, tonic::Status> {
        self.rt
            .block_on(self.client.last_block_height(LastBlockHeightRequest {}))
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

    pub fn get_account(
        &mut self,
        party_id: &str,
    ) -> Result<tonic::Response<PartyAccountsResponse>, tonic::Status> {
        self.rt
            .block_on(self.client.party_accounts(PartyAccountsRequest {
                party_id: party_id.into(),
                market_id: String::new(),
                asset: String::new(),
                r#type: 0,
            }))
    }

    pub fn get_asset(
        &mut self,
        asset_id: &str,
    ) -> Result<tonic::Response<AssetByIdResponse>, tonic::Status> {
        self.rt.block_on(self.client.asset_by_id(AssetByIdRequest {
            id: asset_id.to_string(),
        }))
    }
}
