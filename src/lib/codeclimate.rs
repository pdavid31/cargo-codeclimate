use std::path::PathBuf;

use cargo_metadata::diagnostic::{DiagnosticCode, DiagnosticLevel};
use serde::Serialize;
use sha::{
    sha1::Sha1,
    utils::{Digest, DigestExt},
};

/// Severity enum representing lint severity levels
#[derive(Debug, Serialize)]
#[serde(rename_all(serialize = "lowercase"))]
pub enum Severity {
    Info,
    Minor,
    Major,
    Critical,
    Blocker,
}

/// Severity level conversion from cargo's DiagnosticLevel
impl From<DiagnosticLevel> for Severity {
    fn from(value: DiagnosticLevel) -> Self {
        match value {
            DiagnosticLevel::Help | DiagnosticLevel::Note => Severity::Info,
            DiagnosticLevel::Warning => Severity::Minor,
            DiagnosticLevel::Error => Severity::Major,
            _ => Severity::Info,
        }
    }
}

/// Lines report item giving information on affected lines
#[derive(Debug, Serialize)]
pub struct Lines {
    begin: usize,
}

/// Location report item giving information on affected file and affected lines
#[derive(Debug, Serialize)]
pub struct Location {
    path: PathBuf,
    lines: Lines,
}

/// CodeClimate report item implementing a subset of the Code Climate spec per
/// [GitLab CI Code Quality documentation](https://docs.gitlab.com/ee/ci/testing/code_quality.html#implement-a-custom-tool)
#[derive(Debug, Serialize)]
pub struct CodeClimate {
    check_name: String,
    description: String,
    fingerprint: String,
    severity: Severity,
    location: Location,
}

impl CodeClimate {
    /// CodeClimate constructor
    pub fn new(
        check: Option<DiagnosticCode>,
        message: &str,
        path: &str,
        line: usize,
        severity: DiagnosticLevel,
    ) -> Self {
        Self {
            // extract the diagnostic code or fallback to default "unset code"
            check_name: check.map(|x| x.code).unwrap_or(String::from("unset code")),
            description: String::from(message),
            fingerprint: Self::calculate_fingerprint(message, path, line),
            location: Location {
                path: PathBuf::from(path),
                lines: Lines { begin: line },
            },
            severity: Severity::from(severity),
        }
    }

    /// Calculate sha1 fingerprint using the message, file name and affected line
    fn calculate_fingerprint(message: &str, file: &str, line: usize) -> String {
        // construct the fingerprint string
        let fingerprint = format!("{}-{}-{}", message, file, line);
        // construct the sha1 object
        let mut sha1 = Sha1::default();
        // create the sha1 digest
        let digest = sha1.digest(fingerprint.as_bytes());

        // convert the digest to hex string
        digest.to_hex()
    }
}
