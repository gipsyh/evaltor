# Evaltor

A tool for evaluating the performance of evaluatees on benchmarks.

### Requirements
- Docker
- Rust Compiler

### Install
- build the `evaltor_box` with Docker, and all the evaluatees will be placed inside this container to be evaluated ```docker build -t evaltor_box:latest -f ./BoxDockerfile .```
- ```cargo install --path .```

### Example
- evaluate the rIC3 model checker under hwmcc19, hwmcc20, and hwmcc24
```evaltor ./examples/hwmc.toml ./examples/rIC3.toml  -b hwmcc19.aig,hwmcc20.aig,hwmcc24.aig -e default```

