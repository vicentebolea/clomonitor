use serde::{Deserialize, Serialize};

use super::{CheckOutput, check::CheckId, checks::*};

/// Linter report.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct Report {
    pub code_vulnerabilities: CodeVulnerabilities,
    pub maintenance: Maintenance,
    pub testing: ContinuousTesting,
    pub source: SourceRisk,
    pub build: BuildRisk,
}

impl Report {
    pub(crate) fn apply_exemptions(&mut self) {
        // No cross-check exemptions needed for scorecard-only checks.
    }
}

/// Code Vulnerabilities section (OpenSSF Scorecard).
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct CodeVulnerabilities {
    pub vulnerabilities: Option<CheckOutput>,
}

#[rustfmt::skip]
section_impl!(
    CodeVulnerabilities,
    (vulnerabilities, vulnerabilities)
);

/// Maintenance section (OpenSSF Scorecard).
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct Maintenance {
    pub dependency_update_tool: Option<CheckOutput>,
    pub maintained: Option<CheckOutput>,
    #[serde(rename = "security_policy")]
    pub security_policy_sc: Option<CheckOutput>,
    #[serde(rename = "license")]
    pub license_sc: Option<CheckOutput>,
    pub cii_best_practices: Option<CheckOutput>,
}

#[rustfmt::skip]
section_impl!(
    Maintenance,
    (dependency_update_tool, dependency_update_tool),
    (maintained, maintained),
    (security_policy_sc, security_policy_sc),
    (license_sc, license_sc),
    (cii_best_practices, cii_best_practices)
);

/// Continuous Testing section (OpenSSF Scorecard).
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct ContinuousTesting {
    pub ci_tests: Option<CheckOutput>,
    pub fuzzing: Option<CheckOutput>,
    pub sast: Option<CheckOutput>,
}

#[rustfmt::skip]
section_impl!(
    ContinuousTesting,
    (ci_tests, ci_tests),
    (fuzzing, fuzzing),
    (sast, sast)
);

/// Source Risk section (OpenSSF Scorecard).
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct SourceRisk {
    pub binary_artifacts: Option<CheckOutput>,
    pub branch_protection: Option<CheckOutput>,
    pub dangerous_workflow: Option<CheckOutput>,
    pub code_review: Option<CheckOutput>,
    #[serde(rename = "contributors")]
    pub contributors_sc: Option<CheckOutput>,
}

#[rustfmt::skip]
section_impl!(
    SourceRisk,
    (binary_artifacts, binary_artifacts),
    (branch_protection, branch_protection),
    (dangerous_workflow, dangerous_workflow),
    (code_review, code_review),
    (contributors_sc, contributors_sc)
);

/// Build Risk section (OpenSSF Scorecard).
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct BuildRisk {
    pub pinned_dependencies: Option<CheckOutput>,
    pub token_permissions: Option<CheckOutput>,
    pub packaging: Option<CheckOutput>,
    pub signed_releases: Option<CheckOutput>,
}

#[rustfmt::skip]
section_impl!(
    BuildRisk,
    (pinned_dependencies, pinned_dependencies),
    (token_permissions, token_permissions),
    (packaging, packaging),
    (signed_releases, signed_releases)
);

/// Prepare the implementation for a section in the report.
macro_rules! section_impl {
    ( $section:ident, $( ($field:ident, $module:ident) ),* ) => {
        impl $section {
            /// Returns (CheckId, normalized_score 0.0-1.0) for each check that was run.
            pub(crate) fn check_scores(&self) -> Vec<(CheckId, f64)> {
                let mut scores = Vec::new();
                $(
                if let Some(ref output) = self.$field {
                    let score = if output.exempt {
                        1.0
                    } else if let Some(sc_score) = output.scorecard_score {
                        sc_score.max(0.0) / 10.0
                    } else {
                        if output.passed { 1.0 } else { 0.0 }
                    };
                    scores.push(($module::ID, score));
                }
                )*
                scores
            }
        }
    };
}
use section_impl;
