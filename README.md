A collection of helper binaries that take the output from ftrace sysfs and modify/consolidate them for specific profiling use cases.

Each helper has its own Cargo package and directory (of course, you need a rust compiler to build these Cargo packages).

Summary:
- combine_trace_stat: Combines the per-CPU function profile data (which can be pulled from `sys/kernel/debug/tracing/trace_stat` and prints out the aggregate time across all CPUs.
