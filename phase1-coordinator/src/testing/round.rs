use crate::{environment::Environment, objects::Round};

use chrono::{DateTime, TimeZone, Utc};
use once_cell::sync::Lazy;
use serde_diff::{Diff, SerdeDiff};

/// Version number for testing purposes only.
pub static TEST_VERSION: u64 = Environment::Test.version();

/// Contributor ID 1 for testing purposes only.
pub static TEST_CONTRIBUTOR_ID_1: &str = "test_contributor";

/// Verifier ID 1 for testing purposes only.
pub static TEST_VERIFIER_ID_1: &str = "test_verifier";

/// Verified base URL 1 for testing purposes only.
pub static TEST_VERIFIED_BASE_URL_1: &str = "http://localhost:8080";

lazy_static! {
    /// Round start datetime for testing purposes only.
    pub static ref TEST_STARTED_AT: DateTime<Utc> = Utc.ymd(1970, 1, 1).and_hms(0, 1, 1);

    /// Contributor IDs for testing purposes only.
    pub static ref TEST_CONTRIBUTOR_IDS: Lazy<Vec<String>> = Lazy::new(|| vec![TEST_CONTRIBUTOR_ID_1.to_string()]);

    /// Verifier IDs for testing purposes only.
    pub static ref TEST_VERIFIER_IDS: Lazy<Vec<String>> =  Lazy::new(|| vec![TEST_VERIFIER_ID_1.to_string()]);

    /// Chunk verifier IDs for testing purposes only.
    pub static ref TEST_CHUNK_VERIFIER_IDS: Lazy<Vec<String>> = Lazy::new(|| (0..Environment::Test.number_of_chunks()).into_iter().map(|_| TEST_VERIFIER_IDS[0].clone()).collect());

    /// Chunk verified base URLs for testing purposes only.
    pub static ref TEST_CHUNK_VERIFIED_BASE_URLS: Lazy<Vec<&'static str>> = Lazy::new(|| (0..Environment::Test.number_of_chunks()).into_iter().map(|_| TEST_VERIFIED_BASE_URL_1).collect());
}

/// Loads the reference JSON object with a serialized round for testing purposes only.
pub fn test_round_1_json() -> anyhow::Result<Round> {
    Ok(serde_json::from_str(include_str!("resources/test_round_1.json"))?)
}

/// Creates the first round for testing purposes only.
pub fn test_round_1() -> anyhow::Result<Round> {
    Ok(Round::new(
        TEST_VERSION,
        1, /* height */
        *TEST_STARTED_AT,
        &TEST_CONTRIBUTOR_IDS,
        &TEST_VERIFIER_IDS,
        &TEST_CHUNK_VERIFIER_IDS,
        &TEST_CHUNK_VERIFIED_BASE_URLS,
    )?)
}

/// Prints the difference in JSON objects between `a` and `b`.
pub fn print_diff<S: SerdeDiff>(a: &S, b: &S) {
    println!(
        "\nDifference(s) between left and right values\n-------------------------------------------\n{}\n",
        serde_json::to_string(&Diff::serializable(a, b)).unwrap()
    );
}

#[test]
fn test_round_1_matches() {
    let expected = test_round_1_json().unwrap();
    let candidate = test_round_1().unwrap();

    // Print the differences in JSON if they do not match.
    if candidate != expected {
        print_diff(&expected, &candidate);
    }
    assert_eq!(candidate, expected);
}