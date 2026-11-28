# Geometry M1 offline provisioning gate

Use this gate for delivery and every wheel-lock change. The **Geometry offline** workflow runs on GitHub-hosted Windows and validates a report exported from a separate Windows x64 VM. The VM host must boot a disposable guest from a clean snapshot with its virtual network adapters detached, provision from the offline volume, exercise the installed CLI and export the report. The hosted workflow cannot establish those VM conditions or attest who generated the supplied JSON. GitHub does not officially support nested virtualization on hosted runners. [GitHub-hosted runners](https://docs.github.com/en/actions/concepts/runners/github-hosted-runners)

1. In the base repository, find the completed **Geometry M1** run whose **Geometry M1 gate** passed, including for a fork PR. Use the candidate SHA in its report. For a pull request this can be the tested merge SHA. Check out that SHA and download `geometry-m1-normal-<attempt>` and `geometry-m1-evidence-<attempt>`. Use the attempt that ran the successful **windows-evidence** job for both downloads. Use the normal artifact identity, wheel-lock digest and Windows x64 ABI from the downloaded report to verify the stage before booting the guest.

       python geometry-worker/offline-gate.py precheck --commit $commit --run-id $runId --run-attempt $attempt --normal $normal --windows-report $windowsReport

2. While online, populate the wheelhouse with both pinned wheels. Use the candidate checkout's lock and verify the hashes with the following command.

       python -m pip download --only-binary=:all: --no-deps --require-hashes -r geometry-worker/win-amd64-cp312.requirements.lock -d <wheelhouse>

3. Transfer the normal stage, Windows report, candidate checkout (including Git metadata), wheelhouse, and signed full Windows x64 installers for CPython 3.12.10 and a different CPython 3.12 patch release. Boot a fresh Windows x64 guest snapshot with all virtual network adapters detached at the hypervisor. Use a guest with Git, an appropriate system clock and a code-signing trust store that validates the signed CPython binaries. Install the two builds from the volume at short paths such as `C:\p312` and `C:\p312alt` with pip enabled and PATH updates disabled. From the candidate checkout run the guest command below. It checks for zero active adapters and zero default routes. It provisions and cuts with a disposable copy of the first base, then moves that base aside and puts a copy of the other patch build at the same path to check the two fallback cases.

       C:\p312\python.exe geometry-worker/offline-gate.py guest --commit $commit --run-id $runId --run-attempt $attempt --normal $normal --windows-report $windowsReport --wheels $wheelhouse --python C:\p312\python.exe --replacement-python C:\p312alt\python.exe --report C:\geometry-offline.json

4. Shut down the guest, export `geometry-offline.json` and discard the snapshot. From the base repository, dispatch the hosted validator with the guest file contents.

       @{ candidate_sha = $commit; windows_run_id = "$runId"; guest_report = (Get-Content -Raw -LiteralPath $guestReport) } |
           ConvertTo-Json -Compress | gh workflow run geometry-offline.yml --json

5. Wait for **Geometry offline report validation** to pass and attach its uploaded report to the PR or delivery record. The job checks the source workflow and passing attempt, revalidates the Windows evidence and downloaded normal stage, then checks that the guest report matches the same candidate, lock, ABI, run, attempt, artifact and four expected outcomes.

Configure the repository ruleset to require the stable **Geometry M1 gate** check on protected branches. [GitHub required check behavior](https://docs.github.com/en/pull-requests/how-tos/merge-and-close-pull-requests/troubleshooting-required-status-checks)
