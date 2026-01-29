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
  f top             # browse top dirs (scrollable)

> bookmarks?

  f mark work       # mark current dir
  f work            # jump to bookmark
  f marks           # list all
  f unmark work     # remove

> navigation?

  fb                # jump to .git parent
  fb src            # jump to parent matching "src"

> import?

  f import zoxide   # import from zoxide
  f import z        # import from z

> keys?

  j/k or scroll     # navigate
  tab               # toggle scores
  enter             # select
  esc               # cancel

> features?

  ✓ nucleo fuzzy (6x faster)
  ✓ frecency ranking
  ✓ bookmarks
  ✓ parent jump
  ✓ import from zoxide/z
  ✓ excludes current dir
  ✓ mouse scroll
  ✓ fish/bash/zsh

> stack?

  rust · nucleo · crossterm · ignore
```
