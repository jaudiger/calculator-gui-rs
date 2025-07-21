resource "github_repository" "bevy_calculator" {
  name        = "bevy-calculator"
  description = "Bevy calculator (Wasm)"

  visibility = "public"

  has_downloads = false
  has_issues    = false
  has_projects  = false
  has_wiki      = false

  delete_branch_on_merge = true

  pages {
    build_type = "workflow"
  }
}

resource "github_repository_ruleset" "branch_protection" {
  name        = "main-branch-protection"
  repository  = github_repository.bevy_calculator.name
  target      = "branch"
  enforcement = "active"

  conditions {
    ref_name {
      include = ["refs/heads/main"]
      exclude = []
    }
  }

  rules {
    deletion                = false
    required_linear_history = true
    non_fast_forward        = true
  }
}
