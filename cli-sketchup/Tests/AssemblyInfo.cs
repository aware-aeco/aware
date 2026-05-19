// xUnit serializes tests in the same collection. Disable collection-level
// parallelism so tests that touch the shared %TEMP%\aware-sketchup
// discovery dir (or its overrides via env var) don't race each other.
[assembly: Xunit.CollectionBehavior(DisableTestParallelization = true)]
