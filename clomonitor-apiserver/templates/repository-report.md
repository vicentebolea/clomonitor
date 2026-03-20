## CLOMonitor report

### Summary

**Repository**: {{ name }}
**URL**: {{ url }}

{%- if let (Some(report), Some(score)) = (report.as_ref(), score.as_ref()) %}
**Checks sets**:  {% for check_set in check_sets %}`{{ check_set }}`{% if !loop.last %} + {% endif %}{% endfor %}
**Score**: {{ score.global.round() }}

### Checks passed per category

| Category |                              Score |
| :------- | ----------------------------------: |
| Project  |  {{ category_score(score.project) }} |
| Source   |   {{ category_score(score.source) }} |
| Build    |    {{ category_score(score.build) }} |

## Checks

{% if let Some(value) = score.project -%}
### Project [{{ value.round() }}%]

  {{ check("maintained-from-openssf-scorecard", "Maintained", report.project.maintained) -}}

{%- endif %}
{%- if let Some(value) = score.source %}
### Source [{{ value.round() }}%]

  {{ check("code-review-from-openssf-scorecard", "Code review", report.source.code_review) -}}
  {{ check("dangerous-workflow-from-openssf-scorecard", "Dangerous workflow", report.source.dangerous_workflow) -}}
  {{ check("token-permissions-from-openssf-scorecard", "Token permissions", report.source.token_permissions) -}}

{%- endif %}
{%- if let Some(value) = score.build %}
### Build [{{ value.round() }}%]

  {{ check("binary-artifacts-from-openssf-scorecard", "Binary artifacts", report.build.binary_artifacts) -}}
  {{ check("dependency-update-tool-from-openssf-scorecard", "Dependency update tool", report.build.dependency_update_tool) -}}
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
