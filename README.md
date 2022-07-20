### Approaches
 - maintain the JSON serialised nature to perform the search using serde
 - necessary due to current limitations of wasm bindgen
  - currently can't pass around JS Objects in wasm ([see here](https://rustwasm.github.io/wasm-bindgen/contributing/design/js-objects-in-rust.html))
 - goal it to utilise [this approach](https://rustwasm.github.io/wasm-bindgen/reference/arbitrary-data-with-serde.html) to handling data, however generalise the case **even more** in order to allow fully arbitrary data structures to be past
  - i.e. nested objects, objects of different structures
  - essentially not passing the serde object into some predefined struct
  - current best solution idea to this is to convert the dynamic object into a dynamic Hashmap - will be difficult!