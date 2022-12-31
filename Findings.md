# Prototype findings
My personal experience working with bevy, this was a small glimpse into making a small prototype, more experienced devs may feel differently.

## Bevy strengths
Working with ECS and rust is nice, the performance is really good and no worries about memory leaks.

My favorite thing honestly is the cargo packaging and build system, although web targets won't work with many third party packages.

Building plugins for bevy is pretty straightforward but the community packages are still young, many basic things like skyboxes will need to be made from scratch and you may find a third party package that is a few PRs away from what you need it to do.

## Bevy limitations

Bevy is not a huge engine like Godot or Unity yet so some features will need to be added for more high level development.

Packages Used:
* Rand for rng
* bevy_rapier (which is overkill) for collisions
* bevy_hanabi for high level particle fx    
    * limited to quads for the time being

Plugins made
* Skybox - a modification from the [bevy examples](https://github.com/bevyengine/bevy/blob/main/examples/3d/skybox.rs)