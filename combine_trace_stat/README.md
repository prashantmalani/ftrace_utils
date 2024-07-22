### Overview

A script that takes the function profile output for various CPUs (from `ftrace`) and consolidates the stats, while also sorting the output by average time.

The Linux kernel `ftrace` infrastructure provides tools to aid in system debug and profiling. One of these is the function profiler.
This measures the time spent by every function in the kernel, and presents the output in per-CPU files. More details [here](https://lwn.net/Articles/370423/).
While per-CPU data is useful, sometimes we would like to aggregate the data from across CPUs (especially since the CPU scheduler may transition tasks
to various CPUs in a non-deterministic manner, when tasks aren't pinned to specific CPUs).

Configs required to use the profiler:
- `CONFIG_FUNCTION_TRACER`
- `CONFIG_DYNAMIC_FTRACE`
- `CONFIG_FUNCTION_GRAPH_TRACER`

### Typical usage of the script

#### Collect traces of workload
- Set the ftrace filter for the functions/modules you are interested in observing: 
  `echo nvme* > /sys/kernel/debug/tracing/set_ftrace_filter`
- Set the tracer to `nop`:
  `echo nop > /sys/kernel/debug/tracing/current_tracer`
- Enable function profiling:
  `echo  1 > /sys/kernel/debug/tracing/function_profile_enabled`
- Start tracing:
  `echo 1 > /sys/kernel/debug/tracing/tracing_on`
- Run your workload:
- Stop tracing:
  `echo 0 > /sys/kernel/debug/tracing/tracing_on`

#### Running script
- Copy the system traces to the host machine:
  `scp -r root@$TARGET:/sys/kernel/debug/tracing/trace_stat /tmp`
- Run the command:
  `cargo run /tmp/trace_stat/*`

Pull requests welcomed!
