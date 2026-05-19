---
name: allplan-nemall_python_utility
description: This skill encodes the allplan 2025.0 surface of the NemAll_Python_Utility namespace — 20 types with full vendor-documented methods, properties, events, and behavioral remarks. Read when composing apps that touch any of: Functions, DateDialog, DefaultDirectories, FileDialog, GUID, ProgressBar, InitTkinter, SuppressLangDllErrorMessages, and 12 more types.
---

# NemAll_Python_Utility

Auto-generated from vendor docs for allplan 2025.0. 20 types in this namespace.

## DateDialog (class)

(No description provided in vendor docs for NemAll_Python_Utility.DateDialog.)

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Utility/DateDialog/)

### Methods
#### `GetDate(dateText: str, headerText: str) -> str`

Implementation of the file dialog expose

**Remarks:** Implementation of the file dialog expose

**Parameters:**
- `dateText` (str) — default date
- `headerText` (str) — header text

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Utility/DateDialog/#NemAll_Python_Utility.DateDialog.GetDate)

## DefaultDirectories (class)

Default directories

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Utility/DefaultDirectories/)

### Constructors
- `DefaultDirectories()` — Initialize

### Methods
#### `AddDirectory(path: str) -> bool`

Add a directory

**Remarks:** Add a directory

**Parameters:**
- `path` (str) — path of the directory

**Returns:** `bool` — directory added: true/false

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Utility/DefaultDirectories/#NemAll_Python_Utility.DefaultDirectories.AddDirectory)

## FileDialog (class)

File dialog

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Utility/FileDialog/)

### Constructors
- `FileDialog()` — Initialize

### Methods
#### `AskOpenFavoriteFile( defaultPath: str, title: str, filters: str, extension: str ) -> str`

Ask for a favorite file to open

**Remarks:** Ask for a favorite file to open

**Parameters:**
- `defaultPath` (str) — Initial default path
- `title` (str) — Title of the dialog. If the title is empty an internal string will be used
- `filters` (str) — Filters of the files
- `extension` (str) — Extension of the file

**Returns:** `str` — selected file

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Utility/FileDialog/#NemAll_Python_Utility.FileDialog.AskOpenFavoriteFile)

#### `AskOpenFile( defaultPath: str, title: str, filters: str, extension: str, defaultDir: DefaultDirectories, ) -> str`

Ask for a file to open

**Remarks:** Ask for a file to open

**Parameters:**
- `defaultPath` (str) — Initial default path
- `title` (str) — Title of the dialog. If the title is empty an internal string will be used
- `filters` (str) — Filters of the files
- `extension` (str) — Extension of the file
- `defaultDir` (DefaultDirectories) — Default directories

**Returns:** `str` — selected file

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Utility/FileDialog/#NemAll_Python_Utility.FileDialog.AskOpenFile)

#### `AskSaveFavoriteFile( defaultPath: str, title: str, filters: str, extension: str ) -> str`

Ask for a favorite file to save

**Remarks:** Ask for a favorite file to save

**Parameters:**
- `defaultPath` (str) — Initial default path
- `title` (str) — Title of the dialog. If the title is empty an internal string will be used
- `filters` (str) — Filters of the files
- `extension` (str) — Extension of the file

**Returns:** `str` — selected file

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Utility/FileDialog/#NemAll_Python_Utility.FileDialog.AskSaveFavoriteFile)

#### `AskSaveFile( defaultPath: str, title: str, filters: str, extension: str, defaultDir: DefaultDirectories, ) -> str`

Ask for a file to save

**Remarks:** Ask for a file to save

**Parameters:**
- `defaultPath` (str) — Initial default path
- `title` (str) — Title of the dialog. If the title is empty an internal string will be used
- `filters` (str) — Filters of the files
- `extension` (str) — Extension of the file
- `defaultDir` (DefaultDirectories) — Default directories

**Returns:** `str` — selected file

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Utility/FileDialog/#NemAll_Python_Utility.FileDialog.AskSaveFile)

## Functions (static-class)

Module-level functions of NemAll_Python_Utility

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Utility/)

### Methods
#### `CheckModuleLicence(moduleNumber: int) -> bool`

Check for a module licence

**Remarks:** Check for a module licence

**Parameters:**
- `moduleNumber` (int) — Number of the module

**Returns:** `bool` — Licence for the module available: True/False

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Utility/#NemAll_Python_Utility.CheckModuleLicence)

#### `ClearUnitTestDocument()`

Clear the document

**Remarks:** Clear the document

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Utility/#NemAll_Python_Utility.ClearUnitTestDocument)

#### `EncryptString(stringtoEncrypt: str, pypName: str) -> str`

ImportScript an encrypted PythonPart script

**Remarks:** ImportScript an encrypted PythonPart script

**Parameters:**
- `stringtoEncrypt` (str) — Path of the file
- `pypName` (str) — Name of the PythonPart

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Utility/#NemAll_Python_Utility.EncryptString)

#### `GetPluginNameHash() -> int`



[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Utility/#NemAll_Python_Utility.GetPluginNameHash)

#### `ImportScript(script: str, global_dict: object, pypName: str)`

ImportScript an encrypted PythonPart script

**Remarks:** ImportScript an encrypted PythonPart script

**Parameters:**
- `script` (str) — Path of the file
- `global_dict` (object) — File name
- `pypName` (str) — Name of the PythonPart

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Utility/#NemAll_Python_Utility.ImportScript)

#### `InitReinfNormInterpreter()`



[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Utility/#NemAll_Python_Utility.InitReinfNormInterpreter)

#### `InitUnitTest(loadResources: bool) -> DocumentAdapter`

Initialize the unit test

**Remarks:** Initialize the unit test

**Parameters:**
- `loadResources` (bool) — Load the resources

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Utility/#NemAll_Python_Utility.InitUnitTest)

#### `IsExecutedByUnitTest() -> bool`

Check, whether the script is executed by a unit test

**Remarks:** Check, whether the script is executed by a unit test

**Returns:** `bool` — Script is executed by a unit test: True/False

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Utility/#NemAll_Python_Utility.IsExecutedByUnitTest)

#### `LoadSymbolForUnitTest( file: str, clearDocument: bool, updateArchEleGeometry: bool = False ) -> BaseElementAdapterList`

Load a symbol to the Array

**Remarks:** Load a symbol to the Array

**Parameters:**
- `file` (str) — File name
- `clearDocument` (bool) — Clear the document before symbol loading
- `updateArchEleGeometry` (bool) — Update the geometry elements after load (e.g. adapts slabs to the planes

**Returns:** `BaseElementAdapterList` — Created elements

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Utility/#NemAll_Python_Utility.LoadSymbolForUnitTest)

#### `ShowMessageBox(text: str, flags: int, dontAsk: int = 0) -> int`

Displays Message Box

**Remarks:** Displays Message Box

**Parameters:**
- `text` (str) — Message box text
- `flags` (int) — Button flags dontAsk text number for the text that will be displayed when MB_DONOTASKAGAIN is used

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Utility/#NemAll_Python_Utility.ShowMessageBox)

## GUID (class)

(No description provided in vendor docs for NemAll_Python_Utility.GUID.)

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Utility/GUID/)

### Constructors
- `GUID()` — Initialize

### Methods
#### `FromString(strGuid: str) -> GUID`

Create a GUID from a string

**Remarks:** Create a GUID from a string

**Parameters:**
- `strGuid` (str) — GUID as string

**Returns:** `GUID` — GUID from the string

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Utility/GUID/#NemAll_Python_Utility.GUID.FromString)

#### `__eq__(compGuid: GUID) -> bool`

Compare a GUID

**Remarks:** Compare a GUID

**Parameters:**
- `compGuid` (GUID) — GUID to compare

**Returns:** `bool` — GUIDs are equal: True/False

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Utility/GUID/#NemAll_Python_Utility.GUID.__eq__)

#### `__hash__() -> int`

Create a hash from the GUID

**Remarks:** Create a hash from the GUID

**Returns:** `int` — hash from the GUID

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Utility/GUID/#NemAll_Python_Utility.GUID.__hash__)

#### `__repr__() -> str`

Create a string from the GUID

**Remarks:** Create a string from the GUID

**Returns:** `str` — string from the GUID

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Utility/GUID/#NemAll_Python_Utility.GUID.__repr__)

## InitTkinter (class)

(No description provided in vendor docs for NemAll_Python_Utility.InitTkinter.)

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Utility/InitTkinter/)

### Constructors
- `InitTkinter()` — Initialize the use of tkinter in Allplan

## ProgressBar (class)

Representation of the progress bar

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Utility/ProgressBar/)

### Constructors
- `ProgressBar() | ProgressBar(element: ProgressBar) | ProgressBar(countOfSteps: int, headerTextNumber: int, bWithCancel: bool)` — Shows the progress bar

### Methods
#### `CloseProgressbar() -> bool`

Closed progressbar dialog

**Remarks:** Closed progressbar dialog

**Returns:** `bool` — true if progressbar closed correctly

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Utility/ProgressBar/#NemAll_Python_Utility.ProgressBar.CloseProgressbar)

#### `MakeStep(step: int) -> bool | MakeStep(step: int, additionalInfoStrID: int) -> bool | MakeStep(step: int, additionalInfo: str) -> bool`

Make a step

**Remarks:** Make a step

**Parameters:**
- `step` (int) — step count

**Returns:** `bool` — true step is correctly executed

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Utility/ProgressBar/#NemAll_Python_Utility.ProgressBar.MakeStep)

#### `ResetProgressBar( numberOfSteps: int, progressBarTitle: str, additionalInfo: str, closable: bool, ) -> bool | ResetProgressBar( numberOfSteps: int, progressBarTitleStrID: int, additionalInfoStrID: int, closable: bool, ) -> bool`

Reset progressbar dialog

**Remarks:** Reset progressbar dialog

**Parameters:**
- `numberOfSteps` (int) — number of steps/elements
- `progressBarTitle` (str) — title of progressbar
- `additionalInfo` (str) — additional info
- `closable` (bool) — progressbar can be canceled

**Returns:** `bool` — true if progressbar started correctly

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Utility/ProgressBar/#NemAll_Python_Utility.ProgressBar.ResetProgressBar)

#### `SetAditionalInfo(textId: int) -> bool | SetAditionalInfo(text: str) -> bool`

Set the additional info

**Remarks:** Set the additional info

**Parameters:**
- `textId` (int) — additional info text ID

**Returns:** `bool` — true if set correctly

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Utility/ProgressBar/#NemAll_Python_Utility.ProgressBar.SetAditionalInfo)

#### `SetInfinitProgressbar(isInfinit: bool) -> bool`

Set infinite progressbar dialog

**Remarks:** Set infinite progressbar dialog

**Parameters:**
- `isInfinit` (bool) — infinite state

**Returns:** `bool` — true if set correctly

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Utility/ProgressBar/#NemAll_Python_Utility.ProgressBar.SetInfinitProgressbar)

#### `SetIsClosable(isClosable: bool) -> bool`

Set the closeable state

**Remarks:** Set the closeable state

**Parameters:**
- `isClosable` (bool) — closable state

**Returns:** `bool` — true if set correctly

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Utility/ProgressBar/#NemAll_Python_Utility.ProgressBar.SetIsClosable)

#### `SetNumberOfSteps(numberOfSteps: int) -> bool`

Set number of steps

**Remarks:** Set number of steps

**Parameters:**
- `numberOfSteps` (int) — number of steps/elements

**Returns:** `bool` — true if set correctly

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Utility/ProgressBar/#NemAll_Python_Utility.ProgressBar.SetNumberOfSteps)

#### `SetTitle(textId: int) -> bool | SetTitle(text: str) -> bool`

Set the title

**Remarks:** Set the title

**Parameters:**
- `textId` (int) — title text ID

**Returns:** `bool` — true if set correctly

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Utility/ProgressBar/#NemAll_Python_Utility.ProgressBar.SetTitle)

#### `StartInfiniteProgressbar( progressBarTitle: str, additionalInfo: str, closable: bool, startImmediately: bool = False, ) -> bool | StartInfiniteProgressbar( progressBarTitleID: int, additionalInfoStrID: int, closable: bool, startImmediately: bool = False, ) -> bool | StartInfiniteProgressbar( progressBarTitle: str, closable: bool, startImmediately: bool = False ) -> bool | StartInfiniteProgressbar( progressBarTitleID: int, closable: bool, startImmediately: bool = False ) -> bool`

Starts infinite progressbar dialog

**Remarks:** Starts infinite progressbar dialog

**Parameters:**
- `progressBarTitle` (str) — title of progressbar
- `additionalInfo` (str) — additional info
- `closable` (bool) — progressbar can be canceled
- `startImmediately` (bool) — start immediately

**Returns:** `bool` — true if progressbar started correctly

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Utility/ProgressBar/#NemAll_Python_Utility.ProgressBar.StartInfiniteProgressbar)

#### `StartProgressbar( numberOfSteps: int, progressBarTitle: str, additionalInfo: str, closable: bool, startImmediately: bool = False, ) -> bool | StartProgressbar( numberOfSteps: int, progressBarTitleID: int, additionalInfoStrID: int, closable: bool, startImmediately: bool = False, ) -> bool | StartProgressbar( numberOfSteps: int, progressBarTitle: str, closable: bool, startImmediately: bool = False, ) -> bool | StartProgressbar( numberOfSteps: int, progressBarTitleID: int, closable: bool, startImmediately: bool = False, ) -> bool`

Starts progressbar dialog

**Remarks:** Starts progressbar dialog

**Parameters:**
- `numberOfSteps` (int) — number of steps/elements
- `progressBarTitle` (str) — title of progressbar
- `additionalInfo` (str) — additional info
- `closable` (bool) — progressbar can be canceled
- `startImmediately` (bool) — start immediately

**Returns:** `bool` — true if progressbar started correctly

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Utility/ProgressBar/#NemAll_Python_Utility.ProgressBar.StartProgressbar)

#### `Step() -> bool`

Increase the progress bar by one step

**Remarks:** Increase the progress bar by one step

**Returns:** `bool` — True when cancel button was clicked, False otherwise

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Utility/ProgressBar/#NemAll_Python_Utility.ProgressBar.Step)

## SizeTList (class)

(No description provided in vendor docs for NemAll_Python_Utility.SizeTList.)

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Utility/SizeTList/)

### Constructors
- `SizeTList()` — Initialize

### Methods
#### `__contains__(value: int) -> bool`

Check for a value in the list

**Remarks:** Check for a value in the list

**Parameters:**
- `value` (int) — Value to check

**Returns:** `bool` — State for value is in the list

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Utility/SizeTList/#NemAll_Python_Utility.SizeTList.__contains__)

#### `__delitem__(value: int)`

Delete a list item

**Remarks:** Delete a list item

**Parameters:**
- `value` (int) — Value to delete

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Utility/SizeTList/#NemAll_Python_Utility.SizeTList.__delitem__)

#### `__eq__(compare_list: SizeTList) -> bool`

Compare two list

**Remarks:** Compare two list

**Parameters:**
- `compare_list` (SizeTList) — List to compare

**Returns:** `bool` — Lists are equal state

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Utility/SizeTList/#NemAll_Python_Utility.SizeTList.__eq__)

#### `__getitem__(index: int) -> int`

Get a list item

**Remarks:** Get a list item

**Parameters:**
- `index` (int) — Index of the item

**Returns:** `int` — Value for the index

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Utility/SizeTList/#NemAll_Python_Utility.SizeTList.__getitem__)

#### `__iter__() -> Iterator`

List iterator

**Remarks:** List iterator

**Returns:** `Iterator` — List iterator

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Utility/SizeTList/#NemAll_Python_Utility.SizeTList.__iter__)

#### `__len__() -> int`

Get the list length

**Remarks:** Get the list length

**Returns:** `int` — Length of the list

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Utility/SizeTList/#NemAll_Python_Utility.SizeTList.__len__)

#### `__repr__() -> str`

Convert the list to a string

**Remarks:** Convert the list to a string

**Returns:** `str` — List values as string

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Utility/SizeTList/#NemAll_Python_Utility.SizeTList.__repr__)

#### `__setitem__(index: int | slice, value: int)`

Set a list item

**Remarks:** Set a list item

**Parameters:**
- `index` (int | slice) — Index of the item
- `value` (int) — Value to item

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Utility/SizeTList/#NemAll_Python_Utility.SizeTList.__setitem__)

#### `append(value: int)`

Append a list item

**Remarks:** Append a list item

**Parameters:**
- `value` (int) — Value to append

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Utility/SizeTList/#NemAll_Python_Utility.SizeTList.append)

#### `count(value: int) -> int`

Get the values in the list

**Remarks:** Get the values in the list

**Parameters:**
- `value` (int) — Value to count

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Utility/SizeTList/#NemAll_Python_Utility.SizeTList.count)

#### `extend(iterable: SizeTList)`

Add the items from an iterable to the end of the list

**Remarks:** Add the items from an iterable to the end of the list

**Parameters:**
- `iterable` (SizeTList) — Iterable to add

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Utility/SizeTList/#NemAll_Python_Utility.SizeTList.extend)

#### `index(value: int) -> int`

Get the index for the value

**Remarks:** Get the index for the value

**Parameters:**
- `value` (int) — Value to find

**Returns:** `int` — Index for the value

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Utility/SizeTList/#NemAll_Python_Utility.SizeTList.index)

#### `insert(index: int, item: int)`

Insert an item in the list

**Remarks:** Insert an item in the list

**Parameters:**
- `index` (int) — Index of the item
- `item` (int) — Value to insert

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Utility/SizeTList/#NemAll_Python_Utility.SizeTList.insert)

#### `pop() -> int | pop(index: int) -> int`

Pop the last value from the list

**Remarks:** Pop the last value from the list

**Returns:** `int` — Last value from the list

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Utility/SizeTList/#NemAll_Python_Utility.SizeTList.pop)

#### `remove(value: int)`

Remove a value from the list

**Remarks:** Remove a value from the list

**Parameters:**
- `value` (int) — Value to remove

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Utility/SizeTList/#NemAll_Python_Utility.SizeTList.remove)

#### `reverse()`

Reverse the list

**Remarks:** Reverse the list

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Utility/SizeTList/#NemAll_Python_Utility.SizeTList.reverse)

#### `sort() | sort(cmp: object)`

Sort the list

**Remarks:** Sort the list

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Utility/SizeTList/#NemAll_Python_Utility.SizeTList.sort)

## SuppressLangDllErrorMessages (class)

(No description provided in vendor docs for NemAll_Python_Utility.SuppressLangDllErrorMessages.)

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Utility/SuppressLangDllErrorMessages/)

### Constructors
- `SuppressLangDllErrorMessages()` — Initialize

## Timer (class)

Timer

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Utility/Timer/)

### Constructors
- `Timer() | Timer(timer: Timer)` — Initialize

### Methods
#### `PrintTime(bReset: bool)`

Print the time

**Remarks:** Print the time

**Parameters:**
- `bReset` (bool) — Reset the timer

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Utility/Timer/#NemAll_Python_Utility.Timer.PrintTime)

## VecByteList (class)

(No description provided in vendor docs for NemAll_Python_Utility.VecByteList.)

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Utility/VecByteList/)

### Constructors
- `VecByteList() | VecByteList(valueList: list)` — Initialize

### Methods
#### `__contains__(value: int) -> bool`

Check for a value in the list

**Remarks:** Check for a value in the list

**Parameters:**
- `value` (int) — Value to check

**Returns:** `bool` — State for value is in the list

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Utility/VecByteList/#NemAll_Python_Utility.VecByteList.__contains__)

#### `__delitem__(value: int)`

Delete a list item

**Remarks:** Delete a list item

**Parameters:**
- `value` (int) — Value to delete

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Utility/VecByteList/#NemAll_Python_Utility.VecByteList.__delitem__)

#### `__eq__(compare_list: VecByteList) -> bool`

Compare two list

**Remarks:** Compare two list

**Parameters:**
- `compare_list` (VecByteList) — List to compare

**Returns:** `bool` — Lists are equal state

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Utility/VecByteList/#NemAll_Python_Utility.VecByteList.__eq__)

#### `__getitem__(index: int) -> int`

Get a list item

**Remarks:** Get a list item

**Parameters:**
- `index` (int) — Index of the item

**Returns:** `int` — Value for the index

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Utility/VecByteList/#NemAll_Python_Utility.VecByteList.__getitem__)

#### `__iter__() -> Iterator`

List iterator

**Remarks:** List iterator

**Returns:** `Iterator` — List iterator

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Utility/VecByteList/#NemAll_Python_Utility.VecByteList.__iter__)

#### `__len__() -> int`

Get the list length

**Remarks:** Get the list length

**Returns:** `int` — Length of the list

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Utility/VecByteList/#NemAll_Python_Utility.VecByteList.__len__)

#### `__repr__() -> str`

Create a string from the values

**Remarks:** Create a string from the values

**Returns:** `str` — String

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Utility/VecByteList/#NemAll_Python_Utility.VecByteList.__repr__)

#### `__setitem__(index: int | slice, value: int)`

Set a list item

**Remarks:** Set a list item

**Parameters:**
- `index` (int | slice) — Index of the item
- `value` (int) — Value to item

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Utility/VecByteList/#NemAll_Python_Utility.VecByteList.__setitem__)

#### `append(value: int)`

Append a list item

**Remarks:** Append a list item

**Parameters:**
- `value` (int) — Value to append

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Utility/VecByteList/#NemAll_Python_Utility.VecByteList.append)

#### `extend(iterable: VecByteList)`

Add the items from an iterable to the end of the list

**Remarks:** Add the items from an iterable to the end of the list

**Parameters:**
- `iterable` (VecByteList) — Iterable to add

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Utility/VecByteList/#NemAll_Python_Utility.VecByteList.extend)

## VecDoubleList (class)

(No description provided in vendor docs for NemAll_Python_Utility.VecDoubleList.)

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Utility/VecDoubleList/)

### Constructors
- `VecDoubleList() | VecDoubleList(valueList: list)` — Initialize

### Methods
#### `__contains__(value: float) -> bool`

Check for a value in the list

**Remarks:** Check for a value in the list

**Parameters:**
- `value` (float) — Value to check

**Returns:** `bool` — State for value is in the list

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Utility/VecDoubleList/#NemAll_Python_Utility.VecDoubleList.__contains__)

#### `__delitem__(value: float)`

Delete a list item

**Remarks:** Delete a list item

**Parameters:**
- `value` (float) — Value to delete

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Utility/VecDoubleList/#NemAll_Python_Utility.VecDoubleList.__delitem__)

#### `__eq__(compare_list: VecDoubleList) -> bool`

Compare two list

**Remarks:** Compare two list

**Parameters:**
- `compare_list` (VecDoubleList) — List to compare

**Returns:** `bool` — Lists are equal state

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Utility/VecDoubleList/#NemAll_Python_Utility.VecDoubleList.__eq__)

#### `__getitem__(index: int) -> float`

Get a list item

**Remarks:** Get a list item

**Parameters:**
- `index` (int) — Index of the item

**Returns:** `float` — Value for the index

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Utility/VecDoubleList/#NemAll_Python_Utility.VecDoubleList.__getitem__)

#### `__iter__() -> Iterator`

List iterator

**Remarks:** List iterator

**Returns:** `Iterator` — List iterator

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Utility/VecDoubleList/#NemAll_Python_Utility.VecDoubleList.__iter__)

#### `__len__() -> int`

Get the list length

**Remarks:** Get the list length

**Returns:** `int` — Length of the list

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Utility/VecDoubleList/#NemAll_Python_Utility.VecDoubleList.__len__)

#### `__repr__() -> str`

Create a string from the values

**Remarks:** Create a string from the values

**Returns:** `str` — String

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Utility/VecDoubleList/#NemAll_Python_Utility.VecDoubleList.__repr__)

#### `__setitem__(index: int | slice, value: float)`

Set a list item

**Remarks:** Set a list item

**Parameters:**
- `index` (int | slice) — Index of the item
- `value` (float) — Value to item

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Utility/VecDoubleList/#NemAll_Python_Utility.VecDoubleList.__setitem__)

#### `append(value: float)`

Append a list item

**Remarks:** Append a list item

**Parameters:**
- `value` (float) — Value to append

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Utility/VecDoubleList/#NemAll_Python_Utility.VecDoubleList.append)

#### `extend(iterable: VecDoubleList)`

Add the items from an iterable to the end of the list

**Remarks:** Add the items from an iterable to the end of the list

**Parameters:**
- `iterable` (VecDoubleList) — Iterable to add

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Utility/VecDoubleList/#NemAll_Python_Utility.VecDoubleList.extend)

## VecGUIDList (class)

(No description provided in vendor docs for NemAll_Python_Utility.VecGUIDList.)

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Utility/VecGUIDList/)

### Constructors
- `VecGUIDList()` — Initialize

### Methods
#### `__contains__(value: GUID) -> bool`

Check for a value in the list

**Remarks:** Check for a value in the list

**Parameters:**
- `value` (GUID) — Value to check

**Returns:** `bool` — State for value is in the list

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Utility/VecGUIDList/#NemAll_Python_Utility.VecGUIDList.__contains__)

#### `__delitem__(value: GUID)`

Delete a list item

**Remarks:** Delete a list item

**Parameters:**
- `value` (GUID) — Value to delete

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Utility/VecGUIDList/#NemAll_Python_Utility.VecGUIDList.__delitem__)

#### `__eq__(compare_list: VecGUIDList) -> bool`

Compare two list

**Remarks:** Compare two list

**Parameters:**
- `compare_list` (VecGUIDList) — List to compare

**Returns:** `bool` — Lists are equal state

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Utility/VecGUIDList/#NemAll_Python_Utility.VecGUIDList.__eq__)

#### `__getitem__(index: int) -> GUID`

Get a list item

**Remarks:** Get a list item

**Parameters:**
- `index` (int) — Index of the item

**Returns:** `GUID` — Value for the index

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Utility/VecGUIDList/#NemAll_Python_Utility.VecGUIDList.__getitem__)

#### `__iter__() -> Iterator`

List iterator

**Remarks:** List iterator

**Returns:** `Iterator` — List iterator

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Utility/VecGUIDList/#NemAll_Python_Utility.VecGUIDList.__iter__)

#### `__len__() -> int`

Get the list length

**Remarks:** Get the list length

**Returns:** `int` — Length of the list

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Utility/VecGUIDList/#NemAll_Python_Utility.VecGUIDList.__len__)

#### `__setitem__(index: int | slice, value: GUID)`

Set a list item

**Remarks:** Set a list item

**Parameters:**
- `index` (int | slice) — Index of the item
- `value` (GUID) — Value to item

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Utility/VecGUIDList/#NemAll_Python_Utility.VecGUIDList.__setitem__)

#### `append(value: GUID)`

Append a list item

**Remarks:** Append a list item

**Parameters:**
- `value` (GUID) — Value to append

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Utility/VecGUIDList/#NemAll_Python_Utility.VecGUIDList.append)

#### `extend(iterable: VecGUIDList)`

Add the items from an iterable to the end of the list

**Remarks:** Add the items from an iterable to the end of the list

**Parameters:**
- `iterable` (VecGUIDList) — Iterable to add

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Utility/VecGUIDList/#NemAll_Python_Utility.VecGUIDList.extend)

## VecIntList (class)

(No description provided in vendor docs for NemAll_Python_Utility.VecIntList.)

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Utility/VecIntList/)

### Constructors
- `VecIntList() | VecIntList(valueList: list)` — Initialize

### Methods
#### `__contains__(value: int) -> bool`

Check for a value in the list

**Remarks:** Check for a value in the list

**Parameters:**
- `value` (int) — Value to check

**Returns:** `bool` — State for value is in the list

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Utility/VecIntList/#NemAll_Python_Utility.VecIntList.__contains__)

#### `__delitem__(value: int)`

Delete a list item

**Remarks:** Delete a list item

**Parameters:**
- `value` (int) — Value to delete

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Utility/VecIntList/#NemAll_Python_Utility.VecIntList.__delitem__)

#### `__eq__(compare_list: list[int] | VecIntList) -> bool`

Compare two list

**Remarks:** Compare two list

**Parameters:**
- `compare_list` (list[int] | VecIntList) — List to compare

**Returns:** `bool` — Lists are equal state

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Utility/VecIntList/#NemAll_Python_Utility.VecIntList.__eq__)

#### `__getitem__(index: int) -> int`

Get a list item

**Remarks:** Get a list item

**Parameters:**
- `index` (int) — Index of the item

**Returns:** `int` — Value for the index

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Utility/VecIntList/#NemAll_Python_Utility.VecIntList.__getitem__)

#### `__iter__() -> Iterator`

List iterator

**Remarks:** List iterator

**Returns:** `Iterator` — List iterator

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Utility/VecIntList/#NemAll_Python_Utility.VecIntList.__iter__)

#### `__len__() -> int`

Get the list length

**Remarks:** Get the list length

**Returns:** `int` — Length of the list

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Utility/VecIntList/#NemAll_Python_Utility.VecIntList.__len__)

#### `__repr__() -> str`

Create a string from the values

**Remarks:** Create a string from the values

**Returns:** `str` — String

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Utility/VecIntList/#NemAll_Python_Utility.VecIntList.__repr__)

#### `__setitem__(index: int | slice, value: int)`

Set a list item

**Remarks:** Set a list item

**Parameters:**
- `index` (int | slice) — Index of the item
- `value` (int) — Value to item

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Utility/VecIntList/#NemAll_Python_Utility.VecIntList.__setitem__)

#### `append(value: int)`

Append a list item

**Remarks:** Append a list item

**Parameters:**
- `value` (int) — Value to append

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Utility/VecIntList/#NemAll_Python_Utility.VecIntList.append)

#### `extend(iterable: list[int] | VecIntList)`

Add the items from an iterable to the end of the list

**Remarks:** Add the items from an iterable to the end of the list

**Parameters:**
- `iterable` (list[int] | VecIntList) — Iterable to add

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Utility/VecIntList/#NemAll_Python_Utility.VecIntList.extend)

## VecSizeTList (class)

(No description provided in vendor docs for NemAll_Python_Utility.VecSizeTList.)

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Utility/VecSizeTList/)

### Constructors
- `VecSizeTList() | VecSizeTList(valueList: list)` — Initialize

### Methods
#### `__contains__(value: int) -> bool`

Check for a value in the list

**Remarks:** Check for a value in the list

**Parameters:**
- `value` (int) — Value to check

**Returns:** `bool` — State for value is in the list

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Utility/VecSizeTList/#NemAll_Python_Utility.VecSizeTList.__contains__)

#### `__delitem__(value: int)`

Delete a list item

**Remarks:** Delete a list item

**Parameters:**
- `value` (int) — Value to delete

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Utility/VecSizeTList/#NemAll_Python_Utility.VecSizeTList.__delitem__)

#### `__eq__(compare_list: VecSizeTList) -> bool`

Compare two list

**Remarks:** Compare two list

**Parameters:**
- `compare_list` (VecSizeTList) — List to compare

**Returns:** `bool` — Lists are equal state

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Utility/VecSizeTList/#NemAll_Python_Utility.VecSizeTList.__eq__)

#### `__getitem__(index: int) -> int`

Get a list item

**Remarks:** Get a list item

**Parameters:**
- `index` (int) — Index of the item

**Returns:** `int` — Value for the index

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Utility/VecSizeTList/#NemAll_Python_Utility.VecSizeTList.__getitem__)

#### `__iter__() -> Iterator`

List iterator

**Remarks:** List iterator

**Returns:** `Iterator` — List iterator

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Utility/VecSizeTList/#NemAll_Python_Utility.VecSizeTList.__iter__)

#### `__len__() -> int`

Get the list length

**Remarks:** Get the list length

**Returns:** `int` — Length of the list

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Utility/VecSizeTList/#NemAll_Python_Utility.VecSizeTList.__len__)

#### `__repr__() -> str`

Create a string from the values

**Remarks:** Create a string from the values

**Returns:** `str` — String

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Utility/VecSizeTList/#NemAll_Python_Utility.VecSizeTList.__repr__)

#### `__setitem__(index: int | slice, value: int)`

Set a list item

**Remarks:** Set a list item

**Parameters:**
- `index` (int | slice) — Index of the item
- `value` (int) — Value to item

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Utility/VecSizeTList/#NemAll_Python_Utility.VecSizeTList.__setitem__)

#### `append(value: int)`

Append a list item

**Remarks:** Append a list item

**Parameters:**
- `value` (int) — Value to append

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Utility/VecSizeTList/#NemAll_Python_Utility.VecSizeTList.append)

#### `extend(iterable: VecSizeTList)`

Add the items from an iterable to the end of the list

**Remarks:** Add the items from an iterable to the end of the list

**Parameters:**
- `iterable` (VecSizeTList) — Iterable to add

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Utility/VecSizeTList/#NemAll_Python_Utility.VecSizeTList.extend)

## VecStringList (class)

(No description provided in vendor docs for NemAll_Python_Utility.VecStringList.)

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Utility/VecStringList/)

### Constructors
- `VecStringList() | VecStringList(valueList: list)` — Initialize

### Methods
#### `__contains__(value: str) -> bool`

Check for a value in the list

**Remarks:** Check for a value in the list

**Parameters:**
- `value` (str) — Value to check

**Returns:** `bool` — State for value is in the list

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Utility/VecStringList/#NemAll_Python_Utility.VecStringList.__contains__)

#### `__delitem__(value: str)`

Delete a list item

**Remarks:** Delete a list item

**Parameters:**
- `value` (str) — Value to delete

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Utility/VecStringList/#NemAll_Python_Utility.VecStringList.__delitem__)

#### `__eq__(compare_list: VecStringList) -> bool`

Compare two list

**Remarks:** Compare two list

**Parameters:**
- `compare_list` (VecStringList) — List to compare

**Returns:** `bool` — Lists are equal state

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Utility/VecStringList/#NemAll_Python_Utility.VecStringList.__eq__)

#### `__getitem__(index: int) -> str`

Get a list item

**Remarks:** Get a list item

**Parameters:**
- `index` (int) — Index of the item

**Returns:** `str` — Value for the index

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Utility/VecStringList/#NemAll_Python_Utility.VecStringList.__getitem__)

#### `__iter__() -> Iterator`

List iterator

**Remarks:** List iterator

**Returns:** `Iterator` — List iterator

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Utility/VecStringList/#NemAll_Python_Utility.VecStringList.__iter__)

#### `__len__() -> int`

Get the list length

**Remarks:** Get the list length

**Returns:** `int` — Length of the list

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Utility/VecStringList/#NemAll_Python_Utility.VecStringList.__len__)

#### `__repr__() -> str`

Create a string from the values

**Remarks:** Create a string from the values

**Returns:** `str` — String

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Utility/VecStringList/#NemAll_Python_Utility.VecStringList.__repr__)

#### `__setitem__(index: int | slice, value: str)`

Set a list item

**Remarks:** Set a list item

**Parameters:**
- `index` (int | slice) — Index of the item
- `value` (str) — Value to item

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Utility/VecStringList/#NemAll_Python_Utility.VecStringList.__setitem__)

#### `append(value: str)`

Append a list item

**Remarks:** Append a list item

**Parameters:**
- `value` (str) — Value to append

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Utility/VecStringList/#NemAll_Python_Utility.VecStringList.append)

#### `extend(iterable: VecStringList)`

Add the items from an iterable to the end of the list

**Remarks:** Add the items from an iterable to the end of the list

**Parameters:**
- `iterable` (VecStringList) — Iterable to add

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Utility/VecStringList/#NemAll_Python_Utility.VecStringList.extend)

## VecUIntList (class)

(No description provided in vendor docs for NemAll_Python_Utility.VecUIntList.)

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Utility/VecUIntList/)

### Constructors
- `VecUIntList() | VecUIntList(valueList: list)` — Initialize

### Methods
#### `__contains__(value: int) -> bool`

Check for a value in the list

**Remarks:** Check for a value in the list

**Parameters:**
- `value` (int) — Value to check

**Returns:** `bool` — State for value is in the list

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Utility/VecUIntList/#NemAll_Python_Utility.VecUIntList.__contains__)

#### `__delitem__(value: int)`

Delete a list item

**Remarks:** Delete a list item

**Parameters:**
- `value` (int) — Value to delete

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Utility/VecUIntList/#NemAll_Python_Utility.VecUIntList.__delitem__)

#### `__eq__(compare_list: list[int] | VecUIntList) -> bool`

Compare two list

**Remarks:** Compare two list

**Parameters:**
- `compare_list` (list[int] | VecUIntList) — List to compare

**Returns:** `bool` — Lists are equal state

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Utility/VecUIntList/#NemAll_Python_Utility.VecUIntList.__eq__)

#### `__getitem__(index: int) -> int`

Get a list item

**Remarks:** Get a list item

**Parameters:**
- `index` (int) — Index of the item

**Returns:** `int` — Value for the index

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Utility/VecUIntList/#NemAll_Python_Utility.VecUIntList.__getitem__)

#### `__iter__() -> Iterator`

List iterator

**Remarks:** List iterator

**Returns:** `Iterator` — List iterator

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Utility/VecUIntList/#NemAll_Python_Utility.VecUIntList.__iter__)

#### `__len__() -> int`

Get the list length

**Remarks:** Get the list length

**Returns:** `int` — Length of the list

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Utility/VecUIntList/#NemAll_Python_Utility.VecUIntList.__len__)

#### `__repr__() -> str`

Create a string from the values

**Remarks:** Create a string from the values

**Returns:** `str` — String

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Utility/VecUIntList/#NemAll_Python_Utility.VecUIntList.__repr__)

#### `__setitem__(index: int | slice, value: int)`

Set a list item

**Remarks:** Set a list item

**Parameters:**
- `index` (int | slice) — Index of the item
- `value` (int) — Value to item

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Utility/VecUIntList/#NemAll_Python_Utility.VecUIntList.__setitem__)

#### `append(value: int)`

Append a list item

**Remarks:** Append a list item

**Parameters:**
- `value` (int) — Value to append

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Utility/VecUIntList/#NemAll_Python_Utility.VecUIntList.append)

#### `extend(iterable: list[int] | VecUIntList)`

Add the items from an iterable to the end of the list

**Remarks:** Add the items from an iterable to the end of the list

**Parameters:**
- `iterable` (list[int] | VecUIntList) — Iterable to add

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Utility/VecUIntList/#NemAll_Python_Utility.VecUIntList.extend)

## VecULongList (class)

(No description provided in vendor docs for NemAll_Python_Utility.VecULongList.)

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Utility/VecULongList/)

### Constructors
- `VecULongList() | VecULongList(valueList: list)` — Initialize

### Methods
#### `__contains__(value: int) -> bool`

Check for a value in the list

**Remarks:** Check for a value in the list

**Parameters:**
- `value` (int) — Value to check

**Returns:** `bool` — State for value is in the list

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Utility/VecULongList/#NemAll_Python_Utility.VecULongList.__contains__)

#### `__delitem__(value: int)`

Delete a list item

**Remarks:** Delete a list item

**Parameters:**
- `value` (int) — Value to delete

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Utility/VecULongList/#NemAll_Python_Utility.VecULongList.__delitem__)

#### `__eq__(compare_list: VecULongList) -> bool`

Compare two list

**Remarks:** Compare two list

**Parameters:**
- `compare_list` (VecULongList) — List to compare

**Returns:** `bool` — Lists are equal state

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Utility/VecULongList/#NemAll_Python_Utility.VecULongList.__eq__)

#### `__getitem__(index: int) -> int`

Get a list item

**Remarks:** Get a list item

**Parameters:**
- `index` (int) — Index of the item

**Returns:** `int` — Value for the index

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Utility/VecULongList/#NemAll_Python_Utility.VecULongList.__getitem__)

#### `__iter__() -> Iterator`

List iterator

**Remarks:** List iterator

**Returns:** `Iterator` — List iterator

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Utility/VecULongList/#NemAll_Python_Utility.VecULongList.__iter__)

#### `__len__() -> int`

Get the list length

**Remarks:** Get the list length

**Returns:** `int` — Length of the list

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Utility/VecULongList/#NemAll_Python_Utility.VecULongList.__len__)

#### `__repr__() -> str`

Create a string from the values

**Remarks:** Create a string from the values

**Returns:** `str` — String

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Utility/VecULongList/#NemAll_Python_Utility.VecULongList.__repr__)

#### `__setitem__(index: int | slice, value: int)`

Set a list item

**Remarks:** Set a list item

**Parameters:**
- `index` (int | slice) — Index of the item
- `value` (int) — Value to item

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Utility/VecULongList/#NemAll_Python_Utility.VecULongList.__setitem__)

#### `append(value: int)`

Append a list item

**Remarks:** Append a list item

**Parameters:**
- `value` (int) — Value to append

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Utility/VecULongList/#NemAll_Python_Utility.VecULongList.append)

#### `extend(iterable: VecULongList)`

Add the items from an iterable to the end of the list

**Remarks:** Add the items from an iterable to the end of the list

**Parameters:**
- `iterable` (VecULongList) — Iterable to add

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Utility/VecULongList/#NemAll_Python_Utility.VecULongList.extend)

## VecUShortList (class)

(No description provided in vendor docs for NemAll_Python_Utility.VecUShortList.)

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Utility/VecUShortList/)

### Constructors
- `VecUShortList() | VecUShortList(valueList: list)` — Initialize

### Methods
#### `__contains__(value: int) -> bool`

Check for a value in the list

**Remarks:** Check for a value in the list

**Parameters:**
- `value` (int) — Value to check

**Returns:** `bool` — State for value is in the list

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Utility/VecUShortList/#NemAll_Python_Utility.VecUShortList.__contains__)

#### `__delitem__(value: int)`

Delete a list item

**Remarks:** Delete a list item

**Parameters:**
- `value` (int) — Value to delete

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Utility/VecUShortList/#NemAll_Python_Utility.VecUShortList.__delitem__)

#### `__eq__(compare_list: VecUShortList) -> bool`

Compare two list

**Remarks:** Compare two list

**Parameters:**
- `compare_list` (VecUShortList) — List to compare

**Returns:** `bool` — Lists are equal state

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Utility/VecUShortList/#NemAll_Python_Utility.VecUShortList.__eq__)

#### `__getitem__(index: int) -> int`

Get a list item

**Remarks:** Get a list item

**Parameters:**
- `index` (int) — Index of the item

**Returns:** `int` — Value for the index

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Utility/VecUShortList/#NemAll_Python_Utility.VecUShortList.__getitem__)

#### `__iter__() -> Iterator`

List iterator

**Remarks:** List iterator

**Returns:** `Iterator` — List iterator

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Utility/VecUShortList/#NemAll_Python_Utility.VecUShortList.__iter__)

#### `__len__() -> int`

Get the list length

**Remarks:** Get the list length

**Returns:** `int` — Length of the list

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Utility/VecUShortList/#NemAll_Python_Utility.VecUShortList.__len__)

#### `__repr__() -> str`

Create a string from the values

**Remarks:** Create a string from the values

**Returns:** `str` — String

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Utility/VecUShortList/#NemAll_Python_Utility.VecUShortList.__repr__)

#### `__setitem__(index: int | slice, value: int)`

Set a list item

**Remarks:** Set a list item

**Parameters:**
- `index` (int | slice) — Index of the item
- `value` (int) — Value to item

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Utility/VecUShortList/#NemAll_Python_Utility.VecUShortList.__setitem__)

#### `append(value: int)`

Append a list item

**Remarks:** Append a list item

**Parameters:**
- `value` (int) — Value to append

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Utility/VecUShortList/#NemAll_Python_Utility.VecUShortList.append)

#### `extend(iterable: VecUShortList)`

Add the items from an iterable to the end of the list

**Remarks:** Add the items from an iterable to the end of the list

**Parameters:**
- `iterable` (VecUShortList) — Iterable to add

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Utility/VecUShortList/#NemAll_Python_Utility.VecUShortList.extend)

## wstring (class)

(No description provided in vendor docs for NemAll_Python_Utility.wstring.)

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Utility/wstring/)

### Constructors
- `wstring()` — Initialize

### Methods
#### `__repr__() -> str`

Convert the list to a string

**Remarks:** Convert the list to a string

**Returns:** `str` — List values as string

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_Utility/wstring/#NemAll_Python_Utility.wstring.__repr__)

