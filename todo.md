# Optimizations

### CPU side
- Do not parse glyphs in text buffer every frame. Only parse inserted text via the insert buffer.
- Redo buffer representation from `Vec<String>` -> `GapBuffer<GapBuffer>` or something

### GPU side
- Use banding to reduce frag shader load
- When reallocating SSBO buffer for curve data, copy existing curve data GPU-side to prevent large memory transfer from CPU to GPU

### General
- Cull text CPU side to reduce quads drawn

*Optimizations are not too necessary as it *is* just a text editor, albiet with 2.5D rendering. Mostly just need to parse the text efficiently for both glyphs and syntax, and then cull drawing data.

# Features

### Config
- Allow configurations:
    - Non mono support
    - Colors
    - Font
    - Cursor properties
- Should create a parser (toml?) to load config file
- Global config state to read from?

### Finish VIM support
- Still unsure whether to continue embedding key behavior or to switch to scripting (lua)

### File viewer
- Read-only buffer that acts as file explorer
- May need a crate to moniter fs changes

### Status bar + messages
- Contain file name, lines, cr/lf, modified T/F 
- Messages for errors such as file open fail

### VIM command mode 
- Custom commands that adjust configuration and eye candy

### Syntax highlighting
- Via syntect (probably won't ever use Tree-sitter / LSP due to complexity)

### Colors 
- Load a 256x1024 texture into VRAM
- Each line corresponds to a gradient. Colors are just solid gradients (?)
- Quad data uses 1 byte to specify color (may be able to squeeze into current vars)
- Colors vary by uTime

### Searching via Regex
- title

### Better cursor visuals
- Currently only has 2 control points. Having more would be nice.
- Also improve cursor color + blending

# Code base improvements
- Separate crates, one for renderer, one for editor/buffer, etc
- Add tests + fuzzing
- Consider either dropping freetype crate or font-kit and implementing the tiny amount of code from each crate natively
