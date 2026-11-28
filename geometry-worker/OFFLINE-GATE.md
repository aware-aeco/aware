# Geometry M1 offline provisioning gate

Run this manual gate on the VM-capable Windows x64 runner labelled geometry-offline for delivery and every wheel-lock change. Dispatch with the candidate commit SHA and its passing Geometry M1 Windows build run ID. Use the passing run from the base repository for fork PRs too.

1. Verify that the Windows run tested the requested SHA and that **Geometry M1 gate** passed. Check out the SHA. Download geometry-m1-normal and geometry-m1-evidence with the passing attempt number appended to their names. Use the normal artifact identity, wheel-lock digest and Windows x64 ABI from the downloaded report to verify the stage before booting the guest. Verify the stage with the host command below.

       python geometry-worker/offline-gate.py precheck --commit $commit --run-id $runId --normal $normal --windows-report $windowsReport

2. While online, populate the wheelhouse with both pinned wheels. Use the candidate checkout's lock and verify the hashes with the following command.

       python -m pip download --only-binary=:all: --no-deps --require-hashes -r geometry-worker/win-amd64-cp312.requirements.lock -d <wheelhouse>

3. Transfer the normal stage, Windows report, candidate checkout, signed CPython 3.12 x64 installer and populated wheelhouse on an offline volume. Start a fresh Windows x64 guest snapshot with all network adapters detached at the hypervisor. Install CPython from that volume at a short path such as C:\p312. Use a guest with an appropriate system clock and a Windows code-signing trust store that validates the signed CPython binaries. Run the guest command below.

       C:\p312\python.exe geometry-worker/offline-gate.py guest --commit $commit --run-id $runId --normal $normal --windows-report $windowsReport --wheels $wheelhouse --python C:\p312\python.exe --report C:\geometry-offline.json

4. Shut down the VM and export the guest report. Validate it from the host using the same normal artifact. Upload it as a maintenance-run artifact and attach the passing report to the PR or delivery record. Discard the VM snapshot after extracting the evidence.

       python geometry-worker/offline-gate.py validate --commit $commit --run-id $runId --normal $normal --windows-report $windowsReport --report $guestReport

Configure the repository ruleset to require the stable **Geometry M1 gate** check on protected branches. The check is defined by the workflow. The ruleset enforces it. [GitHub required check behavior](https://docs.github.com/en/pull-requests/how-tos/merge-and-close-pull-requests/troubleshooting-required-status-checks)
