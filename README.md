# Bevy Calculator

## Instructions

This is a calculator built using the Bevy game engine. It's simple enough to demonstrate the usage of Bevy with WebAssembly (aka Wasm).

To run it easily in your Web browser, you can use [trunk](https://trunkrs.dev), and go to [http://localhost:8080](http://localhost:8080):

```bash
trunk serve
```

### Alternative

To see the calculator in action, open your Web browser and navigate to [https://jaudiger.github.io/bevy-calculator/](https://jaudiger.github.io/bevy-calculator/). The Wasm file is smaller than 10MB, but it can take a couple of seconds to be served by the GitHub Pages.

## CI / CD

The CI/CD pipeline is configured using GitHub Actions. The workflow is defined in the `.github/workflows` folder:

- Static Analysis
- Code Audit (on each Cargo dependencies update, or run each day through CronJob)
- Deployment

Additionally, Dependabot is configured to automatically update dependencies (GitHub Actions, Terraform providers, Cargo dependencies).

## Repository configuration

The settings of this repository are managed using Terraform. The configuration is located in the `.github/terraform` folder.
