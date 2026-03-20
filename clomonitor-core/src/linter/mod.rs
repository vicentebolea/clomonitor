use std::{fmt, path::PathBuf, sync::Arc};

use anyhow::Result;
use async_trait::async_trait;
use clap::ValueEnum;
#[cfg(feature = "mocks")]
use mockall::automock;
use postgres_types::ToSql;
use serde::{Deserialize, Serialize};
use time::Date;

use self::{
    check::*,
    checks::util::helpers::{find_exemption, should_skip_check},
};

mod check;
mod checks;
mod metadata;
mod report;

pub use self::{
    check::{CheckId, CheckOutput},
    report::*,
};
pub use checks::datasource::github::setup_http_client as setup_github_http_client;
pub(crate) use checks::*;

/// Type alias to represent a Linter trait object.
pub type DynLinter = Arc<dyn Linter + Send + Sync>;

/// Trait that defines some operations a Linter implementation must support.
#[async_trait]
#[cfg_attr(feature = "mocks", automock)]
pub trait Linter {
    /// Lint the repository provided returning a report with the results.
    async fn lint(&self, input: &LinterInput) -> Result<Report>;
}

/// Input used by the linter to perform its operations.
#[derive(Debug, Clone, Default)]
pub struct LinterInput {
    pub project: Option<Project>,
    pub root: PathBuf,
    pub url: String,
    pub check_sets: Vec<CheckSet>,
    pub github_token: String,
}

/// Project's details
#[derive(Debug, Clone, Default)]
pub struct Project {
    pub name: String,
    pub accepted_at: Option<Date>,
    pub maturity: Option<String>,
    pub foundation: Foundation,
}

/// Foundation's details
#[derive(Debug, Clone, Default)]
pub struct Foundation {
    pub foundation_id: String,
    pub landscape_url: Option<String>,
}

/// Check sets define a set of checks that will be run on a given repository.
/// Multiple check sets can be assigned to a repository.
#[derive(Debug, Clone, PartialEq, Eq, Hash, ValueEnum, Serialize, Deserialize, ToSql)]
#[serde(rename_all = "kebab-case")]
#[postgres(name = "check_set")]
pub enum CheckSet {
    #[postgres(name = "code")]
    Code,
    #[postgres(name = "code-lite")]
    CodeLite,
    #[postgres(name = "community")]
    Community,
    #[postgres(name = "docs")]
    Docs,
}

impl fmt::Display for CheckSet {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let output = match self {
            Self::Code => "CODE",
            Self::CodeLite => "CODE-LITE",
            Self::Community => "COMMUNITY",
            Self::Docs => "DOCS",
        };
        write!(f, "{output}")
    }
}

/// CLOMonitor core linter (Linter implementation).
pub struct CoreLinter;

#[allow(clippy::new_without_default)]
impl CoreLinter {
    /// Create a new CoreLinter instance.
    #[must_use]
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl Linter for CoreLinter {
    async fn lint(&self, li: &LinterInput) -> Result<Report> {
        // Prepare check input
        let ci = CheckInput::new(li).await?;

        // Run OpenSSF Scorecard checks and build report
        let mut report = Report {
            project: Holistic {
                maintained: run!(maintained, &ci),
                contributors_sc: run!(contributors_sc, &ci),
                cii_best_practices: run!(cii_best_practices, &ci),
                security_policy_sc: run!(security_policy_sc, &ci),
                license_sc: run!(license_sc, &ci),
            },
            source: SourceCode {
                code_review: run!(code_review, &ci),
                binary_artifacts: run!(binary_artifacts, &ci),
                dangerous_workflow: run!(dangerous_workflow, &ci),
                sast: run!(sast, &ci),
                vulnerabilities: run!(vulnerabilities, &ci),
            },
            build: BuildProcess {
                branch_protection: run!(branch_protection, &ci),
                ci_tests: run!(ci_tests, &ci),
                dependency_update_tool: run!(dependency_update_tool, &ci),
                fuzzing: run!(fuzzing, &ci),
                pinned_dependencies: run!(pinned_dependencies, &ci),
                signed_releases: run!(signed_releases, &ci),
                token_permissions: run!(token_permissions, &ci),
                packaging: run!(packaging, &ci),
                sbom_sc: run!(sbom_sc, &ci),
            },
        };
        report.apply_exemptions();

        Ok(report)
    }
}
