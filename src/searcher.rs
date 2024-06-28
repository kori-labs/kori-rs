use std::str::FromStr;
use tonic::transport::Channel;

use crate::proto;

use crate::proto::auth::Token;
use crate::proto::searcher::searcher_service_client::SearcherServiceClient;
use crate::proto::searcher::{AddressSubscriptionV0, EventSubscriptionV0, GetTipAddressesRequest, GetTipAddressesResponse, MempoolSubscription, PackageSubscriptionV0, SendBundleResponse, SharedObjectSubscriptionV0};

pub struct KoriSearcher {
    searcher_client: SearcherServiceClient<Channel>,
    access_token: Option<Token>,
}

impl KoriSearcher {
    pub async fn new(searcher_url: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let searcher_client = SearcherServiceClient::connect(searcher_url.to_string()).await?;

        Ok(Self {
            searcher_client,
            access_token: None,
        })
    }

    pub fn set_access_token(&mut self, token: Token) {
        self.access_token = Some(token);
    }

    pub async fn subscribe_mempool<F>(
        &mut self,
        packages: Vec<String>,
        objects: Vec<String>,
        addresses: Vec<String>,
        events: Vec<String>,
        on_data: F,
    ) -> Result<(), Box<dyn std::error::Error>>
        where
            F: Fn(proto::dto::MempoolPacket) + Send + 'static,
    {
        let mut request = tonic::Request::new(MempoolSubscription {
            packages: if !packages.is_empty() { Some(PackageSubscriptionV0 { package: packages }) } else { None },
            objects: if !objects.is_empty() { Some(SharedObjectSubscriptionV0 { object: objects }) } else { None },
            addresses: if !addresses.is_empty() { Some(AddressSubscriptionV0 { address: addresses }) } else { None },
            events: if !events.is_empty() { Some(EventSubscriptionV0 { event: events }) } else { None },
        });

        if let Some(access_token) = &self.access_token {
            request.metadata_mut().insert(
                "authorization",
                tonic::metadata::MetadataValue::from_str(
                    &format!("Bearer {}", access_token.value)
                )?,
            );
        }

        let mut stream = self.searcher_client.subscribe_mempool(request).await?.into_inner();

        tokio::spawn(async move {
            while let Some(response) = stream.message().await.unwrap_or(None) {
                on_data(response);
            }
        });

        Ok(())
    }

    pub async fn send_bundle(&mut self, bundle: proto::dto::Bundle) -> Result<SendBundleResponse, Box<dyn std::error::Error>> {
        let mut request = tonic::Request::new(bundle);

        if let Some(access_token) = &self.access_token {
            request.metadata_mut().insert(
                "authorization",
                tonic::metadata::MetadataValue::from_str(
                    &format!("Bearer {}", access_token.value)
                )?,
            );
        }

        let response = self.searcher_client.send_bundle(request).await?;

        Ok(response.into_inner())
    }

    pub async fn get_tip_addresses(&mut self) -> Result<GetTipAddressesResponse, Box<dyn std::error::Error>> {
        let mut request = tonic::Request::new(GetTipAddressesRequest::default());

        if let Some(access_token) = &self.access_token {
            request.metadata_mut().insert(
                "authorization",
                tonic::metadata::MetadataValue::from_str(
                    &format!("Bearer {}", access_token.value)
                )?,
            );
        }

        let response = self.searcher_client.get_tip_addresses(request).await?;

        Ok(response.into_inner())
    }
}
