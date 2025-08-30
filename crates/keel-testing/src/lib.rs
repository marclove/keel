use std::sync::Once;

static INIT: Once = Once::new();

pub fn init_test_logging() {
    INIT.call_once(|| {
        tracing_subscriber::fmt()
            .with_test_writer()
            .with_max_level(tracing::Level::DEBUG)
            .try_init()
            .ok();
    });
}

// Simplified mock containers for now
#[derive(Debug)]
pub struct TestContainers {
    pub postgres_port: u16,
    pub redis_port: u16,
}

impl TestContainers {
    pub fn new() -> anyhow::Result<Self> {
        // Mock implementation for now - we'll add real containers later
        Ok(Self { 
            postgres_port: 5432, 
            redis_port: 6379 
        })
    }

    pub fn postgres_connection_string(&self) -> String {
        format!("postgresql://postgres:postgres@127.0.0.1:{}/postgres", self.postgres_port)
    }

    pub fn redis_connection_string(&self) -> String {
        format!("redis://127.0.0.1:{}", self.redis_port)
    }
}

#[macro_export]
macro_rules! bdd_test {
    ($name:ident, $feature_path:expr) => {
        #[tokio::test]
        async fn $name() -> cucumber::Result<()> {
            use cucumber::{World as _, WorldInit};
            
            keel_testing::init_test_logging();
            
            let runner = TestWorld::cucumber()
                .before(|_feature, _rule, _scenario, world| {
                    Box::pin(async move {
                        world.containers = Some(
                            keel_testing::TestContainers::new()
                                .expect("Failed to start test containers")
                        );
                    })
                })
                .run($feature_path)
                .await;
                
            runner
        }
    };
}