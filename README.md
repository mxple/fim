# Fim
Text editing has never been more fun. [F]ancy V[im] is a semi-ironic graphical text editor for the postmodern era. Fim combines playful colors and eyecandy with the familiar navigation of Vi/m. While not meant to be used seriously Fim (unless you really want to), Fim aims to (eventually) support advanced text editing features such as syntax highlighting and regex search. While other text editors aim to booost your productivity and workflow, Fim prioritizes something more important --- **dopamine**.

Note, Fim is currently in pre-alpha and is more of a tech demo than anything. It can't even write to the file system yet (so don't be afraid to test it).

## Features
An expanding list of cool features:
- Resolution independent GPU text rendering using bezier curves (credit for [algorithm](https://wdobbie.com/post/gpu-text-rendering-with-vector-textures/) and [shader code](https://github.com/GreenLightning/gpu-font-rendering?tab=MIT-1-ov-file#readme))
- Built-in Vi/m navigation (WIP)
- Lazy font loading using system fonts
- Semi-support for Unicode characters (support for individual characters, not grapheme clusters)
- Semi-support for some IME's (tested CJK + Thai)
- Neovide-like cursor trail

## Building
Fim uses a patched version of `freetype-rs` which can be found [here](https://github.com/mxple/freetype-rs/tree/master). You will need to clone this, and the patched freetype library, then modify `Cargo.toml` accordingly to build. Aside from the patch, just run:
```
$ git clone https://github.com/mxple/fim
$ cd fim
$ cargo r --release
```
If you get errors regarding SDL2, refer to [rust-sdl2 docs](https://github.com/Rust-SDL2/rust-sdl2).

If errors persist, please open an issue. I want Fim to work cross platform and be easy to use :)

*Note for Mac users: Fim uses OpenGL 4.3 which is not natively supported on Mac. However, you may "emulate" gl 4.6 using Mesa + MoltenVK to translate gl -> Vulkan -> Metal calls. More detailed instructions to come after I get to testing on Mac hardware.

## Contributing
If you find bugs, first check to see if the bug has already been noted in todo.md. Otherwise, please open an issue describing the bug.

Pull requests are appreciated! Especailly those that check off the todolist.

## Screenshots
![2024-06-18-212814_hyprshot](https://github.com/mxple/fim/assets/83033020/eba72f1b-fdc3-48b6-b4b6-418619908db8)
![2024-06-17-165616_hyprshot](https://github.com/mxple/fim/assets/83033020/6792ede3-40d8-4d82-bae2-f4c1263b545d)
https://github.com/mxple/fim/assets/83033020/57495d11-1915-4375-ab59-2c1d6f85fc30
