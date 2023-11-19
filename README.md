# Input Mapper
Util to trigger keys when other keys are pressed  
Useful if you're playing a game that doesn't provide alternate keybindings, for example

If you're expecting devices that aren't showing up, you might need to add yourself to the input group, or run this as sudo/root

## TODO
1) Allow selection of input device
    1) May need to return index along with device names, and pass the index back to select device
2) Allow creation of mappings
    1) Would be nice to get a list of all enums evdev uses, and some way of correlating enum names with underlying values
    2) Allowing selection by pressing buttons on the selected input device would be nice, but not super useful for the other side of the mapping
        1) i.e. mapping a keyboard key to a mouse button, how would you select the mouse button? We could set up listeners for ALL input devices, but there may be all sorts of random stuff going on that could interfere
3) Narrow mappings to specific windows?
    1) Get a list of all running window names and provide as a list in the frontend
4) Allow mapping of multiple devices
    1) should be relatively straight forward once the flow for one device is sorted
    2) allow starting and stopping of devices and their mappings
5) Saving
    1) Store as json somewhere?

