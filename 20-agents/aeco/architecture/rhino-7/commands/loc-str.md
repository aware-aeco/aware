# loc-str

Lifecycle: single

Strings that need to be localized should call this function. The STR function doesn't actually              do anything but return the original string. The LocalizationProcessor application walks              through the source code of a project and looks for LOC.STR. The function is then replaced with a              call to Localization.LocalizeString using a unique context ID.
