
[![Build Status][build-image]][build-link]
[![codecov][codecov-image]][codecov-link]
[![Apache 2.0 Licensed][license-image]][license-link]
![Rust Stable][rustc-image]
![Rust 1.65+][rustc-version]

# itf-rs

Rust library for consuming [Apalache ITF Traces][itf-adr].

## Example

**Trace:** [`MissionariesAndCannibals.itf.json`](./apalache-itf/tests/fixtures/MissionariesAndCannibals.itf.json)

```rust
use std::collections::{BTreeSet, BTreeMap};

use serde::Deserialize;

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Deserialize)]
enum Bank {
    #[serde(rename = "N")]
    North,

    #[serde(rename = "W")]
    West,

    #[serde(rename = "E")]
    East,

    #[serde(rename = "S")]
    South,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Deserialize)]
enum Person {
    #[serde(rename = "c1_OF_PERSON")]
    Cannibal1,

    #[serde(rename = "c2_OF_PERSON")]
    Cannibal2,

    #[serde(rename = "m1_OF_PERSON")]
    Missionary1,

    #[serde(rename = "m2_OF_PERSON")]
    Missionary2,
}

#[derive(Clone, Debug, Deserialize)]
struct State {
    pub bank_of_boat: Bank,
    pub who_is_on_bank: BTreeMap<Bank, BTreeSet<Person>>,
}

let data = include_str!("../tests/fixtures/MissionariesAndCannibals.itf.json");
let trace: Trace<State> = itf::trace_from_str(data).unwrap();

dbg!(trace);
```

**Output:**

```rust
[itf/examples/cannibals.rs:45] trace = Trace {
    meta: Meta {
        format: None,
        format_description: None,
        source: Some(
            "MC_MissionariesAndCannibalsTyped.tla",
        ),
        description: None,
        var_types: {
            "bank_of_boat": "Str",
            "who_is_on_bank": "Str -> Set(PERSON)",
        },
        timestamp: None,
        other: {},
    },
    params: [],
    vars: [
        "bank_of_boat",
        "who_is_on_bank",
    ],
    loop_index: None,
    states: [
        State {
            meta: Meta {
                index: Some(
                    0,
                ),
                other: {},
            },
            value: State {
                bank_of_boat: East,
                who_is_on_bank: {
                    West: {},
                    East: {
                        Cannibal1,
                        Cannibal2,
                        Missionary1,
                        Missionary2,
                    },
                },
            },
        },
        State {
            meta: Meta {
                index: Some(
                    1,
                ),
                other: {},
            },
            value: State {
                bank_of_boat: West,
                who_is_on_bank: {
                    West: {
                        Cannibal2,
                        Missionary2,
                    },
                    East: {
                        Cannibal1,
                        Missionary1,
                    },
                },
            },
        },
        State {
            meta: Meta {
                index: Some(
                    2,
                ),
                other: {},
            },
            value: State {
                bank_of_boat: East,
                who_is_on_bank: {
                    West: {
                        Cannibal2,
                    },
                    East: {
                        Cannibal1,
                        Missionary1,
                        Missionary2,
                    },
                },
            },
        },
        State {
            meta: Meta {
                index: Some(
                    3,
                ),
                other: {},
            },
            value: State {
                bank_of_boat: West,
                who_is_on_bank: {
                    West: {
                        Cannibal2,
                        Missionary1,
                        Missionary2,
                    },
                    East: {
                        Cannibal1,
                    },
                },
            },
        },
        State {
            meta: Meta {
                index: Some(
                    4,
                ),
                other: {},
            },
            value: State {
                bank_of_boat: East,
                who_is_on_bank: {
                    West: {
                        Missionary1,
                        Missionary2,
                    },
                    East: {
                        Cannibal1,
                        Cannibal2,
                    },
                },
            },
        },
        State {
            meta: Meta {
                index: Some(
                    5,
                ),
                other: {},
            },
            value: State {
                bank_of_boat: West,
                who_is_on_bank: {
                    West: {
                        Cannibal1,
                        Cannibal2,
                        Missionary1,
                        Missionary2,
                    },
                    East: {},
                },
            },
        },
    ],
}
```

## Versioning

We follow [Semantic Versioning](https://semver.org), though APIs are still under active development.

## Resources

- [Apalache Website][apalache]
- [Apalache ADR-015: Informal Trace Format][itf-adr]

## License

Copyright © 2023 Informal Systems Inc. and itf-rs authors.

Licensed under the Apache License, Version 2.0 (the "License"); you may not use the files in this repository except in compliance with the License. You may obtain a copy of the License at

    https://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software distributed under the License is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied. See the License for the specific language governing permissions and limitations under the License.

[apalache]: http://apalache.informal.systems
[itf-adr]: https://apalache.informal.systems/docs/adr/015adr-trace.html

[build-image]: https://github.com/informalsystems/itf-rs/workflows/Rust/badge.svg
[build-link]: https://github.com/informalsystems/itf-rs/actions?query=workflow%3ARust
[codecov-image]: https://codecov.io/github/informalsystems/itf-rs/branch/main/graph/badge.svg?token=6LFLG9ILD1
[codecov-link]: https://codecov.io/github/informalsystems/itf-rs
[license-image]: https://img.shields.io/badge/license-Apache_2.0-blue.svg
[license-link]: https://github.com/informalsystems/itf-rs/blob/master/LICENSE
[rustc-image]: https://img.shields.io/badge/rustc-stable-blue.svg
[rustc-version]: https://img.shields.io/badge/rustc-1.65+-blue.svg

