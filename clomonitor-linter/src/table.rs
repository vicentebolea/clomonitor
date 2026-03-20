use std::{fs, io};

use anyhow::Result;
use clomonitor_core::{
    linter::{CheckOutput, Report},
    score::Score,
};
use comfy_table::{Table, modifiers::UTF8_ROUND_CORNERS, presets::UTF8_FULL, *};

use crate::Args;

const SUCCESS_SYMBOL: char = '✓';
const FAILURE_SYMBOL: char = '✗';
const WARNING_SYMBOL: char = '!';
const NOT_APPLICABLE_MSG: &str = "n/a";
const EXEMPT_MSG: &str = "Exempt";

/// Print the linter results provided.
#[allow(clippy::too_many_lines)]
pub(crate) fn display(
    report: &Report,
    score: &Score,
    args: &Args,
    w: &mut impl io::Write,
) -> Result<()> {
    writeln!(w, "\nCLOMonitor linter results\n")?;

    // Repository information
    let local_path = match fs::canonicalize(&args.path) {
        Ok(cp) => cp.to_string_lossy().to_string(),
        Err(_) => args.path.to_string_lossy().to_string(),
    };
    writeln!(w, "Repository information\n")?;
    let mut repo_info = new_table();
    repo_info
        .load_preset(UTF8_FULL)
        .apply_modifier(UTF8_ROUND_CORNERS)
        .add_row(vec![cell_entry("Local path"), cell_entry(&local_path)])
        .add_row(vec![cell_entry("Remote url"), cell_entry(&args.url)])
        .add_row(vec![
            cell_entry("Check sets"),
            cell_entry(&format!("{:?}", args.check_set)),
        ]);
    writeln!(w, "{repo_info}\n")?;

    // Summary table
    writeln!(w, "Score summary\n")?;
    let mut score_summary = new_table();
    score_summary
        .load_preset(UTF8_FULL)
        .apply_modifier(UTF8_ROUND_CORNERS)
        .set_header(vec![cell_header("Section"), cell_header("Score")])
        .add_row(vec![cell_entry("Global"), cell_score(Some(score.global))])
        .add_row(vec![
            cell_entry("Code Vulnerabilities"),
            cell_score(score.code_vulnerabilities),
        ])
        .add_row(vec![
            cell_entry("Maintenance"),
            cell_score(score.maintenance),
        ])
        .add_row(vec![
            cell_entry("Continuous Testing"),
            cell_score(score.testing),
        ])
        .add_row(vec![
            cell_entry("Source Risk"),
            cell_score(score.source),
        ])
        .add_row(vec![
            cell_entry("Build Risk"),
            cell_score(score.build),
        ]);
    writeln!(w, "{score_summary}\n")?;

    // Checks table
    writeln!(w, "Checks summary\n")?;
    let mut checks_summary = new_table();
    checks_summary
        .load_preset(UTF8_FULL)
        .apply_modifier(UTF8_ROUND_CORNERS)
        .set_header(vec![cell_header("Check"), cell_header("Passed")])
        .add_row(vec![
            cell_entry("Code Vulnerabilities / Vulnerabilities"),
            cell_check(report.code_vulnerabilities.vulnerabilities.as_ref()),
        ])
        .add_row(vec![
            cell_entry("Maintenance / Dependency update tool"),
            cell_check(report.maintenance.dependency_update_tool.as_ref()),
        ])
        .add_row(vec![
            cell_entry("Maintenance / Maintained"),
            cell_check(report.maintenance.maintained.as_ref()),
        ])
        .add_row(vec![
            cell_entry("Maintenance / Security policy"),
            cell_check(report.maintenance.security_policy_sc.as_ref()),
        ])
        .add_row(vec![
            cell_entry("Maintenance / License"),
            cell_check(report.maintenance.license_sc.as_ref()),
        ])
        .add_row(vec![
            cell_entry("Maintenance / CII Best Practices"),
            cell_check(report.maintenance.cii_best_practices.as_ref()),
        ])
        .add_row(vec![
            cell_entry("Continuous Testing / CI tests"),
            cell_check(report.testing.ci_tests.as_ref()),
        ])
        .add_row(vec![
            cell_entry("Continuous Testing / Fuzzing"),
            cell_check(report.testing.fuzzing.as_ref()),
        ])
        .add_row(vec![
            cell_entry("Continuous Testing / SAST"),
            cell_check(report.testing.sast.as_ref()),
        ])
        .add_row(vec![
            cell_entry("Source Risk / Binary artifacts"),
            cell_check(report.source.binary_artifacts.as_ref()),
        ])
        .add_row(vec![
            cell_entry("Source Risk / Branch protection"),
            cell_check(report.source.branch_protection.as_ref()),
        ])
        .add_row(vec![
            cell_entry("Source Risk / Dangerous workflow"),
            cell_check(report.source.dangerous_workflow.as_ref()),
        ])
        .add_row(vec![
            cell_entry("Source Risk / Code review"),
            cell_check(report.source.code_review.as_ref()),
        ])
        .add_row(vec![
            cell_entry("Source Risk / Contributors"),
            cell_check(report.source.contributors_sc.as_ref()),
        ])
        .add_row(vec![
            cell_entry("Build Risk / Pinned dependencies"),
            cell_check(report.build.pinned_dependencies.as_ref()),
        ])
        .add_row(vec![
            cell_entry("Build Risk / Token permissions"),
            cell_check(report.build.token_permissions.as_ref()),
        ])
        .add_row(vec![
            cell_entry("Build Risk / Packaging"),
            cell_check(report.build.packaging.as_ref()),
        ])
        .add_row(vec![
            cell_entry("Build Risk / Signed releases"),
            cell_check(report.build.signed_releases.as_ref()),
        ]);
    writeln!(w, "{checks_summary}\n")?;

    // Check if the linter succeeded according to the provided pass score
    if score.global() >= args.pass_score {
        writeln!(
            w,
            "{SUCCESS_SYMBOL} Succeeded with a global score of {}\n",
            score.global().round()
        )?;
    } else {
        writeln!(
            w,
            "{FAILURE_SYMBOL} Failed with a global score of {} (pass score is {})\n",
            score.global().round(),
            args.pass_score
        )?;
    }

    Ok(())
}

/// Helper function to create a new table that will be forced to use a non-tty
/// mode when running tests.
#[allow(clippy::let_and_return, unused_mut)]
fn new_table() -> Table {
    let mut table = Table::new();

    #[cfg(test)]
    table.force_no_tty();

    table
}

/// Build a cell used for headers text.
fn cell_header(title: &str) -> Cell {
    Cell::new(title)
        .set_alignment(CellAlignment::Center)
        .add_attribute(Attribute::Bold)
}

/// Build a cell used for regular entries text.
fn cell_entry(title: &str) -> Cell {
    Cell::new(title).set_alignment(CellAlignment::Left)
}

/// Build a cell used for scores.
fn cell_score(score: Option<f64>) -> Cell {
    let (content, color) = match score {
        #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
        Some(v) => match v as usize {
            75..=100 => (v.round().to_string(), Color::Green),
            50..=74 => (v.round().to_string(), Color::Yellow),
            25..=49 => (v.round().to_string(), Color::DarkYellow),
            0..=24 => (v.round().to_string(), Color::Red),
            _ => ("?".to_string(), Color::Grey),
        },
        None => (NOT_APPLICABLE_MSG.to_string(), Color::Grey),
    };
    Cell::new(content)
        .set_alignment(CellAlignment::Center)
        .add_attribute(Attribute::Bold)
        .fg(color)
}

/// Build a cell used for checks output.
fn cell_check<T>(output: Option<&CheckOutput<T>>) -> Cell {
    let (content, color) = match output {
        Some(r) => match (r.passed, r.exempt, r.failed) {
            (true, _, _) => (SUCCESS_SYMBOL.to_string(), Color::Green),
            (false, true, _) => (EXEMPT_MSG.to_string(), Color::Grey),
            (false, _, false) => (FAILURE_SYMBOL.to_string(), Color::Red),
            (false, _, true) => (WARNING_SYMBOL.to_string(), Color::Yellow),
        },
        None => (NOT_APPLICABLE_MSG.to_string(), Color::Grey),
    };
    Cell::new(content)
        .set_alignment(CellAlignment::Center)
        .add_attribute(Attribute::Bold)
        .fg(color)
}

#[cfg(test)]
mod tests {
    use std::{path::PathBuf, str, str::FromStr};

    use clomonitor_core::{
        linter::{
            BuildRisk, CheckOutput, CheckSet, CodeVulnerabilities, ContinuousTesting, Maintenance,
            Report, SourceRisk,
        },
        score::Score,
    };

    use crate::{Args, Format};

    use super::display;

    #[test]
    fn display_prints_results() {
        // Setup test linter results
        let report = Report {
            code_vulnerabilities: CodeVulnerabilities {
                vulnerabilities: Some(CheckOutput::passed()),
            },
            maintenance: Maintenance {
                dependency_update_tool: Some(CheckOutput::passed()),
                maintained: Some(CheckOutput::passed()),
                security_policy_sc: Some(CheckOutput::passed()),
                license_sc: Some(CheckOutput::passed()),
                cii_best_practices: Some(CheckOutput::passed()),
            },
            testing: ContinuousTesting {
                ci_tests: Some(CheckOutput::passed()),
                fuzzing: Some(CheckOutput::passed()),
                sast: Some(CheckOutput::passed()),
            },
            source: SourceRisk {
                binary_artifacts: Some(CheckOutput::passed()),
                branch_protection: Some(CheckOutput::passed()),
                dangerous_workflow: Some(CheckOutput::passed()),
                code_review: Some(CheckOutput::passed()),
                contributors_sc: Some(CheckOutput::passed()),
            },
            build: BuildRisk {
                pinned_dependencies: Some(CheckOutput::passed()),
                token_permissions: Some(CheckOutput::passed()),
                packaging: Some(CheckOutput::passed()),
                signed_releases: Some(CheckOutput::passed()),
            },
        };
        let score = Score {
            global: 99.999_999_999_999_99,
            global_weight: 18,
            code_vulnerabilities: Some(100.0),
            code_vulnerabilities_weight: Some(1),
            maintenance: Some(100.0),
            maintenance_weight: Some(5),
            testing: Some(100.0),
            testing_weight: Some(3),
            source: Some(100.0),
            source_weight: Some(5),
            build: Some(100.0),
            build_weight: Some(4),
        };
        let args = Args {
            path: PathBuf::from_str("test-repo-path").unwrap(),
            url: "https://github.com/test-org/test-repo".to_string(),
            check_set: vec![CheckSet::Code],
            pass_score: 80.0,
            format: Format::Table,
        };

        // Display linter results using a vector as output
        let mut w = Vec::new();
        display(&report, &score, &args, &mut w).unwrap();

        let golden_path = "src/testdata/display.golden";

        // Write output to golden file (uncomment line below to update golden)
        // fs::write(golden_path, &w).unwrap();

        // Check output matches golden file content
        let output = str::from_utf8(w.as_slice()).unwrap();
        let golden = std::fs::read_to_string(golden_path).unwrap();
        assert_eq!(output, golden);
    }
}
