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

// Simplified test databases for WASM compatibility
#[derive(Debug)]
pub struct TestDatabases {
    sqlite_path: std::path::PathBuf,
}

impl TestDatabases {
    pub fn new() -> anyhow::Result<Self> {
        // Use fixed paths since tempfile doesn't work with WASM
        let sqlite_path = std::path::PathBuf::from("test.db");

        Ok(Self { sqlite_path })
    }

    pub fn sqlite_connection_string(&self) -> String {
        format!("sqlite:{}", self.sqlite_path.display())
    }

    pub fn sqlite_path(&self) -> &std::path::Path {
        &self.sqlite_path
    }
}

// Keep old interface for backward compatibility during transition
#[derive(Debug)]
pub struct TestContainers {
    pub databases: TestDatabases,
}

impl TestContainers {
    pub fn new() -> anyhow::Result<Self> {
        Ok(Self {
            databases: TestDatabases::new()?,
        })
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
                                .expect("Failed to start test containers"),
                        );
                    })
                })
                .run($feature_path)
                .await;

            runner
        }
    };
}
