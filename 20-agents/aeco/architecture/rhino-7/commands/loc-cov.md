# loc-cov

Lifecycle: single

Command option name strings that need to be localized should call this function. The COV function              doesn't actually do anything but return the original string. The LocalizationProcessor application walks              through the source code of a project and looks for LOC.COV. The function is then replaced with a              call to Localization.LocalizeCommandOptionValue using a unique context ID.
