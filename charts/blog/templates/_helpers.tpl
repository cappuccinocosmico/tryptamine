{{-/*
Helper templates for the blog chart
*/-}}
{{- define "blog.name" -}}
{{- .Chart.Name | trunc 63 | trimSuffix "-" -}}
{{- end -}}

{{- define "blog.fullname" -}}
{{- printf "%s-%s" .Release.Name (include "blog.name" .) | trunc 63 | trimSuffix "-" -}}
{{- end -}}

{{- define "blog.labels" -}}
app.kubernetes.io/name: {{ include "blog.name" . }}
helm.sh/chart: {{ .Chart.Name }}-{{ .Chart.Version }}
app.kubernetes.io/instance: {{ .Release.Name }}
app.kubernetes.io/managed-by: {{ .Release.Service }}
{{- end -}}
