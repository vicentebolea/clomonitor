use serde::{Deserialize, Serialize};

use crate::linter::*;

/// Score information.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct Score {
    pub global: f64,
    pub global_weight: usize,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub code_vulnerabilities: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code_vulnerabilities_weight: Option<usize>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub maintenance: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maintenance_weight: Option<usize>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub testing: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub testing_weight: Option<usize>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_weight: Option<usize>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub build: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub build_weight: Option<usize>,
}

impl Score {
    /// Return the score's global value.
    #[must_use]
    pub fn global(&self) -> f64 {
        self.global
    }

    /// Return the score's rating (a, b, c or d).
    #[must_use]
    pub fn rating(&self) -> char {
        rating(self.global())
    }
}

/// Calculate score for the given linter report.
#[must_use]
pub fn calculate(report: &Report) -> Score {
    let mut score = Score::default();

    (score.code_vulnerabilities, score.code_vulnerabilities_weight) =
        calculate_section(&report.code_vulnerabilities.check_scores());
    (score.maintenance, score.maintenance_weight) =
        calculate_section(&report.maintenance.check_scores());
    (score.testing, score.testing_weight) =
        calculate_section(&report.testing.check_scores());
    (score.source, score.source_weight) =
        calculate_section(&report.source.check_scores());
    (score.build, score.build_weight) =
        calculate_section(&report.build.check_scores());

    // Global
    let sections_scores = &[
        score.code_vulnerabilities,
        score.maintenance,
        score.testing,
        score.source,
        score.build,
    ];
    let sections_weights = &[
        score.code_vulnerabilities_weight,
        score.maintenance_weight,
        score.testing_weight,
        score.source_weight,
        score.build_weight,
    ];
    score.global_weight = sections_weights
        .iter()
        .fold(0, |gw, sw| gw + sw.unwrap_or_default());
    score.global = sections_scores
        .iter()
        .zip(sections_weights.iter())
        .fold(0.0, |gs, (ss, sw)| {
            let k = sw.unwrap_or_default() as f64 / score.global_weight as f64;
            gs + ss.unwrap_or_default() * k
        });

    score
}

/// Calculate score and weight for a report's section.
/// Each entry is (CheckId, normalized_score) where normalized_score is 0.0-1.0.
fn calculate_section(check_scores: &[(CheckId, f64)]) -> (Option<f64>, Option<usize>) {
    let weight = check_scores
        .iter()
        .fold(0, |weight, (check_id, _)| weight + CHECKS[check_id].weight);
    if weight == 0 {
        return (None, None);
    }
    let score = check_scores.iter().fold(0.0, |score, (check_id, normalized)| {
        score + CHECKS[check_id].weight as f64 / weight as f64 * normalized * 100.0
    });
    (Some(score), Some(weight))
}

/// Merge the scores provided into a single score.
#[must_use]
pub fn merge(scores: &[Score]) -> Score {
    let mut global_weights_sum = 0;
    let mut code_vuln_weights_sum = 0;
    let mut maintenance_weights_sum = 0;
    let mut testing_weights_sum = 0;
    let mut source_weights_sum = 0;
    let mut build_weights_sum = 0;
    for score in scores {
        global_weights_sum += score.global_weight;
        code_vuln_weights_sum += score.code_vulnerabilities_weight.unwrap_or_default();
        maintenance_weights_sum += score.maintenance_weight.unwrap_or_default();
        testing_weights_sum += score.testing_weight.unwrap_or_default();
        source_weights_sum += score.source_weight.unwrap_or_default();
        build_weights_sum += score.build_weight.unwrap_or_default();
    }

    let merge = |merged: Option<f64>, score: Option<f64>, k: f64| -> Option<f64> {
        if let Some(v) = score {
            return match merged {
                Some(mv) => Some(mv + v * k),
                None => Some(v * k),
            };
        }
        merged
    };

    let mut m = Score::default();
    for s in scores {
        m.global += s.global * (s.global_weight as f64 / global_weights_sum as f64);
        m.code_vulnerabilities = merge(
            m.code_vulnerabilities,
            s.code_vulnerabilities,
            s.code_vulnerabilities_weight.unwrap_or_default() as f64 / code_vuln_weights_sum as f64,
        );
        m.maintenance = merge(
            m.maintenance,
            s.maintenance,
            s.maintenance_weight.unwrap_or_default() as f64 / maintenance_weights_sum as f64,
        );
        m.testing = merge(
            m.testing,
            s.testing,
            s.testing_weight.unwrap_or_default() as f64 / testing_weights_sum as f64,
        );
        m.source = merge(
            m.source,
            s.source,
            s.source_weight.unwrap_or_default() as f64 / source_weights_sum as f64,
        );
        m.build = merge(
            m.build,
            s.build,
            s.build_weight.unwrap_or_default() as f64 / build_weights_sum as f64,
        );
    }

    m
}

/// Return the score's rating (a, b, c or d).
#[must_use]
pub fn rating(score: f64) -> char {
    match score.round() as usize {
        75..=100 => 'a',
        50..=74 => 'b',
        25..=49 => 'c',
        0..=24 => 'd',
        _ => '?',
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn score_global() {
        assert!(
            (Score {
                global: 10.0,
                ..Score::default()
            }
            .global()
                - 10.0)
                .abs()
                < f64::EPSILON
        );
    }

    #[test]
    fn score_rating() {
        assert_eq!(
            Score {
                global: 80.0,
                ..Score::default()
            }
            .rating(),
            'a'
        );
    }

    #[test]
    fn rating_returns_correct_level() {
        assert_eq!(rating(80.0), 'a');
        assert_eq!(rating(75.0), 'a');
        assert_eq!(rating(74.0), 'b');
        assert_eq!(rating(50.0), 'b');
        assert_eq!(rating(49.0), 'c');
        assert_eq!(rating(25.0), 'c');
        assert_eq!(rating(20.0), 'd');
    }

    fn check_with_sc_score(sc_score: f64) -> CheckOutput {
        CheckOutput {
            scorecard_score: Some(sc_score),
            passed: sc_score > 5.0,
            ..CheckOutput::default()
        }
    }

    #[test]
    fn calculate_report_all_checks_perfect() {
        let check = || Some(check_with_sc_score(10.0));
        let result = calculate(&Report {
            code_vulnerabilities: CodeVulnerabilities {
                vulnerabilities: check(),
            },
            maintenance: Maintenance {
                dependency_update_tool: check(),
                maintained: check(),
                security_policy_sc: check(),
                license_sc: check(),
                cii_best_practices: check(),
            },
            testing: ContinuousTesting {
                ci_tests: check(),
                fuzzing: check(),
                sast: check(),
            },
            source: SourceRisk {
                binary_artifacts: check(),
                branch_protection: check(),
                dangerous_workflow: check(),
                code_review: check(),
                contributors_sc: check(),
            },
            build: BuildRisk {
                pinned_dependencies: check(),
                token_permissions: check(),
                packaging: check(),
                signed_releases: check(),
            },
        });
        assert!((result.global - 100.0).abs() < 1e-6);
        assert!((result.code_vulnerabilities.unwrap() - 100.0).abs() < 1e-6);
        assert!((result.maintenance.unwrap() - 100.0).abs() < 1e-6);
        assert!((result.testing.unwrap() - 100.0).abs() < 1e-6);
        assert!((result.source.unwrap() - 100.0).abs() < 1e-6);
        assert!((result.build.unwrap() - 100.0).abs() < 1e-6);
        // weights: 1 + 5 + 3 + 5 + 4 = 18
        assert_eq!(result.global_weight, 18);
    }

    #[test]
    fn calculate_report_all_checks_zero() {
        let check = || Some(check_with_sc_score(0.0));
        let result = calculate(&Report {
            code_vulnerabilities: CodeVulnerabilities {
                vulnerabilities: check(),
            },
            maintenance: Maintenance {
                dependency_update_tool: check(),
                maintained: check(),
                security_policy_sc: check(),
                license_sc: check(),
                cii_best_practices: check(),
            },
            testing: ContinuousTesting {
                ci_tests: check(),
                fuzzing: check(),
                sast: check(),
            },
            source: SourceRisk {
                binary_artifacts: check(),
                branch_protection: check(),
                dangerous_workflow: check(),
                code_review: check(),
                contributors_sc: check(),
            },
            build: BuildRisk {
                pinned_dependencies: check(),
                token_permissions: check(),
                packaging: check(),
                signed_releases: check(),
            },
        });
        assert!((result.global - 0.0).abs() < 1e-6);
        assert_eq!(result.global_weight, 18);
    }
}
