A scratchpad for now, thinking about the general design.

One open question: Should osxnav change it's name and also support linux in the
future? (Could replace keynav.) Whatever the answer is, I'll try to keep the
config file design cross platform.

There should be a config file similar to keynav. Left side is key code/name,
right side is command. You can have multiple commands for a keypress. Commands
are run sequentially. 2 key presses will never run commands at the same time,
because they might interfere with each other. Commands will run for whichever
key was pressed first.

Here are some of the commands:
```
hold-{ctrl|shift|super|alt|left|middle|right} - hold a modifier/mouse key
drop-{ctrl|shift|super|alt|left|middle|right} - drop (release) a modifier/mouse key, does nothing if the key isn't currently being held
tap-{left|middle|right} - tap (press then release) a mouse key
scroll-{left|down|up|right} - scroll to a direction

cut-{left|down|up|right} - cuts the grid in half to the direction
move-{left|down|up|right} - moves the grid by one grid length in the direction
move-half-{left|down|up|right} - moves the grid by half a grid length in the direction

reset - resets the grid to take up the whole screen
warp - moves the mouse to the center of the grid
exit - leave osxnav

record - all osxnav keypresses are now being recorded in a macro, if recording is in progress, it is now reset
stop - stop the macro recording, if there is no recording in progress, do nothing
play - play the macro recording, if there is a recording in progress, repeat what is in the recording so far

undo - move the grid to the position it was previously, does nothing if there is nothing to undo
redo - if you called undo, go up on the grid position stack, does nothing if there is nothing to redo
```

Here is my keynav config, translated into this config format:

```
c: reset

e:       exit
esc:     exit
shift+e: exit

h: cut-left
j: cut-down
k: cut-up
l: cut-right

w: warp

shift+h: move-left
shift+j: move-down
shift+k: move-up
shift+l: move-right

super+h: cut-left
super+j: cut-down
super+k: cut-up
super+l: cut-right

shift+q: record
s: stop
q: play

u: undo
shift+u: redo

shift+p: warp,hold-shift,scroll-up,drop-shift
shift+n: warp,hold-shift,scroll-down,drop-shift

p: warp,scroll-up
n: warp,scroll-down

space: warp,click-left
m:     warp,click-middle
r:     warp,click-right

super+space: warp,tap-left
super+m:     warp,tap-middle
super+r:     warp,tap-right

shift+space: warp,hold-left
shift+m:     warp,hold-middle
shift+r:     warp,hold-right

ctrl+space: warp,hold-ctrl,tap-left,drop-ctrl
ctrl+m:     warp,hold-ctrl,tap-middle,drop-ctrl
ctrl+r:     warp,hold-ctrl,tap-right,drop-ctrl
```

That doesn't have to be the syntax though. We could go with something a bit more complex, like supporting function calls:

```
shift+p: scroll-vertical(-10%)
shift+n: scroll-vertical(10%)
x: exec(echo hello)
```

That will probably make things too complicated. So I don't think that's the right route to go.

There could be some global configurations too. Like the color of the grid. Either that would go in a separate config file, or you can just pass things like that as arguments, since osxnav works by constantly opening and exiting anyways. Ex:

```
osxnav --grid-color red
osxnav --grid-color '#ff0000'
```

Hooks/scripts might be useful too. Here is one that could get called when your
command is one that doesn't exist. This is an executable file called
click-speak.

```
#!/bin/bash
# filename is: ~/.config/osxnav/hooks/click-speak
speak "click"
echo "tap-left"
```

And you'd invoke the command normally, ex:

```
super+space: click-speak
```

The command would speak then click.

Maybe hooks/scripts aren't needed either. It's just an idea.
