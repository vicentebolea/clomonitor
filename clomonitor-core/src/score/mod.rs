use serde::{Deserialize, Serialize};

use crate::linter::*;

/// Score information.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct Score {
    pub global: f64,
    pub global_weight: usize,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub project: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub project_weight: Option<usize>,

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

    // Sections
    (score.project, score.project_weight) = calculate_section(
        &report.project.available(),
        &report.project.passed_or_exempt(),
    );
    (score.source, score.source_weight) = calculate_section(
        &report.source.available(),
        &report.source.passed_or_exempt(),
    );
    (score.build, score.build_weight) = calculate_section(
        &report.build.available(),
        &report.build.passed_or_exempt(),
    );

    // Global
    let sections_scores = &[score.project, score.source, score.build];
    let sections_weights = &[score.project_weight, score.source_weight, score.build_weight];
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

/// Calculate score and weight for a report's section from the checks provided.
fn calculate_section(
    checks_available: &[CheckId],
    checks_passed_or_exempt: &[CheckId],
) -> (Option<f64>, Option<usize>) {
    // Calculate section weight
    let weight = checks_available
        .iter()
        .fold(0, |weight, check_id| weight + CHECKS[check_id].weight);
    if weight == 0 {
        return (None, None);
    }

    // Calculate section score
    let score = checks_passed_or_exempt.iter().fold(0.0, |score, check_id| {
        score + CHECKS[check_id].weight as f64 / weight as f64 * 100.0
    });

    (Some(score), Some(weight))
}

/// Merge the scores provided into a single score.
#[must_use]
pub fn merge(scores: &[Score]) -> Score {
    let mut global_weights_sum = 0;
    let mut project_weights_sum = 0;
    let mut source_weights_sum = 0;
    let mut build_weights_sum = 0;
    for score in scores {
        global_weights_sum += score.global_weight;
        project_weights_sum += score.project_weight.unwrap_or_default();
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
        m.project = merge(
            m.project,
            s.project,
            s.project_weight.unwrap_or_default() as f64 / project_weights_sum as f64,
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

    #[test]
    fn calculate_report_with_all_checks_passed_got_max_score() {
        // maintained(3) + code_review(3) + dangerous_workflow(2) + token_permissions(1)
        // + binary_artifacts(2) + dependency_update_tool(2) + signed_releases(2) = 15
        assert_eq!(
            calculate(&Report {
                project: Holistic {
                    maintained: Some(CheckOutput::passed()),
                },
                source: SourceCode {
                    code_review: Some(CheckOutput::passed()),
                    dangerous_workflow: Some(CheckOutput::passed()),
                    token_permissions: Some(CheckOutput::passed()),
                },
                build: BuildProcess {
                    binary_artifacts: Some(CheckOutput::passed()),
                    dependency_update_tool: Some(CheckOutput::passed()),
                    signed_releases: Some(CheckOutput::passed()),
                },
            }),
            Score {
                global: 100.0,
                global_weight: 15,
                project: Some(100.0),
                project_weight: Some(3),
                source: Some(100.0),
                source_weight: Some(6),
                build: Some(100.0),
                build_weight: Some(6),
            }
        );
    }

    #[test]
    fn calculate_report_with_all_checks_non_passed_got_min_score() {
        assert_eq!(
            calculate(&Report {
                project: Holistic {
                    maintained: Some(CheckOutput::not_passed()),
                },
                source: SourceCode {
                    code_review: Some(CheckOutput::not_passed()),
                    dangerous_workflow: Some(CheckOutput::not_passed()),
                    token_permissions: Some(CheckOutput::not_passed()),
                },
                build: BuildProcess {
                    binary_artifacts: Some(CheckOutput::not_passed()),
                    dependency_update_tool: Some(CheckOutput::not_passed()),
                    signed_releases: Some(CheckOutput::not_passed()),
                },
            }),
            Score {
                global: 0.0,
                global_weight: 15,
                project: Some(0.0),
                project_weight: Some(3),
                source: Some(0.0),
                source_weight: Some(6),
                build: Some(0.0),
                build_weight: Some(6),
            }
        );
    }
}
