# {{project_name}}

{{description}}

{%- if on_github -%}

[![Linux build status](https://travis-ci.org/{{gh_username}}/{{project_name}}.svg?branch=master)](https://travis-ci.org/{{gh_username}}/{{project_name}})
[![Windows build status](https://ci.appveyor.com/api/projects/status/github/{{gh_username}}/{{project_name}}?svg=true)](https://ci.appveyor.com/project/{{gh_username}}/{{project_name}})
{%- endif %}

{%- if license %}

Licensed under {{license}}.
{%- endif %}

{%- if changelog %}

## Changelog

Please see the [CHANGELOG](CHANGELOG.md) for a release history.
{%- endif -%}

{%- if publish_binaries %}

## Installation
Binaries are automatically uploaded to GitHub for each version.
{%- endif -%}
