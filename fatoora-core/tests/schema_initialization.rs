use fatoora_core::invoice::validation::Validator;
use std::{
    process::Command,
    sync::Barrier,
    time::{Duration, Instant},
};

const CHILD: &str = "FATOORA_SCHEMA_INITIALIZATION_CHILD";
const INVOICE: &str = include_str!("fixtures/invoices/sample-simplified-invoice.xml");

#[test]
fn concurrent_first_schema_use_completes() {
    if std::env::var_os(CHILD).is_some() {
        // No other test or XML operation may warm the schema cache before these calls.
        let start = Barrier::new(8);
        std::thread::scope(|scope| {
            for _ in 0..8 {
                let start = &start;
                scope.spawn(move || {
                    start.wait();
                    Validator::new().validate_xml(INVOICE).unwrap();
                });
            }
        });
        return;
    }

    // Fresh processes exercise initialization even when the suite has already
    // parsed XML. Bound the wait so a deadlock fails CI instead of hanging it.
    for _ in 0..4 {
        let mut child = Command::new(std::env::current_exe().unwrap())
            .args([
                "--exact",
                "concurrent_first_schema_use_completes",
                "--nocapture",
            ])
            .env(CHILD, "1")
            .spawn()
            .unwrap();
        let deadline = Instant::now() + Duration::from_secs(30);
        loop {
            if let Some(status) = child.try_wait().unwrap() {
                assert!(
                    status.success(),
                    "schema initialization child failed: {status}"
                );
                break;
            }
            if Instant::now() >= deadline {
                child.kill().unwrap();
                child.wait().unwrap();
                panic!("concurrent first schema use exceeded 30 seconds");
            }
            std::thread::sleep(Duration::from_millis(20));
        }
    }
}
