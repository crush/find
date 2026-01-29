```
┌──────────────────────────────────────────────────────────────┐
│   / find                                                     │
│   instant directory jumper with frecency                     │
└──────────────────────────────────────────────────────────────┘
```

```bash
> install?

  curl -fsSL raw.githubusercontent.com/crush/find/main/i | sh

> usage?

  f add ~/code      # add root
  f index           # index projects
  f noro            # jump
  f code rust       # multi-term search

> features?

  ✓ frecency ranking (frequency + recency)
  ✓ multi-term search (f fo ba → /foo/bar)
  ✓ auto-prune dead paths
  ✓ learns from usage

> keys?

  j/k or arrows     # navigate
  tab               # toggle path
  enter             # select
  esc/ctrl+c        # cancel

> frecency?

  score starts at 1, +1 per visit

  multiplier based on recency:
    last hour  → score × 4
    last day   → score × 2
    last week  → score ÷ 2
    older      → score ÷ 4

> stack?

  rust · crossterm · jwalk
```
