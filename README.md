# Calculator GUI (WebAssembly)

## Getting Started

This is a calculator built using [Rust](https://www.rust-lang.org) language with the [Bevy](https://bevy.org) game engine library. It's simple enough to demonstrate the usage of Bevy with WebAssembly (aka Wasm).

To run it locally in your Web browser, you can use [trunk](https://trunkrs.dev), and go to [http://localhost:8080](http://localhost:8080):

```bash
trunk serve
```

Wasm is not the only target of this application, the GUI can also be run as a desktop application with:

```bash
cargo run
```

### Alternative

The Wasm file is served through Cloudflare Pages. To see it in action, open your Web browser and navigate to [https://calculator-gui-rs.jaudiger.dev/](https://calculator-gui-rs.jaudiger.dev/).

> **⚠️ Warning:** Serving the Wasm file can take a couple of second, even if the binary size is under 20MB.
