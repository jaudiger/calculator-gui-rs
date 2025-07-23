# Bevy Calculator

## Instructions

This is a calculator built using the [Bevy](https://bevy.org) game engine. It's simple enough to demonstrate the usage of Bevy with WebAssembly (aka Wasm).

To run it locally in your Web browser, you can use [trunk](https://trunkrs.dev), and go to [http://localhost:8080](http://localhost:8080):

```bash
trunk serve
```

### Alternative

The Wasm file is served through Cloudflare Pages. To see the calculator in action, open your Web browser and navigate to [https://bevy-calculator.jaudiger.dev/](https://bevy-calculator.jaudiger.dev/).

## CI / CD

The CI/CD pipeline is configured using GitHub Actions. The workflow is defined in the [`.github/workflows`](.github/workflows) folder:

- Static Analysis (source code, GitHub Actions)
- Code Audit (on each Cargo dependencies update, or run each day through CronJob)
- Deployment

Additionally, Dependabot is configured to automatically update dependencies (GitHub Actions, Terraform providers, Cargo dependencies).

## Repository configuration

The settings of this repository are managed from the [gitops-deployments](https://github.com/jaudiger/gitops-deployments) repository using Terraform. The actual configuration applied is located in the Terraform module [`modules/github-repository`](https://github.com/jaudiger/gitops-deployments/tree/main/modules/github-repository).
