# Stale Read Fix - Expert Challenge

## Problem

A Rust microservice with read replica routing is returning stale data after write operations.

## Expected Behavior

After any write, subsequent reads should return the written data immediately.

## Current Behavior

Reads sometimes return stale data from replicas instead of fresh data from primary.

## Constraints

- The service uses Tokio async runtime
- Session state must persist across async operations
- Use standard Rust concurrency primitives only

## Acceptance Criteria

Run `cargo test` and ensure all tests pass consistently.
