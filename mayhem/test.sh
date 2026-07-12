#!/usr/bin/env bash
#
# feather/mayhem/test.sh — RUN feather-rs/feather's own test suite (`cargo test --workspace`,
# the same suite upstream CI runs in .github/workflows/main.yml) and emit a CTRF summary.
# exit 0 iff no test failed.
#
# PATCH-grade oracle: feather ships a real assertion suite across the workspace —
#   - libcraft crates: block/item/inventory known-answer tests (exact IDs, state round-trips);
#   - feather/base: anvil region/level save-format round-trip + known-answer decode tests;
#   - feather/protocol: packet codec round-trip tests asserting exact encoded/decoded values
#     (the same code path the `codec` fuzz target drives);
#   - feather/common, feather/ecs, feather/datapacks, feather/worldgen, quill crates: unit
#     tests asserting concrete values (ECS queries, biome sampling, plugin ABI round-trips).
# These assert concrete values / round-trip equality, so a no-op / "exit(0)" patch CANNOT pass.
# This script only RUNS the suite; build.sh pre-compiled it with `cargo test --workspace --no-run`.
#
# Skipped upstream tests (exactly ONE): the quill-common doctest
# `component::bincode_component_impl` — upstream documents a compiler ERROR MESSAGE inside an
# untagged ``` fence (quill/common/src/component.rs), which rustdoc compiles as Rust code and
# which can never compile. It is a broken-by-construction doc example (not a behavioral test);
# fixing it would require modifying an upstream file (breaks the purely-additive overlay), so
# it is excluded via `--skip bincode_component_impl` and counted as skipped below.
set -uo pipefail
[ -n "${SOURCE_DATE_EPOCH:-}" ] || unset SOURCE_DATE_EPOCH

: "${MAYHEM_JOBS:=$(nproc)}"
cd "$SRC"

# emit_ctrf <tool> <passed> <failed> [skipped] [pending] [other]
emit_ctrf() {
  local tool="$1" passed="$2" failed="$3" skipped="${4:-0}" pending="${5:-0}" other="${6:-0}"
  local tests=$(( passed + failed + skipped + pending + other ))
  cat > "${CTRF_REPORT:-$SRC/ctrf-report.json}" <<JSON
{
  "results": {
    "tool": { "name": "$tool" },
    "summary": {
      "tests": $tests,
      "passed": $passed,
      "failed": $failed,
      "pending": $pending,
      "skipped": $skipped,
      "other": $other
    }
  }
}
JSON
  printf 'CTRF {"results":{"tool":{"name":"%s"},"summary":{"tests":%d,"passed":%d,"failed":%d,"pending":%d,"skipped":%d,"other":%d}}}\n' \
    "$tool" "$tests" "$passed" "$failed" "$pending" "$skipped" "$other"
  [ "$failed" -eq 0 ]
}

if ! command -v cargo >/dev/null 2>&1; then
  echo "cargo not available — cannot run the test suite" >&2
  emit_ctrf "cargo-test" 0 1 0; exit 2
fi

echo "=== running cargo test --workspace (feather unit + integration suite) ==="
# Use the image's DEFAULT toolchain (the Dockerfile pins it to the same nightly the fuzz build
# uses) — no `+toolchain` override. --no-fail-fast so we count every test; RUSTFLAGS cleared so
# it inherits nothing from the sanitizer build (same fingerprint as build.sh's --no-run
# pre-build, so nothing recompiles here).
out="$(RUSTFLAGS="" cargo test --workspace --no-fail-fast --jobs "$MAYHEM_JOBS" -- --skip bincode_component_impl 2>&1)"; rc=$?
echo "$out"

# libtest prints one line per test binary:
#   test result: ok. 12 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; ...
# Sum across all binaries.
# Count ignored + filtered-out (the one --skip'd broken doctest) as skipped.
PASSED=0; FAILED=0; IGNORED=0
while read -r p f i flt; do
  PASSED=$(( PASSED + p )); FAILED=$(( FAILED + f )); IGNORED=$(( IGNORED + i + flt ))
done < <(printf '%s\n' "$out" \
  | sed -n 's/^test result:.* \([0-9][0-9]*\) passed; \([0-9][0-9]*\) failed; \([0-9][0-9]*\) ignored; [0-9]* measured; \([0-9][0-9]*\) filtered out.*/\1 \2 \3 \4/p')

# If we parsed no result lines, fall back to the cargo exit code (e.g. compile error).
if [ "$(( PASSED + FAILED + IGNORED ))" -eq 0 ]; then
  echo "could not parse any 'test result:' lines; using cargo exit code $rc" >&2
  [ "$rc" -eq 0 ] && { emit_ctrf "cargo-test" 1 0 0; exit 0; }
  emit_ctrf "cargo-test" 0 1 0; exit 1
fi

emit_ctrf "cargo-test" "$PASSED" "$FAILED" "$IGNORED"
