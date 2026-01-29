```
┌──────────────────────────────────────────────────────────────┐
│   / find                                                     │
│   instant directory jumper                                   │
└──────────────────────────────────────────────────────────────┘
```

```bash
> install?

  curl -fsSL raw.githubusercontent.com/crush/find/main/i | sh

> usage?

  f add ~/code      # add root directory
  f index           # index projects
  f noro            # jump to noro

> how it works?

  f <query>         # exact match = instant jump
                    # multiple matches = minimal picker

> ui?

  > noro            # clean folder names
    api             # no ugly paths
    web             # just names

> stack?

  rust · nucleo · inquire · jwalk
```
