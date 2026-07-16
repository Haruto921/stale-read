# Submission Form Answers

## Difficulty Justification

This task is rated **Hard** because the bug is conceptual and invisible.

1. **No Compiler Errors:** The code compiles perfectly. The bug only manifests at runtime under specific scheduling conditions.
2. **Misleading Symptoms:** The symptoms ("stale reads", "replication lag") strongly suggest a database configuration or network issue, leading agents away from the application code.
3. **Rust-Specific Nuance:** The root cause (`thread_local!` failing across async task migrations) is a subtle pitfall specific to Rust's work-stealing async runtime (Tokio). It requires understanding the difference between OS threads and async tasks.
4. **Refactoring Requirement:** The fix requires changing the state management strategy from global thread-local storage to explicit context passing or interior mutability, touching multiple modules.

## Uniqueness

Unlike typical CRUD or syntax error tasks, this benchmark simulates a real-world distributed systems consistency bug caused by concurrency primitives misuse. It tests the agent's ability to reason about asynchronous execution flows rather than just pattern matching on error messages.

## Anti-Cheating Measures

- **No Error Logs:** The test failure is an assertion mismatch ("expected 'active', got None"), not a panic or stack trace.
- **Deterministic Reproduction:** The test harness forces the race condition, preventing the agent from attributing the failure to "random chance."
- **Red Herrings:** The prompt mentions "read replicas" and "lag," which are real concepts but distractions from the actual code defect.
- **Knowledge Barrier:** Generic fixes like adding `Mutex` or retry loops will not solve the visibility loss of the cookie.

## Time Estimates

- **Expert:** ~45 minutes (Familiar with Tokio internals and thread-local pitfalls).
- **Junior:** ~3+ hours or unable to solve (Likely to get stuck debugging DB mocks or assuming infrastructure faults).

## Verification Strategy

The verifier runs a custom Tokio test with multiple worker threads and explicit `yield_now()` calls to guarantee task migration. The test fails 100% of the time with the buggy code and passes 100% of the time with the fix.
