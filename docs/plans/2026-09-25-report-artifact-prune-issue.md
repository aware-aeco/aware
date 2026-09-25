## Gap

`aware app artifact` can copy one run-owned artifact and report its byte usage, but has no supported way to remove the artifacts of one exact completed or crash-interrupted report run. A recurring front door can retain a bounded report history only if it can reclaim the retired run's AWARE-owned source spool and renderer bundle. Deleting files directly from the front door bypasses AWARE's ownership and path checks.

## Reproduction

1. Run `aware app artifact --help` with the current CLI. It exposes `--usage`, `--output`, and `--max-bytes`, but no remove/prune operation.
2. Execute a report app with a trusted `AWARE_REPORT_RESERVATION_ID`. AWARE records the exact app/instance/run reservation under `logs/.report-reservations/` and stores source/render artifacts under `logs/<app>/<instance>/<run>.artifacts/`.
3. `aware app artifact <app> --instance <instance> --run-id <run> --usage` measures those files. No CLI command can retire that exact run's artifact directory when the report ages out, or recover its files after an interrupted writer.

## Expected

A scoped CLI operation should accept an exact app, instance, run ID and reservation ID, refuse linked or unexpected paths, prove that the actual artifact writer has stopped (including child/host writers), remove only that run's flat artifact files, keep the trace and reservation evidence, and return machine-readable, idempotent results. It must refuse an interrupted legacy run when writer absence cannot be proven. Concurrent readers and a live writer must be fenced. Callers can verify `--usage` is zero before releasing their own quota reservation.

## Observed

No operation exists. The only cleanup route is direct filesystem deletion by a downstream consumer, which cannot enforce AWARE's run ownership or writer lifecycle.

Related: streamed report artifacts and reservation ownership in #572.
