# Regelungstechnik, Robotik

[Teoretische und matematische Anteil aus Wikipedia](https://de.wikipedia.org/wiki/Kontrolltheorie) und hier auf Englich [Control theory](https://en.wikipedia.org/wiki/Control_theory) and [control systems engineering](https://en.wikipedia.org/wiki/Control_engineering)

[Wikibook Englich `Control Systems`](https://en.wikibooks.org/wiki/Control_Systems)

`Control Systems is an inter-disciplinary engineering text that analyzes the effects and interactions of mathematical systems. This book is for third and fourth year undergraduates in an engineering program.`

[WiKi](https://de.wikipedia.org/wiki/Regler)
[Automat](https://de.wikipedia.org/wiki/Automat_(Informatik))

## 2D-3D Algebra und Kollisionserkennung

`ncollide` is a 2 and 3-dimensional collision detection library written with
the rust programming language.

The official user guide is available [here](http://ncollide.org).
The rustdoc documentation is available [for 3D](http://ncollide.org/rustdoc/ncollide3d) and [for 2D](http://ncollide.org/rustdoc/ncollide3d).

`nalgebra` is a linear algebra library written for Rust targeting:

- General-purpose linear algebra (still lacks a lot of features…)
- Real time computer graphics.
- Real time computer physics.

The official user guide is available [here](http://ncollide.org).

## Simulation

`nphysics` is a 2 and 3-dimensional physics engine for games and animations. It uses ncollide for collision detection, and `nalgebra` for vector/matrix math. 2D and 3D implementations both share the same code!

- Static and dynamic rigid bodies.
- Common convex primitives: cone, box, ball, cylinder.
- Concave geometries build from convex primitives (aka. compound geometries).
- Stable stacking.
- Island based sleeping (objects deactivation).
- Ray casting.
- Swept sphere based continuous collision detection.
- Ball-in-socket joint.
- FixedJoint joint.
- Sensors.

The official user guide is available [here](http://nphysics.org")

## Interessante Projekten

[Eurobot](eurobot.md)
[braun robotic](https://github.com/braun-robotics)