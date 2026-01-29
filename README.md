```
┌──────────────────────────────────────────────────────────────┐
│   / find                                                     │
│   instant directory jumper                                   │
└──────────────────────────────────────────────────────────────┘
```

```bash
> what is this?

  a fast directory jumper that indexes your projects
  no cd history needed - knows your folders instantly

> features?

  ✓ instant project detection (git, cargo, npm, etc)
  ✓ fuzzy search with smart ranking
  ✓ interactive selection when multiple matches
  ✓ parallel indexing with rayon
  ✓ zero warmup time

> usage?

  # add directories to index
  f add ~/code
  f add ~/projects

  # index your directories
  f index

  # jump to a project
  f noro          # jumps if unique match
  f proj          # shows picker if multiple matches

> install?

  curl -fsSL raw.githubusercontent.com/crush/find/main/i | sh

> update?

  # same command
  curl -fsSL raw.githubusercontent.com/crush/find/main/i | sh

> shell integration?

  # bash/zsh - add to ~/.bashrc or ~/.zshrc
  function j() {
    local dir
    dir=$(f "$@")
    if [ -n "$dir" ]; then
      cd "$dir"
    fi
  }

  # fish - add to ~/.config/fish/config.fish
  function j
    set dir (f $argv)
    if test -n "$dir"
      cd $dir
    end
  end

> stack?

  rust · clap · nucleo · inquire · jwalk · rayon

> visit?

  https://github.com/crush/find
```
