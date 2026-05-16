# loc-con

Lifecycle: single

Command option name strings that need to be localized should call this function. The CON function              doesn't actually do anything but return the original string. The LocalizationProcessor application walks              through the source code of a project and looks for LOC.CON. The function is then replaced with a              call to Localization.LocalizeCommandOptionName using a unique context ID.
