use serde::{Deserialize, Serialize};

use super::{CheckOutput, check::CheckId, checks::*};

/// Linter report.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Report {
    pub project: Project,
    pub source: Source,
    pub build: Build,
}

impl Report {
    pub(crate) fn apply_exemptions(&mut self) {
        // No cross-check exemptions needed for scorecard-only checks.
    }
}

/// Project section: holistic security practices (OpenSSF Scorecard).
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Project {
    pub maintained: Option<CheckOutput>,
}

#[rustfmt::skip]
section_impl!(
    Project,
    maintained
);

/// Source section: source code risk assessment (OpenSSF Scorecard).
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Source {
    pub code_review: Option<CheckOutput>,
    pub dangerous_workflow: Option<CheckOutput>,
    pub token_permissions: Option<CheckOutput>,
}

#[rustfmt::skip]
section_impl!(
    Source,
    code_review,
    dangerous_workflow,
    token_permissions
);

/// Build section: build process risk assessment (OpenSSF Scorecard).
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Build {
    pub binary_artifacts: Option<CheckOutput>,
    pub dependency_update_tool: Option<CheckOutput>,
    pub signed_releases: Option<CheckOutput>,
}

#[rustfmt::skip]
section_impl!(
    Build,
    binary_artifacts,
    dependency_update_tool,
    signed_releases
);

/// Prepare the implementation for a section in the report.
macro_rules! section_impl {
    ( $section:ident, $( $check:ident ),* ) => {
        impl $section {
            pub(crate) fn available(&self) -> Vec<CheckId> {
                let mut checks = Vec::new();
                $(
                if self.$check.as_ref().is_some() {
                    checks.push($check::ID);
                }
                )*
                checks
            }

            pub(crate) fn passed_or_exempt(&self) -> Vec<CheckId> {
                let mut checks = Vec::new();
                $(
                if self.$check.as_ref().map_or(false, |o| o.passed || o.exempt) {
                    checks.push($check::ID);
                }
                )*
                checks
            }
        }
    };
}
use section_impl;
