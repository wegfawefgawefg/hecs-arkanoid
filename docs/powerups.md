# Powerups And Run Rules

This repo started as a small HECS Arkanoid experiment, but the natural next step is a real Arkanoid-style powerup loop instead of only clean bouncing and level progression.

## Core Loop

- Breakable blocks have a chance to drop a powerup capsule.
- Capsules drift downward.
- Catching a capsule with the paddle activates the effect immediately.
- Score goes up for:
  - damaging or destroying blocks
  - catching powerups
  - clearing a level
- Score can go down when a ball is lost.
- Lives are shown in the HUD and the run only ends when all lives are gone.

## First Pass Powerups

These came from the original `todo.txt`, with a few Arkanoid-standard clarifications:

1. `Enlarge`
  - widens the paddle
2. `Shrink`
  - narrows the paddle
3. `SpeedUp`
  - increases ball speed
4. `SlowDown`
  - decreases ball speed
5. `BallSplit`
  - splits every live ball into more balls
6. `Catch`
  - ball sticks to the paddle until fired
7. `ExtraLife`
  - adds one life
8. `Lasers`
  - paddle can fire upward shots
9. `BombBall`
  - fireball mode; breakable blocks do not bounce the ball back

## Symbol Language

Capsules should not be plain colors only. Each one should have a line-drawn symbol inside the box.

Suggested first-pass symbols:

- `Enlarge`: left-right arrows
- `Shrink`: inward arrows
- `SpeedUp`: `>>`
- `SlowDown`: `<<`
- `BallSplit`: two or three small circles
- `Catch`: a cup / tray shape
- `ExtraLife`: plus sign
- `Lasers`: twin vertical beams
- `BombBall`: a circle with a small fuse / spark

The same symbol language should be reused in the HUD for active effects.

## HUD

The low-res HUD should show:

- score
- lives
- current level
- active powerups / paddle mode

Lives can be small paddle or ball icons. Active effects should be listed vertically with a symbol and short label.

## Rules

- Powerup drops should feel helpful more often than harmful.
- `Shrink` and `SpeedUp` are fine as tension spikes, but they should be less common than the helpful drops.
- Powerups can persist across levels for now.
- If only a few breakable blocks remain for too long, the level should auto-advance rather than soft-locking the run in cleanup.
- Losing a life should clear active modifiers and return the run to a clean default state, except score / remaining lives.

## Follow-Ups

- indestructible / strong / powerup-carrying block variants
- better score table and bonus scoring
- particles and stronger feedback when catching a capsule
