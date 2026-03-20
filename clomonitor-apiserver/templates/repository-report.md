## CLOMonitor report

### Summary

**Repository**: {{ name }}
**URL**: {{ url }}

{%- if let (Some(report), Some(score)) = (report.as_ref(), score.as_ref()) %}
**Checks sets**:  {% for check_set in check_sets %}`{{ check_set }}`{% if !loop.last %} + {% endif %}{% endfor %}
**Score**: {{ score.global.round() }}

### Checks passed per category

| Category             |                                       Score |
| :------------------- | ------------------------------------------: |
| Code Vulnerabilities | {{ category_score(score.code_vulnerabilities) }} |
| Maintenance          |        {{ category_score(score.maintenance) }} |
| Continuous Testing   |           {{ category_score(score.testing) }} |
| Source Risk          |             {{ category_score(score.source) }} |
| Build Risk           |              {{ category_score(score.build) }} |

## Checks

{% if let Some(value) = score.code_vulnerabilities -%}
### Code Vulnerabilities [{{ value.round() }}%]

  {{ check("vulnerabilities-from-openssf-scorecard", "Vulnerabilities", report.code_vulnerabilities.vulnerabilities) -}}

{%- endif %}
{%- if let Some(value) = score.maintenance %}
### Maintenance [{{ value.round() }}%]

  {{ check("dependency-update-tool-from-openssf-scorecard", "Dependency update tool", report.maintenance.dependency_update_tool) -}}
  {{ check("maintained-from-openssf-scorecard", "Maintained", report.maintenance.maintained) -}}
  {{ check("security-policy-from-openssf-scorecard", "Security policy", report.maintenance.security_policy_sc) -}}
  {{ check("license-from-openssf-scorecard", "License", report.maintenance.license_sc) -}}
  {{ check("cii-best-practices-from-openssf-scorecard", "CII Best Practices", report.maintenance.cii_best_practices) -}}

{%- endif %}
{%- if let Some(value) = score.testing %}
### Continuous Testing [{{ value.round() }}%]

  {{ check("ci-tests-from-openssf-scorecard", "CI tests", report.testing.ci_tests) -}}
  {{ check("fuzzing-from-openssf-scorecard", "Fuzzing", report.testing.fuzzing) -}}
  {{ check("sast-from-openssf-scorecard", "SAST", report.testing.sast) -}}

{%- endif %}
{%- if let Some(value) = score.source %}
### Source Risk [{{ value.round() }}%]

  {{ check("binary-artifacts-from-openssf-scorecard", "Binary artifacts", report.source.binary_artifacts) -}}
  {{ check("branch-protection-from-openssf-scorecard", "Branch protection", report.source.branch_protection) -}}
  {{ check("dangerous-workflow-from-openssf-scorecard", "Dangerous workflow", report.source.dangerous_workflow) -}}
  {{ check("code-review-from-openssf-scorecard", "Code review", report.source.code_review) -}}
  {{ check("contributors-from-openssf-scorecard", "Contributors", report.source.contributors_sc) -}}

{%- endif %}
{%- if let Some(value) = score.build %}
### Build Risk [{{ value.round() }}%]

  {{ check("pinned-dependencies-from-openssf-scorecard", "Pinned dependencies", report.build.pinned_dependencies) -}}
  {{ check("token-permissions-from-openssf-scorecard", "Token permissions", report.build.token_permissions) -}}
  {{ check("packaging-from-openssf-scorecard", "Packaging", report.build.packaging) -}}
  {{ check("signed-releases-from-openssf-scorecard", "Signed releases", report.build.signed_releases) -}}

{%- endif %}
For more information about each check, see the [OpenSSF Scorecard documentation](https://scorecard.dev/).

{%- else %}

This repository hasn't been processed yet, please try again later.
{%- endif -%}

{% macro check(doc_id, display_name, option) %}
  {%- if let Some(check_output) = option -%}
    - [{% if check_output.passed || check_output.exempt %}x{% else %} {% endif %}]
    {%- if let Some(link) = check_output.url %} [{{ display_name }}]({{ link }}) {% else %} {{ display_name }} {% endif -%}
    ([_docs_](https://scorecard.dev/))
    {%- if check_output.exempt %} `EXEMPT`{%- endif %}
    {%- if check_output.failed %} `CHECK FAILED`{%- endif %}
  {% endif -%}
{%- endmacro %}

{% macro category_score(option) %}
  {%- if let Some(value) = option -%}{{ value.round() }}%{%- else -%}n/a{%- endif -%}
{% endmacro %}
