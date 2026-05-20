// Tests in this assembly mutate a shared process-environment variable
// (AWARE_STUB_DUMP_DIR) to coordinate with the stub-rhinocode.cmd. xUnit's
// default parallel execution races on that var. Force serial execution.
using Xunit;

[assembly: CollectionBehavior(DisableTestParallelization = true)]
