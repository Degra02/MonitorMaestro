# MonitorMaestro
Tired of fiddling with Hyprland configs?  

Monitor Maestro lets you switch between monitor layouts in a snap!  

This Rust-powered TUI app lets you define layouts in a simple JSON file and switch between them with a few keystrokes.  

Effortless Workflow:
- Define layouts in JSON.
- Simple TUI for easy selection.
- Switch layouts instantly.

The TUI runs on a terminal, so it's up to you to add a `bind` in you Hyprland configs  
If you want an example, take a look at my Hyprlad [dotfiles](https://github.com/Degra02/dotfiles/blob/master/hypr/hyprland.conf) and search for `monitor_maestro`

## Functionality
The program has 3 execution options:  
- tui: start the TUI
- workspace \<NAME\>: no tui, only run the commands to start the specified workspace
- state: get the current workspace from the file `/tmp/monitor_maestro_state.txt`, auto-handled

## Keybinds
`j - k`: go down - up the list  
`q - ESC`: exit  
`Enter`: select monitor layout


## Installation
Clone the repo, then build with `cargo` 
```bash
cargo build --release
```
and copy the executable `monitor_maestro` wherever you choose.

## Configuration
An example workspaces configuration json [file](./workspaces_example.json) is provided  
A monitor can either be __Enabled__, and requires the fields:
- dimensions
- refresh rate
- position
- scaling

Or __Disabled__


## Milestones
- [ ] Dynamic TUI showing rectangles as current monitor layout, in a pseudo realistic way  
- [ ] TUI for interactive monitor layout creation and json output
