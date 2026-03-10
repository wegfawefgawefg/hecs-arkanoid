# Juice Pass Plan

This repo does not need heavy shaders or a giant post stack to feel good. The first meaningful pass should focus on timing, motion, impacts, and readability.

## First Pass

1. `Hitstop`
   - short freezes on paddle hits, block hits, laser hits, powerup catches, and life loss
   - stronger on destroys than on simple bounces
2. `Camera shake`
   - tiny on normal impacts
   - stronger on brick destroy, multiball, level clear, and life loss
3. `Camera pulse / zoom`
   - subtle zoom pulse on stronger impacts
   - not a permanent moving camera, just impact emphasis
4. `Impact particles`
   - small sparks on bounce
   - larger burst on brick destruction
   - pickup pop on capsule catch
5. `Squash / stretch / recoil`
   - paddle compresses slightly on impact
   - ball gets a short pulse on impact
6. `Impact flash`
   - tiny screen flash on strong impacts and transitions
7. `Sound variation`
   - slight pitch / volume variation on repeated bounce sounds
8. `Transition punch`
   - level intro / complete / loss text should pulse more instead of just appearing statically

## What To Avoid For Now

- bloom
- CRT shaders
- big postprocessing chains
- complicated audio compressors / sidechains
- full camera follow logic

The game can get most of its feel from simple layered feedback first.

## Arkanoid-Specific Notes

- Strong blocks should feel heavier than normal blocks.
- Fast balls should read louder and sharper.
- Paddle modes should have more visual identity:
  - `Catch` should feel magnetic / sticky
  - `Lasers` should feel charged and punchy
  - `BombBall` should feel hotter / more destructive
- Powerup drops should bob and pop a little.
- Near-clear cleanup should feel intentional, not like a silent auto-skip.

## Initial Tuning Targets

- bounce hitstop: `1` frame
- destroy hitstop: `2-3` frames
- powerup catch hitstop: `2` frames
- life loss hitstop: `4-6` frames
- normal shake: `0.5-1.0 px`
- destroy shake: `1.5-2.5 px`
- big event zoom pulse: `1-3%`

## Follow-Ups

- ball trails at high speed
- floating score popups
- better transition typography / motion
- stronger paddle-mode rendering
- richer particles per event kind

## Next Layer

The next strong juice layer for this repo is `contextual block death`.

- normal ball break:
  - little shard pop
  - stronger for full destroy than for partial damage
- fireball break:
  - red / orange ball look
  - smoke trail while active
  - ember burst and melt / burn effect when a block dies
- laser break:
  - vertical streaks and hot white-red carve effect
  - should read like the block was cut, not just bumped
- strong block hit:
  - heavier gray crack / shatter effect without the block dying

This should stay lightweight and data-poor. The repo does not need a full animation system yet; a few particle flavors and per-cause effect helpers are enough.

## Juice Still Left On The Table

- stronger block death variants:
  - split into pieces and fall with gravity
  - shatter
  - melt
  - burn
  - shrink out
- better laser travel trail
- fireball heat distortion / aura
- score-chain / combo callouts
- last-brick tension treatment
- stronger level transition polish
- better audio layering per powerup mode
