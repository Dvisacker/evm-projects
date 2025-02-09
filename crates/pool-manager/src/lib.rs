use alloy::network::Ethereum;
use alloy::primitives::Address;
use alloy::providers::Provider;
use alloy_chains::Chain;
use amms::amm::uniswap_v2::batch_request::get_v2_pool_data_batch_request;
use amms::amm::uniswap_v2::UniswapV2Pool;
use amms::amm::uniswap_v3::batch_request::get_v3_pool_data_batch_request;
use amms::amm::uniswap_v3::UniswapV3Pool;
use amms::amm::ve33::factory::Ve33Factory;
use amms::amm::AutomatedMarketMaker;
use amms::errors::AMMError;
use amms::{
    amm::{
        factory::Factory, uniswap_v2::factory::UniswapV2Factory,
        uniswap_v3::factory::UniswapV3Factory,
    },
    sync::{self},
};
use db::establish_connection;
use db::models::db_pool::DbPool;
use db::models::{NewDbPool, NewDbTag, NewDbUniV2Pool, NewDbUniV3Pool};
use db::queries::exchange::get_exchange_by_name;
use db::queries::tag::insert_tag;
use db::queries::uni_v2_pool::{
    batch_update_uni_v2_pool_active, batch_upsert_uni_v2_pools, get_uni_v2_pools,
};
use db::queries::uni_v3_pool::{
    batch_update_uni_v3_pool_active, batch_upsert_uni_v3_pools, get_uni_v3_pools,
};
use diesel::PgConnection;
use shared::evm_helpers::get_contract_creation_block;
use shared::pool_helpers::{db_pools_to_amms, extract_v2_pools, extract_v3_pools, filter_amms};
use std::sync::Arc;
use types::exchange::{ExchangeName, ExchangeType};

pub struct PoolStorageManager<P>
where
    P: Provider<Ethereum> + 'static,
{
    db_url: String,
    provider: Arc<P>,
}

impl<P> PoolStorageManager<P>
where
    P: Provider<Ethereum> + 'static,
{
    pub fn new(db_url: &str, provider: Arc<P>) -> Self {
        Self {
            db_url: db_url.to_string(),
            provider,
        }
    }

    pub async fn store_pools_from_factory(
        &self,
        chain: Chain,
        exchange_name: ExchangeName,
        factory_address: Address,
        from_block: Option<u64>,
        to_block: Option<u64>,
        step: u64,
        tag: Option<String>,
    ) -> Result<(), AMMError> {
        let mut conn = establish_connection(&self.db_url);
        let exchange = get_exchange_by_name(
            &mut conn,
            &chain.named().unwrap().to_string(),
            &exchange_name.to_string(),
        )
        .unwrap();

        let exchange_type =
            ExchangeType::from_str(&exchange.exchange_type).expect("Invalid exchange type");

        match exchange_type {
            ExchangeType::UniV2 => {
                self.store_univ2_pools_from_factory(chain, exchange_name, factory_address, tag)
                    .await
            }
            ExchangeType::UniV3 => {
                self.store_univ3_pools_from_factory(
                    chain,
                    exchange_name,
                    factory_address,
                    from_block,
                    to_block,
                    step,
                    tag,
                )
                .await
            }
            ExchangeType::Ve33 => {
                self.store_ve33_pools_from_factory(chain, exchange_name, factory_address, tag)
                    .await
            }
            _ => Err(AMMError::UnknownPoolType),
        }
    }

    pub async fn store_pools(
        &self,
        chain: Chain,
        exchange_name: ExchangeName,
        pool_addresses: Vec<Address>,
        tag: Option<String>,
    ) -> Result<(), AMMError> {
        let mut conn = establish_connection(&self.db_url);
        let exchange = get_exchange_by_name(
            &mut conn,
            &chain.named().unwrap().to_string(),
            &exchange_name.to_string(),
        )
        .unwrap();
        let exchange_type =
            ExchangeType::from_str(&exchange.exchange_type).expect("Invalid exchange type");

        match exchange_type {
            ExchangeType::UniV2 => {
                self.store_univ2_pools(chain, exchange_name, pool_addresses, tag)
                    .await
            }
            ExchangeType::UniV3 => {
                self.store_univ3_pools(chain, exchange_name, pool_addresses, tag)
                    .await
            }
            _ => Err(AMMError::UnknownPoolType),
        }
    }

    pub async fn store_univ3_pools(
        &self,
        chain: Chain,
        exchange_name: ExchangeName,
        pool_addresses: Vec<Address>,
        tag: Option<String>,
    ) -> Result<(), AMMError> {
        let mut conn = establish_connection(&self.db_url);
        let mut pools = pool_addresses
            .iter()
            .map(|pool_address| {
                let pool = UniswapV3Pool::new_empty(*pool_address, chain.named().unwrap()).unwrap();
                pool
            })
            .collect::<Vec<UniswapV3Pool>>();

        for chunk in pools.chunks_mut(50) {
            get_v3_pool_data_batch_request(chunk, None, self.provider.clone()).await?;

            let new_pools = chunk
                .iter_mut()
                .map(|pool| {
                    pool.exchange_type = ExchangeType::UniV3;
                    pool.exchange_name = exchange_name;
                    pool.chain = chain.named().unwrap();
                    pool.to_new_db_pool(tag.clone())
                })
                .filter_map(|db_pool| {
                    if let NewDbPool::UniV3(v3_pool) = db_pool {
                        Some(v3_pool)
                    } else {
                        None
                    }
                })
                .collect::<Vec<NewDbUniV3Pool>>();

            batch_upsert_uni_v3_pools(&mut conn, &new_pools).unwrap();
            tracing::info!("Inserted {:?} pools", new_pools.len());
        }

        Ok(())
    }

    pub async fn store_univ2_pools(
        &self,
        chain: Chain,
        exchange_name: ExchangeName,
        pool_addresses: Vec<Address>,
        tag: Option<String>,
    ) -> Result<(), AMMError> {
        let mut conn = establish_connection(&self.db_url);
        let mut pools = pool_addresses
            .iter()
            .map(|pool_address| {
                let pool = UniswapV2Pool::new_empty(*pool_address, chain.named().unwrap()).unwrap();
                pool
            })
            .collect::<Vec<UniswapV2Pool>>();

        for chunk in pools.chunks_mut(50) {
            get_v2_pool_data_batch_request(chunk, self.provider.clone()).await?;

            let new_pools = chunk
                .iter_mut()
                .map(|pool| {
                    pool.exchange_type = ExchangeType::UniV2;
                    pool.exchange_name = exchange_name;
                    pool.chain = chain.named().unwrap();
                    pool.to_new_db_pool(tag.clone())
                })
                .filter_map(|db_pool| {
                    if let NewDbPool::UniV2(v2_pool) = db_pool {
                        Some(v2_pool)
                    } else {
                        None
                    }
                })
                .collect::<Vec<NewDbUniV2Pool>>();

            batch_upsert_uni_v2_pools(&mut conn, &new_pools).unwrap();
            tracing::info!("Inserted {:?} pools", new_pools.len());
        }

        Ok(())
    }

    /// Stores Uniswap V3 pools in the database.
    ///
    /// This function fetches Uniswap V3 pools from logs within a specified block range,
    /// populates their data, and stores them in the database.
    pub async fn store_univ3_pools_from_factory(
        &self,
        chain: Chain,
        exchange_name: ExchangeName,
        factory_address: Address,
        from_block: Option<u64>,
        to_block: Option<u64>,
        step: u64,
        tag: Option<String>,
    ) -> Result<(), AMMError> {
        let mut conn = establish_connection(&self.db_url);

        let start_block = from_block.unwrap_or(0);
        let end_block = to_block.unwrap_or(self.provider.get_block_number().await.unwrap());

        if let Some(ref tag) = tag {
            insert_tag(&mut conn, &NewDbTag { name: tag.clone() }).unwrap();
        }

        let contract_creation_block = get_contract_creation_block(
            self.provider.clone(),
            factory_address,
            start_block,
            end_block,
        )
        .await
        .unwrap();

        let contract_creation_block = if contract_creation_block > start_block {
            contract_creation_block
        } else {
            start_block
        };

        let factory = UniswapV3Factory::new(factory_address, contract_creation_block);

        for block in (contract_creation_block..=end_block).step_by(step as usize) {
            tracing::info!("Fetching pools from block {:?}", block);

            let addresses = factory
                .get_pools_from_logs(block, block + step - 1, step, self.provider.clone())
                .await?
                .iter()
                .map(|pool| pool.address())
                .collect::<Vec<Address>>();

            self.store_univ3_pools(chain, exchange_name, addresses, tag.clone())
                .await?;
        }

        Ok(())
    }

    /// Stores Uniswap V2 pools in the database.
    pub async fn store_univ2_pools_from_factory(
        &self,
        chain: Chain,
        exchange_name: ExchangeName,
        factory_address: Address,
        tag: Option<String>,
    ) -> Result<(), AMMError> {
        let mut conn = establish_connection(&self.db_url);
        let factory = Factory::UniswapV2Factory(UniswapV2Factory::new(factory_address, 0, 3000));

        tracing::info!("Syncing uni-v2 like pools");

        let (amms, _) = sync::sync_amms(vec![factory], self.provider.clone(), None, 100000)
            .await
            .unwrap();

        let mut pools = extract_v2_pools(&amms);

        for mut chunk in pools.chunks_mut(50) {
            get_v2_pool_data_batch_request(&mut chunk, self.provider.clone()).await?;

            let new_pools = chunk
                .iter_mut()
                .map(|pool| {
                    pool.exchange_type = ExchangeType::UniV2;
                    pool.exchange_name = exchange_name;
                    pool.chain = chain.named().unwrap();
                    pool.to_new_db_pool(tag.clone())
                })
                .filter_map(|db_pool| {
                    if let NewDbPool::UniV2(v2_pool) = db_pool {
                        Some(v2_pool)
                    } else {
                        None
                    }
                })
                .collect::<Vec<NewDbUniV2Pool>>();

            batch_upsert_uni_v2_pools(&mut conn, &new_pools).unwrap();
            tracing::info!("Inserted {:?} pools", new_pools.len());
        }

        Ok(())
    }

    pub async fn store_ve33_pools_from_factory(
        &self,
        chain: Chain,
        exchange_name: ExchangeName,
        factory_address: Address,
        tag: Option<String>,
    ) -> Result<(), AMMError> {
        let mut conn = establish_connection(&self.db_url);
        let factory = Factory::Ve33Factory(Ve33Factory::new(factory_address, 0, 3000));

        let (amms, _) = sync::sync_amms(vec![factory], self.provider.clone(), None, 100000)
            .await
            .unwrap();

        let mut pools = extract_v2_pools(&amms);

        for mut chunk in pools.chunks_mut(50) {
            // add additional data such as the exchange name
            get_v2_pool_data_batch_request(&mut chunk, self.provider.clone()).await?;
            let new_pools = chunk
                .iter_mut()
                .map(|pool| {
                    pool.exchange_type = ExchangeType::Ve33;
                    pool.exchange_name = exchange_name;
                    pool.chain = chain.named().unwrap();
                    pool.to_new_db_pool(tag.clone())
                })
                .filter_map(|db_pool| {
                    if let NewDbPool::UniV2(v2_pool) = db_pool {
                        Some(v2_pool)
                    } else {
                        None
                    }
                })
                .collect::<Vec<NewDbUniV2Pool>>();

            // upsert pools in the database
            batch_upsert_uni_v2_pools(&mut conn, &new_pools).unwrap();
            tracing::info!("Inserted {:?} pools", new_pools.len());
        }

        Ok(())
    }

    async fn activate_v3_pools(
        &self,
        chain: Chain,
        mut conn: &mut PgConnection,
        exchange_name: ExchangeName,
        usd_threshold: f64,
    ) -> Result<(), AMMError> {
        let named_chain = chain.named().unwrap();

        let pools = get_uni_v3_pools(
            &mut conn,
            Some(&named_chain.to_string()),
            Some(&exchange_name.to_string()),
            None,
            None,
            None,
        )
        .unwrap();

        let v3_pools = pools
            .iter()
            .map(|pool| DbPool::UniV3((*pool).clone()))
            .collect::<Vec<DbPool>>();

        let amms = db_pools_to_amms(&v3_pools)?;

        tracing::info!("Got {:?} uni v3 pools", pools.len());

        for chunk in amms.chunks(1000) {
            let result = filter_amms(chain, usd_threshold, chunk.to_vec()).await;
            if result.is_err() {
                tracing::error!("Error filtering amms: {:?}", result.err());
                continue;
            }
            let active_pools = result.unwrap();
            let active_pools = active_pools
                .iter()
                .map(|amm| amm.address().to_string())
                .collect::<Vec<String>>();

            let inactive_pools = chunk
                .iter()
                .filter(|amm| !active_pools.contains(&amm.address().to_string()))
                .map(|amm| amm.address().to_string())
                .collect::<Vec<String>>();

            batch_update_uni_v3_pool_active(&mut conn, &active_pools, true).unwrap();
            batch_update_uni_v3_pool_active(&mut conn, &inactive_pools, false).unwrap();

            tracing::info!(
                "Processed pool chunk. Active pools: {:?}. Inactive pools: {:?}",
                active_pools.len(),
                inactive_pools.len()
            );
        }

        Ok(())
    }

    async fn activate_v2_pools(
        &self,
        chain: Chain,
        mut conn: &mut PgConnection,
        exchange_name: ExchangeName,
        usd_threshold: f64,
    ) -> Result<(), AMMError> {
        let named_chain = chain.named().unwrap();

        let pools = get_uni_v2_pools(
            &mut conn,
            Some(&named_chain.to_string()),
            Some(&exchange_name.to_string()),
            None,
            None,
            None,
        )
        .unwrap();

        let v2_pools = pools
            .iter()
            .map(|pool| DbPool::UniV2((*pool).clone()))
            .collect::<Vec<DbPool>>();

        let amms = db_pools_to_amms(&v2_pools)?;

        for chunk in amms.chunks(1000) {
            let result = filter_amms(chain, usd_threshold, chunk.to_vec()).await;
            if result.is_err() {
                tracing::error!("Error filtering amms: {:?}", result.err());
                continue;
            }
            let active_pools = result.unwrap();
            let active_pools = active_pools
                .iter()
                .map(|amm| amm.address().to_string())
                .collect::<Vec<String>>();

            let inactive_pools = chunk
                .iter()
                .filter(|amm| !active_pools.contains(&amm.address().to_string()))
                .map(|amm| amm.address().to_string())
                .collect::<Vec<String>>();

            batch_update_uni_v2_pool_active(&mut conn, &active_pools, true).unwrap();
            batch_update_uni_v2_pool_active(&mut conn, &inactive_pools, false).unwrap();

            tracing::info!(
                "Processed pool chunk. Active pools: {:?}. Inactive pools: {:?}",
                active_pools.len(),
                inactive_pools.len()
            );
        }

        Ok(())
    }

    /// Activates pools that meet a certain USD value threshold.
    pub async fn activate_pools(
        &self,
        chain: Chain,
        exchange_name: ExchangeName,
        usd_threshold: f64,
    ) -> Result<(), AMMError> {
        let mut conn = establish_connection(&self.db_url);

        let exchange = get_exchange_by_name(
            &mut conn,
            &chain.named().unwrap().to_string(),
            &exchange_name.to_string(),
        )
        .unwrap();

        let exchange_type =
            ExchangeType::from_str(&exchange.exchange_type).expect("Invalid exchange type");

        match exchange_type {
            ExchangeType::UniV2 => {
                self.activate_v2_pools(chain, &mut conn, exchange_name, usd_threshold)
                    .await?;
            }
            ExchangeType::UniV3 => {
                self.activate_v3_pools(chain, &mut conn, exchange_name, usd_threshold)
                    .await?;
            }
            _ => {
                return Err(AMMError::UnknownPoolType);
            }
        }

        Ok(())
    }
}
