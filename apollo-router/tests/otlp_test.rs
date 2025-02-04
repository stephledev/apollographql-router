mod common;
use std::result::Result;

use tower::BoxError;

use crate::common::IntegrationTest;
use crate::common::Telemetry;

#[ignore]
#[tokio::test(flavor = "multi_thread")]
async fn test_otlp_tracing() -> Result<(), BoxError> {
    let mut router = IntegrationTest::builder()
        .telemetry(Telemetry::Otlp)
        .config(include_str!("fixtures/otlp.router.yaml"))
        .build()
        .await;
    router.start().await;
    router.assert_started().await;
    router.run_query().await;
    Ok(())
}
