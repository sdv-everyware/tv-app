# tv-app: the guide for coding agents

A home screen in the style of a smart television's launcher. It is one EveryWare app: a harry app on the EveryWare UI library, its own git repository
in the tree at `packages/services/display_safety/utils/tv-app`. The project's guide (the
repositories, every skill, the rules for every change) is
`vendor/sdv-everyware/everyware/AGENTS.md`: read it first.

## What is where

| Path | What |
|---|---|
| `design/tv-patch.js` | the screen: Library components in slots under `#tv-root`, named `#tv/...` |
| `design/weather-pictures.js` | the app's own pictures of the weather, the set `#tv-weather` |
| `design/build.sh` | builds the design: `design/build/tv.fig` and `data/tv.dcf` |
| `data/tv.dcf` | the design the app loads; committed |
| `src/ui_model.rs` | the names of the design the code writes to or taps |
| `src/state.rs, actions.rs, presenter.rs` | the state, what a gesture or the data changes, the state to the screen |
| `src/house_link.rs` | the house's data from the dashboard bundle (gRPC, 127.0.0.1:7100; one app at a time holds the port) |
| `src/weather.rs` | what the weather looks like here: the bundle's code to a picture and to words |

## The rules of an app

- **Library components only on the screen.** A new kind of control is a new library component
  (the `ui-component` skill): stop and ask the user, do not draw it in the patch.
- **What the app shows that is its own is the app's:** its pictures are a component set of its
  design, its type in the code implements `Component` with `ComponentKind::App` and `Slotted`.
  Nothing of an app goes into the UI library, also not under a generic name.
- **A bundle sends information; how it looks is decided here** (the presenter and the modules it
  uses), not in a bundle and not in the library.
- **The design's names are a contract** with `src/ui_model.rs`: change both in one commit.
- **Nothing hidden in the design** (a hidden layer never reaches the device): hide with
  `slot_shown`, `grid_shown` or an opacity.
- **Rules that need no screen** (a game, a calculation) are a module of their own with host
  tests.

## Commands

```bash
# the design: the Library, this app's .fig (design/build/, not in git) and data/tv.dcf
packages/services/display_safety/utils/tv-app/design/build.sh
# the app, for the RB3 (from the tree's top)
bash -c 'source device/rb3/sdv_media_db845c/tools/env.sh >/dev/null 2>&1; m tv_app'
```

Running it on the RB3, a screenshot and a tap for a test: the `new-app` skill
(`vendor/sdv-everyware/ui-lib/skills/new-app/SKILL.md`), steps 7 and 8. After a design build give
the user the full path of the built `.fig` (the script's last line).

A change lands through its own branch, a review by another agent and the user's word
(`vendor/sdv-everyware/everyware/AGENTS.md`, "How a change lands"; `scripts/feature.sh` there).
The hooks in `scripts/git-hooks/` refuse a commit on `main`, by another author, with something
private in it or with an AI attribution (`vendor/sdv-everyware/everyware/scripts/setup-agents.sh`
installs them).
