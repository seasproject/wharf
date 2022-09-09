# Wharf
A reversable scripting language and JellyFish package extension.

## As a JellyFish package
Wharf packages are exactly the same as JellyFish packages, except they contain 'ropes'. They have a 'build.rope' file, which is executed *before* package install (added to path, etc). They can also contain other '.rope' files, which 'build.rope' may depend on.

## As a scripting language
Wharf scripting is centered around *ropes*, which can be deloyed, and removed from the system at will. This is because it is fully* reversable, so any action is just undone.

## Getting Started
Binaries are available on github, in the jellyfish package format.
