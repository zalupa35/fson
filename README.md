# fson

**FSON** _(Flexible Serialized Object Notation)_ is an extension for JSON that
is used primarily for configuration. The user can quickly configure the
configuration using [**references**](#features) or a
[**template strings**](#features).

# Features

## Comments and new values

```
/* Multiline comment */ [null, NaN, Infinity, -Infinity, 0x1ABC /*Hexadecimal*/] // Single comment
```

## Identifiers

- You can use identifiers without quotes and with single quotes:

```
{
    "double quotes": null,
    'single quotes': null,
    withoutQuotes: null
}
```

## References

- A **reference** is an object **that can be anywhere (it can be either a pair
  of an object or it can be in an array)** that can be referenced using it's
  identifier or path. For example:

```
{
  something: {
      key: #{ #id: "identifier"; #value: "value"; }
  }
}
```

- The **reference** in the example above can be referenced in two ways:
  - Using it's identifier: `#identifier` or `#"identifier"`
  - Using it's path: `#/something/identifier` or `#/"something"/"identifier"`

## Template strings

- **Template strings** are strings enclosed in backticks. They allow you to
  embed other values **(including [references](#references))** in them using
  `${value}`. For example:
  ```
  {
    x: 5,
    something: `x is ${x}`
  }
  ```

## Other

- Objects and arrays can have a trailing comma: `{ x: { y: [], }, }`
- Numbers can start with a plus: `+1.5`
- Strings can be multiline:

```
"hello
world"
```

- Whitespaces don't matter

# Examples

See all examples in [Examples](examples) directory.<br>How to run example:
`cargo run --example EXAMPLE_NAME`

# Compiling to WebAssembly

**FSON** is already ready for compilation to **WebAssembly** and already has the
necessary functions. `js-sys` and `wasm-bindgen` libraries and functions are
used only when compiling to **WebAssembly**.<br> Use these commands to compile
to wasm:

```
# Install wasm-pack
cargo install wasm-pack

# Compile to wasm
wasm-pack build --target web
```

<details>
<summary>JavaScript example</summary>

```js
import init, { parse, stringify } from "./jsonparser.js";
init().then(() => {
  console.log(stringify({
    // Creating reference
    x: {
      "#id": "test",
      "#value": "value",
    },

    // Using reference
    y: [
      // Identifier
      { "#reference_id": "test" },

      // Path
      { "#reference_path": ["x"] },

      // Template string
      {
        "@template_string": [
          "test is ",
          { "#reference_id": "test" }, /* Reference */
          "; 2 + 2 = ",
          4, /* Normal value */
        ],
      },
    ],
  }));
});
```

</details>

# License

MIT
