# Fetch Example (Rust)

This example demonstrates how to fetch content from a URL using a Wassette component written in Rust.

For more information on installing Wassette, please see the [installation instructions](https://github.com/microsoft/wassette?tab=readme-ov-file#installation).

## Usage

To use this component, load it from the OCI registry and provide a URL to fetch.

**Load the component:**

```
Please load the component from oci://ghcr.io/microsoft/fetch-rs:latest
```

**Fetch content:**

```
Please fetch the content of https://example.com
```

## Policy

By default, WebAssembly (Wasm) components do not have any access to the host machine or network. The `policy.yaml` file is used to explicitly define what network resources are made available to the component. This ensures that the component can only access the resources that are explicitly allowed.

Example:

```yaml
version: "1.0"
description: "Permission policy for fetch-rs example in wassette"
permissions:
  network:
    allow:
      - host: "https://rss.nytimes.com/"
```

The source code for this example can be found in [`src/lib.rs`](src/lib.rs).
