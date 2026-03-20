use serde::{Deserialize, Serialize};

use super::{CheckOutput, check::CheckId, checks::*};

/// Linter report.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Report {
    pub project: Holistic,
    pub source: SourceCode,
    pub build: BuildProcess,
}

impl Report {
    pub(crate) fn apply_exemptions(&mut self) {
        // No cross-check exemptions needed for scorecard-only checks.
    }
}

/// Project section: holistic security practices (OpenSSF Scorecard).
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Holistic {
    pub maintained: Option<CheckOutput>,
    #[serde(rename = "contributors")]
    pub contributors_sc: Option<CheckOutput>,
    pub cii_best_practices: Option<CheckOutput>,
    #[serde(rename = "security_policy")]
    pub security_policy_sc: Option<CheckOutput>,
    #[serde(rename = "license")]
    pub license_sc: Option<CheckOutput>,
}

#[rustfmt::skip]
section_impl!(
    Holistic,
    (maintained, maintained),
    (contributors_sc, contributors_sc),
    (cii_best_practices, cii_best_practices),
    (security_policy_sc, security_policy_sc),
    (license_sc, license_sc)
);

/// Source section: source code risk assessment (OpenSSF Scorecard).
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct SourceCode {
    pub code_review: Option<CheckOutput>,
    pub binary_artifacts: Option<CheckOutput>,
    pub dangerous_workflow: Option<CheckOutput>,
    pub sast: Option<CheckOutput>,
    pub vulnerabilities: Option<CheckOutput>,
}

#[rustfmt::skip]
section_impl!(
    SourceCode,
    (code_review, code_review),
    (binary_artifacts, binary_artifacts),
    (dangerous_workflow, dangerous_workflow),
    (sast, sast),
    (vulnerabilities, vulnerabilities)
);

/// Build section: build process risk assessment (OpenSSF Scorecard).
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct BuildProcess {
    pub branch_protection: Option<CheckOutput>,
    pub ci_tests: Option<CheckOutput>,
    pub dependency_update_tool: Option<CheckOutput>,
    pub fuzzing: Option<CheckOutput>,
    pub pinned_dependencies: Option<CheckOutput>,
    pub signed_releases: Option<CheckOutput>,
    pub token_permissions: Option<CheckOutput>,
    pub packaging: Option<CheckOutput>,
    #[serde(rename = "sbom")]
    pub sbom_sc: Option<CheckOutput>,
}

#[rustfmt::skip]
section_impl!(
    BuildProcess,
    (branch_protection, branch_protection),
    (ci_tests, ci_tests),
    (dependency_update_tool, dependency_update_tool),
    (fuzzing, fuzzing),
    (pinned_dependencies, pinned_dependencies),
    (signed_releases, signed_releases),
    (token_permissions, token_permissions),
    (packaging, packaging),
    (sbom_sc, sbom_sc)
);

/// Prepare the implementation for a section in the report.
macro_rules! section_impl {
    ( $section:ident, $( ($field:ident, $module:ident) ),* ) => {
        impl $section {
            pub(crate) fn available(&self) -> Vec<CheckId> {
                let mut checks = Vec::new();
                $(
                if self.$field.as_ref().is_some() {
                    checks.push($module::ID);
                }
                )*
                checks
            }

            pub(crate) fn passed_or_exempt(&self) -> Vec<CheckId> {
                let mut checks = Vec::new();
                $(
                if self.$field.as_ref().map_or(false, |o| o.passed || o.exempt) {
                    checks.push($module::ID);
                }
                )*
                checks
            }
        }
    };
}
use section_impl;
