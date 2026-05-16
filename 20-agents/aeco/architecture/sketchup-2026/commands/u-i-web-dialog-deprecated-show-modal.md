# u-i-web-dialog-deprecated-show-modal

Lifecycle: single

The show_modal method is used to display a modal dialog box. In SketchUp 6 and 7, this behaves differently on Mac vs. PC. On the PC, it shows a truly modal dialog, meaning so long as the WebDialog is visible, no input can be performed elsewhere inside SketchUp. On the Mac, “modal” WebDialogs do not behave this way, but instead are “always on top” of other windows.
