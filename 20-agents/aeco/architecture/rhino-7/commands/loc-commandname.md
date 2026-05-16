# loc-commandname

Lifecycle: single

Command names that need to be localized should call this function. The COMMANDNAME function doesn't actually              do anything but return the original string. The LocalizationProcessor application walks              through the source code of a project and looks for LOC.COMMANDNAME and builds a record for each command              name for the translators that can be used by developers in a commands overridden Rhino.Commands.Command.LocalName              which should call Rhino.UI.Localization.LocalizeCommandName(EnglishName)
