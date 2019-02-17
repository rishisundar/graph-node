use futures::future;
use std::env;
use std::str::FromStr;
use std::time::{Duration, Instant};

use graph::prelude::{GraphQlRunner as GraphQlRunnerTrait, *};
use graph_graphql::prelude::*;

use lazy_static::lazy_static;

/// GraphQL runner implementation for The Graph.
pub struct GraphQlRunner<S> {
    logger: Logger,
    store: Arc<S>,
}

lazy_static! {
    static ref GRAPHQL_QUERY_TIMEOUT: Option<Duration> = env::var("GRAPH_GRAPHQL_QUERY_TIMEOUT")
        .ok()
        .map(|s| Duration::from_secs(
            u64::from_str(&s)
                .unwrap_or_else(|_| panic!("failed to parse env var GRAPH_GRAPHQL_QUERY_TIMEOUT"))
        ));
}

impl<S> GraphQlRunner<S>
where
    S: Store,
{
    /// Creates a new query runner.
    pub fn new(logger: &Logger, store: Arc<S>) -> Self {
        GraphQlRunner {
            logger: logger.new(o!("component" => "GraphQlRunner")),
            store,
        }
    }
}

impl<S> GraphQlRunnerTrait for GraphQlRunner<S>
where
    S: Store,
{
    fn run_query(&self, query: Query) -> QueryResultFuture {
        let result = execute_query(
            &query,
            QueryExecutionOptions {
                logger: self.logger.clone(),
                resolver: StoreResolver::new(&self.logger, self.store.clone()),
                deadline: GRAPHQL_QUERY_TIMEOUT.map(|t| Instant::now() + t),
            },
        );
        Box::new(future::ok(result))
    }

    fn run_subscription(&self, subscription: Subscription) -> SubscriptionResultFuture {
        let result = execute_subscription(
            &subscription,
            SubscriptionExecutionOptions {
                logger: self.logger.clone(),
                resolver: StoreResolver::new(&self.logger, self.store.clone()),
                timeout: GRAPHQL_QUERY_TIMEOUT.clone(),
            },
        );

        Box::new(future::result(result))
    }
}
