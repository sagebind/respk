workflow "Main" {
  on = "push"
  resolves = ["Build"]
}

action "Build" {
  uses = "docker://rust"
  args = "cargo test"
}
