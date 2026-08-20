#!/usr/bin/env bash
set -euo pipefail

repository_root="$(git rev-parse --show-toplevel)"
cd "$repository_root"

sensitive_paths="$(
  git ls-files --cached --others --exclude-standard |
    rg -i '(^|/)(\.env($|\.)|\.secrets(/|$)|keys?/.*\.json$|wallets?(/|$)|.*keypair.*\.json$|id_(rsa|ed25519)($|\.)|.*\.(pem|p12|key)$|terraform\.tfstate|terraform\.tfvars$|credentials?($|\.)|op\.txt$)' || true
)"
if [[ -n "$sensitive_paths" ]]; then
  printf 'Refusing public release: sensitive tracked path(s) found:\n%s\n' "$sensitive_paths" >&2
  exit 1
fi

secret_patterns=(
  '-----BEGIN ([A-Z ]+ )?PRIVATE KEY-----'
  'AKIA[0-9A-Z]{16}'
  'github_pat_[A-Za-z0-9_]{20,}'
  'gh[pousr]_[A-Za-z0-9]{30,}'
  'sk-[A-Za-z0-9_-]{24,}'
  'xox[baprs]-[A-Za-z0-9-]{20,}'
  '(API_KEY|ACCESS_TOKEN|AUTH_TOKEN|PRIVATE_KEY|SECRET_KEY|PASSWORD)[[:space:]]*[:=][[:space:]]*"?[A-Za-z0-9_+/=-]{16,}'
  '^\s*\[(\s*[0-9]{1,3}\s*,){31,}\s*[0-9]{1,3}\s*\]\s*$'
)

secret_files=""
for pattern in "${secret_patterns[@]}"; do
  matches="$(
    rg -Il --hidden --no-messages \
      -g '!.git/**' \
      -g '!**/node_modules/**' \
      -g '!**/target/**' \
      -g '!scripts/check-public-repo.sh' \
      -- "$pattern" . || true
  )"
  if [[ -n "$matches" ]]; then
    secret_files="${secret_files}${matches}"$'\n'
  fi
done
if [[ -n "${secret_files//$'\n'/}" ]]; then
  printf 'Refusing public release: possible secret material found in:\n' >&2
  printf '%s' "$secret_files" | sort -u >&2
  exit 1
fi

oversized_files=""
while IFS= read -r path; do
  [[ -f "$path" ]] || continue
  byte_count="$(wc -c < "$path" | tr -d ' ')"
  if (( byte_count > 5242880 )); then
    oversized_files="${oversized_files}${path} (${byte_count} bytes)"$'\n'
  fi
done < <(git ls-files --cached --others --exclude-standard)
if [[ -n "$oversized_files" ]]; then
  printf 'Refusing public release: tracked file(s) exceed 5 MiB:\n%s' "$oversized_files" >&2
  exit 1
fi

printf 'Public repository audit passed.\n'
