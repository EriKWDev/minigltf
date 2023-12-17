# minigltf
A stupid-simple, best-effort compliant glTF parser with minimal amount of dependencies.
`minigltf` currently only supports glTF 2.0, but is structured in a way that other versions could be implemented.

Has been tested on various glTFs from https://github.com/KhronosGroup/glTF-Sample-Models/tree/master/2.0, though not all of them.

If you find any models in the `glTF-Sample-Models` repo that cannot be parsed by `minigltf`, please file an issue or make a PR fixing it!

#### goals:
  - best-effort spec compliance
  - best-effort extension support
  - minimal amount of dependencies
  - fast compilation
  - fast parsing
  - simple API
    - no getters and setters just for the sake of it
    - O(1) operations should not look or act like O(n) operations (looking at you `gltf` and your `.nth()`!)

#### non-goals:
  - spec compliance
  - support of all extensions

# License
This code is released under the MIT license.
See the `LICENSE` file for more details.
