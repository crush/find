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
  f noro            # jump (fuzzy)
  f code rust       # multi-term
  f top             # top directories

> bookmarks?

  f mark work       # mark current dir
  f mark docs ~/d   # mark specific path
  f work            # jump to bookmark
  f marks           # list bookmarks
  f unmark work     # remove bookmark

> parent jump?

  fb                # jump to nearest .git parent
  fb src            # jump to parent containing "src"

> keys?

  j/k               # navigate
  tab               # toggle scores
  enter             # select
  esc               # cancel

> features?

  ✓ nucleo fuzzy matching (6x faster)
  ✓ frecency ranking
  ✓ bookmarks
  ✓ parent directory jump
  ✓ bincode serialization
  ✓ mimalloc allocator
  ✓ shell completions

> completions?

  f completions bash >> ~/.bashrc
  f completions zsh >> ~/.zshrc
  f completions fish > ~/.config/fish/completions/f.fish

> stack?

  rust · nucleo · crossterm · ignore
```
