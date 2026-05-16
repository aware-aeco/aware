---
name: yard-u-i-notification
description: UI::Notification API reference (YARD)
---

# UI::Notification API reference

Notification objects allows you to show native notifications in the desktop. Notifications can have a message, icon and accept and/or dismiss buttons with callback blocks.

## Methods

- `initialize` — In order to insert line breaks into the message you need to use \r\n. From SketchUp 2019 and onwards \n also works on both Mac and Windows.
- `icon_name` — Gets the icon name, this is the path that will be used to get the icon from the file system path.
- `icon_name=` — Sets the icon path, this icon will be loaded from the path give, the path has to be a local filesystem path.
- `icon_tooltip` — Gets the icon Tooltip, this is the string that appear when the mouse is over the icon.
- `icon_tooltip=` — Sets the icon Tooltip, this string will appear when the mouse is over the icon.
- `message` — Gets the message as string.
- `message=` — In order to insert line breaks into the message you need to use \r\n. From SketchUp 2019 and onwards \n also works on both Mac and Windows.
- `on_accept` — Shows a button in the notification with the given title and callback block, both arguments are required.
- `on_accept_title` — Returns the accept's button title.
- `on_dismiss` — Shows a button in the notification with the given title and callback block. Both arguments are required. This callback is only called if you press the Dismiss button, not when the time runs out and the notification automatically disappears.
- `on_dismiss_title` — Returns the dismiss's button title.
- `show` — Shows the notification. If not interacted with, the notification will disappear without calling any callbacks.
