# ultrakill-save-editor
A save editor for ULTRAKILL written in Rust.

### Warning: This tool is still very much in alpha, please backup your saves before using it as it may corrupt your saves.

# FAQ
### Why won't it detect my save?
If your game is not installed inside your Steam installation directory (default: `C:\Program Files (x86)\Steam\`) the save editor will be unable to detect it.

To manually load your save file, locate your game files (if you have moved your Steam primary library folder, it should be inside `PATHTOLIBRARYFOLDER\steamapps\Common\ULTRAKILL\`), then navigate into the `Saves` directory. Then select the Slot you wish to edit (`Slot1` is the default slot) and open it, then copy the path from the address bar, and paste it into the `Save path:` field inside the save editor. From there simply hit `Load`.

### Why won't it save any General data?
This was an issue in v0.1.0 and v0.1.1 due to a bug in the code. It should now be fixed in v0.1.2.

### Why won't it save any data?
This app does not feature auto-saving as it can very easily cause invalid data to be saved. You have to hit `Save` once you have made all your changes. Additionally, this app will not automatically create files. If the `Delete file` button under the options you wish to change is grayed out, make sure to hit the `Create file` button to create the file if it does not exist. If this still doesn't work report the issue [here](https://github.com/PyPylia/ultrakill-save-editor/issues/new).

### How do I suggest a feature or report an issue?
Feel free to submit any feature requests or report issues in the Issue tab [here](https://github.com/PyPylia/ultrakill-save-editor/issues). Make sure to double check if your issue/feature already has been reported/requested.
