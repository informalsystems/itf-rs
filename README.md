
[![Build Status][build-image]][build-link]
[![Apache 2.0 Licensed][license-image]][license-link]
![Rust Stable][rustc-image]
![Rust 1.65+][rustc-version]

# itf-rs

Rust library for consuming [Apalache ITF Traces][itf-adr].

> ⚠️  This library is currently under heavy development.

## Example

**Trace:** [`MissionariesAndCannibals.itf.json`](./apalache-itf-derive/tests/fixtures/MissionariesAndCannibals.itf.json)

```rust
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, DecodeItfValue)]
enum Bank {
    N,
    W,
    E,
    S,
}

#[derive(Clone, Debug, TryFromRawState)]
#[allow(dead_code)]
struct State {
    #[itf(rename = "bank_of_boat")]
    pub boat_is_on_bank: Bank,
    pub who_is_on_bank: HashMap<Bank, HashSet<String>>,
}

let data = include_str!("../tests/fixtures/MissionariesAndCannibals.itf.json");
let raw_trace: raw::Trace = serde_json::from_str(data).unwrap();
let trace = parse_raw_trace::<State>(raw_trace).unwrap();

dbg!(trace);
```

**Output:**

```
trace = Trace {
    meta: TraceMeta {
        description: None,
        source: Some(
            "MC_MissionariesAndCannibalsTyped.tla",
        ),
        var_types: {
            "bank_of_boat": "Str",
            "who_is_on_bank": "Str -> Set(PERSON)",
        },
        format: None,
        format_description: None,
        other: {},
    },
    params: [],
    vars: [
        "bank_of_boat",
        "who_is_on_bank",
    ],
    loop: None,
    states: [
        State {
            boat_is_on_bank: East,
            who_is_on_bank: {
                West: {},
                East: {
                    Cannibal1,
                    Cannibal2,
                    Missionary2,
                    Missionary1,
                },
            },
        },
        State {
            boat_is_on_bank: West,
            who_is_on_bank: {
                East: {
                    Missionary1,
                    Cannibal1,
                },
                West: {
                    Missionary2,
                    Cannibal2,
                },
            },
        },
        State {
            boat_is_on_bank: East,
            who_is_on_bank: {
                East: {
                    Cannibal1,
                    Missionary1,
                    Missionary2,
                },
                West: {
                    Cannibal2,
                },
            },
        },
        State {
            boat_is_on_bank: West,
            who_is_on_bank: {
                West: {
                    Cannibal2,
                    Missionary2,
                    Missionary1,
                },
                East: {
                    Cannibal1,
                },
            },
        },
        State {
            boat_is_on_bank: East,
            who_is_on_bank: {
                East: {
                    Cannibal1,
                    Cannibal2,
                },
                West: {
                    Missionary1,
                    Missionary2,
                },
            },
        },
        State {
            boat_is_on_bank: West,
            who_is_on_bank: {
                West: {
                    Missionary2,
                    Cannibal1,
                    Cannibal2,
                    Missionary1,
                },
                East: {},
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

Copyright © 2022 Informal Systems Inc. and itf-rs authors.

Licensed under the Apache License, Version 2.0 (the "License"); you may not use the files in this repository except in compliance with the License. You may obtain a copy of the License at

    https://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software distributed under the License is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied. See the License for the specific language governing permissions and limitations under the License.

[apalache]: http://apalache.informal.systems
[itf-adr]: https://apalache.informal.systems/docs/adr/015adr-trace.html

[build-image]: https://github.com/informalsystems/itf-rs/workflows/Rust/badge.svg
[build-link]: https://github.com/informalsystems/itf-rs/actions?query=workflow%3ARust
[license-image]: https://img.shields.io/badge/license-Apache_2.0-blue.svg
[license-link]: https://github.com/informalsystems/itf-rs/blob/master/LICENSE
[rustc-image]: https://img.shields.io/badge/rustc-stable-blue.svg
[rustc-version]: https://img.shields.io/badge/rustc-1.65+-blue.svg

