# conc-map-bench

conc-map-bench uses the bustle benchmarking harness. This is a port of the well regarded libcuckoo benchmark.

## Workloads

The benchmark measures performance under varying load conditions. This is done
because a map suitable for one workload may not be suitable for another.

### Update Heavy 1

```
read   10%
insert  5%
remove  5%
update  80%
```

### Update Heavy 2

```
read    45%
insert  5%
remove  5%
update  45%
```

## Results

Machine: Apple M3 Max

OS: macOS 14.3

See the `results/` directory.

<!-- ### Read Heavy (std hasher)
| | |
:-------------------------:|:-------------------------:
![](results/ReadHeavy.std.throughput.svg) | ![](results/ReadHeavy.std.latency.svg)

### Exchange (std hasher)
| | |
:-------------------------:|:-------------------------:
![](results/Exchange.std.throughput.svg) | ![](results/Exchange.std.latency.svg)

### Rapid Grow (std hasher)
| | |
:-------------------------:|:-------------------------:
![](results/RapidGrow.std.throughput.svg) | ![](results/RapidGrow.std.latency.svg)

### Read Heavy (ahash)
| | |
:-------------------------:|:-------------------------:
![](results/ReadHeavy.ahash.throughput.svg) | ![](results/ReadHeavy.ahash.latency.svg)

### Exchange (ahash)
| | |
:-------------------------:|:-------------------------:
![](results/Exchange.ahash.throughput.svg) | ![](results/Exchange.ahash.latency.svg)

### Rapid Grow (ahash)
| | |
:-------------------------:|:-------------------------:
![](results/RapidGrow.ahash.throughput.svg) | ![](results/RapidGrow.ahash.latency.svg) -->

### Update Heavy 1 (std hasher)
| | |
:-------------------------:|:-------------------------:
![](results/UpdateHeavy1.std.throughput.svg) | ![](results/UpdateHeavy1.std.latency.svg)

### Update Heavy 2 (std hasher)
| | |
:-------------------------:|:-------------------------:
![](results/UpdateHeavy2.std.throughput.svg) | ![](results/UpdateHeavy2.std.latency.svg)
