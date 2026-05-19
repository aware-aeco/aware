---
name: tekla-tekla-structures-dialog-uicontrols
description: This skill encodes the tekla 2025.0 surface of the Tekla.Structures.Dialog.UIControls namespace — 52 types with full vendor-documented methods, properties, events, and behavioral remarks. Read when composing apps that touch any of: BindableRadioButton, BoltCatalogSize, BoltCatalogStandard, CommitAction, ComponentCatalog, CreateApplyCancel, CreateDialog, CustomObservableCollection<T>, and 44 more types.
---

# Tekla.Structures.Dialog.UIControls

Auto-generated from vendor docs for tekla 2025.0. 52 types in this namespace.

## BindableRadioButton (class)

The BindableRadioButton class represents a RadioButton control that can be bound to the dialog attributes file. Use "Checked" as the BindPropertyName and "Integer" as the AttributeTypeName.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/831807d7-dee9-aa94-5158-91599354ddbe)

### Constructors
- `public BindableRadioButton()` — Initializes a new instance of the BindableRadioButton class

### Properties
- `Checked` (Boolean, get/set) — Gets or sets a value indicating whether the control is checked.

## BoltCatalogSize (class)

The BoltCatalogSize class represents a control to select the bolt size using the bolt catalog. Bolt catalog controls always need to be in pairs, meaning in every dialog there has to be a BoltCatalogStandard and a BoltCatalogSize control. The property BoltCatalogStandard.LinkedBoltCatalogSize sets to which BoltCatalogSize the control is linked.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/fbd028e7-0e46-5602-c31c-8fab18e862f8)

### Constructors
- `public BoltCatalogSize()` — Initiates a new instance of the control.

## BoltCatalogStandard (class)

The BoltCatalogStandard class represents a control to select the bolt standard using the bolt catalog. Bolt catalog controls always need to be in pairs, meaning in every dialog there has to be a BoltCatalogStandard and a BoltCatalogSize control. The property BoltCatalogStandard.LinkedBoltCatalogSize sets to which BoltCatalogSize the control is linked.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/9ff0b8bf-2f0f-4301-4868-2d8fb0795f68)

### Constructors
- `public BoltCatalogStandard()` — Initiates a new instance of the control.

### Properties
- `LinkedBoltCatalogSize` (BoltCatalogSize, get/set) — The BoltCatalogSize control linked to the BoltCatalogStandard control. Bolt catalog controls always need to be in pairs, meaning in every dialog there has to be a BoltCatalogStandard and a BoltCatalogSize control.

## CommitAction (class)

The CommitAction class represents a "template" dialog for commit actions.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/7b1782c1-250b-0de1-71e8-d99f462cb221)

### Constructors
- `public CommitAction()` — Initializes a new instance of the CommitAction class.

### Methods
#### `public void ApplyValues(string FileName)`

Loads the dialog values from a file and performs Apply() on the loaded values. To match the files to a certain dialog, the file suffix is set as the dialog type's name.

**Parameters:**
- `FileName` (System.String) — The name of the file.

[Docs](https://developer.tekla.com/topic/en/18/43/c4096cdf-60d9-f7b8-1daf-e71de20cf47d)

#### `public bool GetConnectionStatus()`

Returns true if a proper connection to the Tekla Structures process has been established. Currently, there's no way to re-establish the connection.

**Returns:** `Boolean` — True on success.

[Docs](https://developer.tekla.com/topic/en/18/43/2ba6677a-b8a7-cbfa-2724-e58464054abe)

#### `public void InitializeForm()`

Prepares the data storage for the dialog and scans through the fields.

[Docs](https://developer.tekla.com/topic/en/18/43/70881deb-ea45-96c7-ee73-399bc7b76a90)

#### `public void LoadValues(string FileName)`

Loads the dialog values from a file. To match the files to a certain dialog, the file suffix is set as the dialog type's name.

**Parameters:**
- `FileName` (System.String) — The name of the file.

[Docs](https://developer.tekla.com/topic/en/18/43/df11f56f-8372-2886-e011-b4fcad4ac3de)

#### `public void ModifyValues(string FileName)`

Loads the dialog values from a file and performs Modify() on the loaded values. To match the files to a certain dialog, the file suffix is set as the dialog type's name.

**Parameters:**
- `FileName` (System.String) — The name of the file.

[Docs](https://developer.tekla.com/topic/en/18/43/d20a7c4e-e913-92e5-0925-44015cc68ebf)

#### `public void SaveValues(string fileName)`

Serializes the dialog values to an xml file.

**Parameters:**
- `fileName` (System.String) — The path of the file.

[Docs](https://developer.tekla.com/topic/en/18/43/a817a258-86a2-8616-be00-eda92c7e5f8e)

#### `public void SetAttributeValue(Control Ctrl, Object Value)`

Sets a value for the given control. When the dialog is not shown, setting a property directly for a control (such as textBox1.Text = "text") will not work for controls that have a Tekla Structures AttributeTypeName set. This method is going to have to be used to set the value.

**Parameters:**
- `Ctrl` (System.Windows.Forms.Control) — The control whose value to set.
- `Value` (System.Object) — The new value. It can be of any primitive datatype supported by the control's AttributeType (normally int, double or string).

[Docs](https://developer.tekla.com/topic/en/18/43/30e56a80-1c86-4606-781c-2a91b7bb8884)

#### `public void ShowForm()`

Displays the form.

[Docs](https://developer.tekla.com/topic/en/18/43/79258182-6491-e7e6-99d0-de2072cb85a5)

#### `[ObsoleteAttribute("Does currently nothing. Will be removed in near future.")] public void UpdateValues()`

Rereads and updates all the field values on the form.

[Docs](https://developer.tekla.com/topic/en/18/43/5ab0c9d1-4ddf-1e05-29ca-560e387350e5)

### Properties
- `Localization` (Localization, get) — The localization instance for the dialog. Each dialog has its own localization instance that has read the localization files needed for that dialog.

## ComponentCatalog (class)

The ComponentCatalog class represents a group of controls to select components using the component catalog.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/5b4207d7-2a3c-f34b-9cda-65386cf1f092)

### Constructors
- `public ComponentCatalog()` — Initializes a new instance of the ComponentCatalog class.

### Properties
- `SelectedComponentType` (ComponentItem.ComponentTypeEnum, get/set) — Gets or sets the selected component type in the control.
- `SelectedName` (String, get/set) — Gets or sets the selected component name in the control.
- `SelectedNumber` (Int32, get/set) — Gets or sets the selected component number in the control.

### Events
#### `SelectClicked` (`System.EventHandler`)

**Signature:** `public event EventHandler SelectClicked`

The SelectClicked event is raised when the Select button is clicked.

[Docs](https://developer.tekla.com/topic/en/18/43/b2e533a4-f75c-fe43-0d2c-b47f74f7c209)

#### `SelectionDone` (`System.EventHandler`)

**Signature:** `public event EventHandler SelectionDone`

The SelectionDone event is raised when the selection has been done.

[Docs](https://developer.tekla.com/topic/en/18/43/83b4d55d-cef1-631e-0026-00bddd897eac)

## CreateApplyCancel (class)

The CreateApplyCancel class represents a control including the Create-Apply-Cancel button group.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/592d3a1e-eee3-33c3-4afe-afcb196d6207)

### Constructors
- `public CreateApplyCancel()` — Initializes CreateApplyCancel user control.

### Events
#### `ApplyClicked` (`System.EventHandler`)

**Signature:** `public event EventHandler ApplyClicked`

The ApplyClicked event is raised when the Apply button is clicked.

[Docs](https://developer.tekla.com/topic/en/18/43/a87487ad-c241-be07-4e9c-8d4eb154e757)

#### `CancelClicked` (`System.EventHandler`)

**Signature:** `public event EventHandler CancelClicked`

The CancelClicked event is raised when the Cancel button is clicked.

[Docs](https://developer.tekla.com/topic/en/18/43/eee2017b-b034-450a-2dc4-3aab9beebd27)

#### `CreateClicked` (`System.EventHandler`)

**Signature:** `public event EventHandler CreateClicked`

The CreateClicked event is raised when the Create button is clicked.

[Docs](https://developer.tekla.com/topic/en/18/43/c47f141d-9872-e0e8-466b-7adea15be32f)

## CreateDialog (class)

The CreateDialog class represents a "template" dialog for creating something from parts.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/a91c61e7-953b-a78d-eea8-dad3125bcf16)

### Constructors
- `public CreateDialog()` — Initializes a new instance of the CreateDialog class.

### Methods
#### `public void ApplyValues(string FileName)`

Loads the dialog values from a file and performs Apply() on the loaded values. To match the files to a certain dialog, the file suffix is set as the dialog type's name.

**Parameters:**
- `FileName` (System.String) — The name of the file.

[Docs](https://developer.tekla.com/topic/en/18/43/c4096cdf-60d9-f7b8-1daf-e71de20cf47d)

#### `public bool GetConnectionStatus()`

Returns true if a proper connection to the Tekla Structures process has been established. Currently, there's no way to re-establish the connection.

**Returns:** `Boolean` — True on success.

[Docs](https://developer.tekla.com/topic/en/18/43/2ba6677a-b8a7-cbfa-2724-e58464054abe)

#### `public void InitializeForm()`

Prepares the data storage for the dialog and scans through the fields.

[Docs](https://developer.tekla.com/topic/en/18/43/70881deb-ea45-96c7-ee73-399bc7b76a90)

#### `public void LoadValues(string FileName)`

Loads the dialog values from a file. To match the files to a certain dialog, the file suffix is set as the dialog type's name.

**Parameters:**
- `FileName` (System.String) — The name of the file.

[Docs](https://developer.tekla.com/topic/en/18/43/df11f56f-8372-2886-e011-b4fcad4ac3de)

#### `public void ModifyValues(string FileName)`

Loads the dialog values from a file and performs Modify() on the loaded values. To match the files to a certain dialog, the file suffix is set as the dialog type's name.

**Parameters:**
- `FileName` (System.String) — The name of the file.

[Docs](https://developer.tekla.com/topic/en/18/43/d20a7c4e-e913-92e5-0925-44015cc68ebf)

#### `public void SaveValues(string fileName)`

Serializes the dialog values to an xml file.

**Parameters:**
- `fileName` (System.String) — The path of the file.

[Docs](https://developer.tekla.com/topic/en/18/43/a817a258-86a2-8616-be00-eda92c7e5f8e)

#### `public void SetAttributeValue(Control Ctrl, Object Value)`

Sets a value for the given control. When the dialog is not shown, setting a property directly for a control (such as textBox1.Text = "text") will not work for controls that have a Tekla Structures AttributeTypeName set. This method is going to have to be used to set the value.

**Parameters:**
- `Ctrl` (System.Windows.Forms.Control) — The control whose value to set.
- `Value` (System.Object) — The new value. It can be of any primitive datatype supported by the control's AttributeType (normally int, double or string).

[Docs](https://developer.tekla.com/topic/en/18/43/30e56a80-1c86-4606-781c-2a91b7bb8884)

#### `public void ShowForm()`

Displays the form.

[Docs](https://developer.tekla.com/topic/en/18/43/79258182-6491-e7e6-99d0-de2072cb85a5)

#### `[ObsoleteAttribute("Does currently nothing. Will be removed in near future.")] public void UpdateValues()`

Rereads and updates all the field values on the form.

[Docs](https://developer.tekla.com/topic/en/18/43/5ab0c9d1-4ddf-1e05-29ca-560e387350e5)

### Properties
- `Localization` (Localization, get) — The localization instance for the dialog. Each dialog has its own localization instance that has read the localization files needed for that dialog.

## CustomObservableCollection<T> (class)

CustomObservableCollection for synchronizing UI controls.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/4ca922b0-cea2-2be2-ca5b-2dd603c8c10e)

### Constructors
- `public CustomObservableCollection()` — CustomObservableCollection constructor.
- `public CustomObservableCollection(IEnumerable<T> collection)` — CustomObservableCollection constructor.
- `public CustomObservableCollection(List<T> list)` — CustomObservableCollection constructor.

### Methods
#### `public void Repopulate(IEnumerable<T> collection)`

CustomObservableCollection repopulator, sends notification about property and collection change.

**Parameters:**
- `collection` (System.Collections.Generic.IEnumerable<T>) — The collection used in repopulation.

[Docs](https://developer.tekla.com/topic/en/18/43/9c83ef1c-6b7b-1aa3-da5b-9fdd9a28309a)

## DataGrid (class)

The DataGrid class represents a data grid control that can contain images.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/15327883-bd73-f2c5-ce5d-79cb2ef01153)

### Constructors
- `public DataGrid()` — Initializes a new instance of the DataGrid class.

## EnvironmentFiles (class)

The EnvironmentFiles class is for the paths where the attributes file will be searched for.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/827d1e0c-fca9-2053-0228-224c20fe1f9f)

### Constructors
- `public EnvironmentFiles()` — Initializes a new instance of the EnvironmentFiles class

### Methods
#### `[ObsoleteAttribute("Deprecated. Please use GetAttributeFile(string fileName) instead.", false)] public static FileInfo GetAttributeFile(List<string> searchDirectories, string fileName)`

Gets a file info representing the first match in the search directories.

**Parameters:**
- `searchDirectories` (System.Collections.Generic.List<String>) — The list of directories to be used for searching for the file.
- `fileName` (System.String) — The name of the file including the file extension.

**Returns:** `FileInfo` — A file info for the first match in the directory list. Null if no match was found.

[Docs](https://developer.tekla.com/topic/en/18/43/1d543ea0-e1bd-f108-64cf-c057005203fa)

#### `public static FileInfo GetAttributeFile(string fileName)`

Gets a file info representing the first match in the standard property file directories.

**Parameters:**
- `fileName` (System.String) — The name of the file including the file extension.

**Returns:** `FileInfo` — A file info for the first match in the directory list. Null if no match was found.

[Docs](https://developer.tekla.com/topic/en/18/43/25184c95-e965-0260-3b9f-5000f8b11a1f)

#### `public static List<string> GetAttributeFiles(string suffix)`

Gets attribute files with specified suffix from model, XS_FIRM and XS_PROJECT directories and subdirectories.

**Parameters:**
- `suffix` (System.String) — The filename suffix.

**Returns:** `List<String>` — A list of found file names

[Docs](https://developer.tekla.com/topic/en/18/43/8989c740-9207-24ea-2551-74bffdd718d7)

#### `public static List<string> GetMultiDirectoryFileList(List<string> searchDirectories, string fileExtension)`

Gets a list of files with the given extension from the given search directories and from XS_FIRM, XS_PROJECT subdirectories.

**Parameters:**
- `searchDirectories` (System.Collections.Generic.List<String>) — The search directories to be used.
- `fileExtension` (System.String) — The file extension to be used.

**Returns:** `List<String>` — A list of found file names without the file extension.

[Docs](https://developer.tekla.com/topic/en/18/43/2b3a348a-bf7d-942b-0353-ba9d32e66ab6)

#### `public static List<string> GetMultiDirectoryFileList(string fileExtension)`

Gets a list of files with the given extension from the default search directories.

**Parameters:**
- `fileExtension` (System.String) — The file extension to be used.

**Returns:** `List<String>` — A list of files with the given extension.

[Docs](https://developer.tekla.com/topic/en/18/43/c369ab72-1076-53f7-b462-df1a718863de)

#### `public static List<string> GetStandardPropertyFileDirectories()`

Gets the paths where to look for the property files.

**Returns:** `List<String>` — The paths where to look for the property files.

[Docs](https://developer.tekla.com/topic/en/18/43/1d1abc86-3590-1bb6-65ac-c97febcbf03a)

#### `public static bool IsValidDirectory(string directory)`

Checks if a directory is valid.

**Parameters:**
- `directory` (System.String) — The directory to be checked.

**Returns:** `Boolean` — True if the directory is valid.

[Docs](https://developer.tekla.com/topic/en/18/43/0b445871-456d-31a8-68c7-40832b64cc7e)

### Properties
- `PropertyFileDirectories` (List<String>, get/set) — The directories where to look for property files.

## EnvironmentVariables (class)

The EnvironmentVariables class contains a sorted list specializing in getting active environment variables and advanced option settings. It also checks options.ini files in the active model folder as well as options_user.ini files.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/55ce6899-663c-4581-14ea-228425de9a97)

### Methods
#### `public static void Add(string key)`

Adds a variable to the list.

**Parameters:**
- `key` (System.String) — The variable to be added.

[Docs](https://developer.tekla.com/topic/en/18/43/c7d998b9-ddb7-e742-a3e1-7fe8ecfb949d)

#### `public static string Get(string key)`

Gets a key.

**Parameters:**
- `key` (System.String) — The key to get.

**Returns:** `String` — The key.

[Docs](https://developer.tekla.com/topic/en/18/43/002ca3fd-b561-354f-b73a-0422e9d79c18)

#### `public static string GetEnvironmentVariable(string variableName)`

Gets an environment variable.

**Parameters:**
- `variableName` (System.String) — The name of the variable to get.

**Returns:** `String` — The environment variable.

[Docs](https://developer.tekla.com/topic/en/18/43/38ea5140-b4be-74f3-f298-1270d3588221)

## ImageComboBox (class)

The ImageComboBox class represents a combo box control that can contain images.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/6a9566df-5ab1-19ad-b407-c587eee6436d)

### Constructors
- `public ImageComboBox()` — Initializes a new instance of the ImageComboBox class.

### Properties
- `DefaultValue` (String, get/set) — Gets or sets the default value in the image combo box.
- `OptionList` (ImageList, get/set) — Gets or sets the elements of the option list, the images that are in the image combo box.
- `SelectedIndex` (Int32, get/set) — Gets or sets the selected index of the image in the image combo box.
- `SelectedItem` (Object, get/set) — Gets or sets the selected image in the image combo box.

### Events
#### `ImageCBSelectedIndexChanged` (`System.EventHandler`)

**Signature:** `public event EventHandler ImageCBSelectedIndexChanged`

The ImageCBSelectedIndexChanged event is triggered just after the selected index has been changed in the image combo box.

[Docs](https://developer.tekla.com/topic/en/18/43/8c110efc-be84-b175-0b28-ae8256dd6ed7)

## ImageItem (class)

The ImageItem class defines the images that will be contained in the image combo box.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/dfcffcee-b109-8148-dc82-1b9292dd4f92)

### Properties
- `Name` (String, get/set) — Gets or sets the image name.
- `Picture` (Image, get/set) — Gets or sets the image.

## ImageList (class)

The ImageList class contains a list of ImageItems.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/f4a5efe1-273a-dfc1-f835-f417cf772dde)

### Constructors
- `public ImageList()` — Initializes a new instance of the ImageList class

## ImageListComboBox (class)

The ImageListComboBox class represents a combo box control that can contain images from an ImageList control.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/1601578c-61e1-7a7f-e39e-d4bd3927d844)

### Constructors
- `public ImageListComboBox()` — Initializes a new instance of the ImageListComboBox class.

### Methods
#### `public void RefreshOptionList()`

Refreshes the option list content.

[Docs](https://developer.tekla.com/topic/en/18/43/c78dae68-5e4f-7944-27a9-eb944ef62b38)

### Properties
- `DefaultValue` (String, get/set) — Gets or sets the default value in the image combo box. If the value is not found, SelectedItem is used instead.
- `HoverColor` (Color, get/set) — Gets or sets the hover color of the combo box.
- `ImageList` (ImageList, get/set) — Gets or sets the ImageList that contains the images to be displayed in the combo box. Recommended image file formats are JPG and BMP.
- `SelectedIndex` (Int32, get/set) — Gets or sets the selected index of the image in the image combo box.
- `SelectedItem` (Object, get/set) — Gets or sets the selected image in the image combo box. If the image is not found, SelectedIndex is used instead.
- `ToolTipText` (String, get/set) — Gets or sets the tool tip text.

### Events
#### `ImageListComboBoxMouseWheel` (`System.Windows.Forms.MouseEventHandler`)

**Signature:** `public event MouseEventHandler ImageListComboBoxMouseWheel`

The ImageListComboBoxMouseWheel event is triggered just after the mouse wheel has been activated on top of the combo box.

[Docs](https://developer.tekla.com/topic/en/18/43/8a816d16-75a8-7478-e598-046ea2501746)

#### `ImageListComboBoxSelectedIndexChanged` (`System.EventHandler`)

**Signature:** `public event EventHandler ImageListComboBoxSelectedIndexChanged`

The ImageListComboBoxSelectedIndexChanged event is triggered just after the selected index has been changed in the image combo box.

[Docs](https://developer.tekla.com/topic/en/18/43/8de9966c-3515-eafb-9e11-cdf53b7029e8)

## LoadingForm (class)

The LoadingForm class creates a dialog that is shown while something is being loaded by the main window and that needs the process to finish.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/4efab7b6-d829-72d2-48d7-abdcc7c3ac0c)

### Constructors
- `public LoadingForm()` — Creates a new instance of the form.

### Properties
- `AllowClosing` (Boolean, get/set) — Allows the closing of the dialog. By default disabled.

## LocalizeForm (class)

The LocalizeForm class is for localizing the forms.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/2bc9b34b-378c-79b3-513c-c0941222b31a)

### Properties
- `Localization` (Localization, get) — Gets the localization for the dialog.

## MaterialCatalog (class)

The MaterialCatalog class represents a group of controls to select materials using the material catalog.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/18f14693-f93b-dd0c-d5ae-a997113bfd06)

### Constructors
- `public MaterialCatalog()` — Initiates a new instance of the control.
- `public MaterialCatalog(string Material)` — Creates a new instance of the MaterialCatalog and will select the material (if available) in the tree.

### Properties
- `ButtonText` (String, get/set) — Gets and sets the text of the button control.
- `SelectedMaterial` (String, get/set) — Gets the selected material in the control.

### Events
#### `SelectClicked` (`System.EventHandler`)

**Signature:** `public event EventHandler SelectClicked`

The SelectClicked event is raised when the Select button is clicked.

[Docs](https://developer.tekla.com/topic/en/18/43/788d5c1d-dfc3-1b3f-f849-9e96756dbb4d)

#### `SelectionDone` (`System.EventHandler`)

**Signature:** `public event EventHandler SelectionDone`

The SelectionDone event is raised when the selection has been done.

[Docs](https://developer.tekla.com/topic/en/18/43/8e5da3b1-7d90-73eb-3806-28e4d4c34003)

## MaterialSelectionForm (class)

The MaterialSelectionForm class represents a dialog to select materials using the material catalogs.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/c171b4db-6ad7-65a7-5659-3d3a409427f7)

### Constructors
- `public MaterialSelectionForm(List<MaterialItem> Materials)` — Creates a new instance of the MaterialSelectionForm.
- `public MaterialSelectionForm(List<MaterialItem> Materials, string Material)` — Creates a new instance of the MaterialSelectionForm, selecting the given material in the tree control.

### Properties
- `SelectedMaterial` (String, get/set) — The selected material in the control.

## MeshCatalog (class)

The MeshCatalog class represents a group of controls to select meshes using the mesh catalogs.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/ccb3ba76-9a72-37f5-5721-b1638bcd9cec)

### Constructors
- `public MeshCatalog()` — Initiates a new instance of the control.

### Properties
- `ButtonText` (String, get/set) — Gets and sets the text of the button control.
- `SelectedMeshGrade` (String, get/set) — The mesh grade in the control.
- `SelectedMeshName` (String, get/set) — The mesh name in the control.

### Events
#### `SelectClicked` (`System.EventHandler`)

**Signature:** `public event EventHandler SelectClicked`

The SelectClicked event is raised when the Select button is clicked.

[Docs](https://developer.tekla.com/topic/en/18/43/19fa75ac-8361-397c-f0ab-2d6f3ffb5540)

#### `SelectionDone` (`System.EventHandler`)

**Signature:** `public event EventHandler SelectionDone`

The SelectionDone event is raised when the selection has been done.

[Docs](https://developer.tekla.com/topic/en/18/43/1ac3f345-d39f-9fd2-4d18-8343990a198e)

## MeshSelectionForm (class)

The MeshSelectionForm class represents a dialog to select meshes using the mesh catalog.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/9a8d2cea-1d80-e03f-a289-d09bb95ca23b)

### Constructors
- `public MeshSelectionForm(List<MeshItem> MeshList)` — Creates a new instance of the MeshSelectionForm.
- `public MeshSelectionForm(List<MeshItem> MeshList, string StartingMesh, string StartingGrade)` — Creates a new instance of the MeshSelectionForm and will select in the tree the mesh that contains the given parameters (if possible).

### Methods
#### `public void ApplyValues(string FileName)`

Loads the dialog values from a file and performs Apply() on the loaded values. To match the files to a certain dialog, the file suffix is set as the dialog type's name.

**Parameters:**
- `FileName` (System.String) — The name of the file.

[Docs](https://developer.tekla.com/topic/en/18/43/c4096cdf-60d9-f7b8-1daf-e71de20cf47d)

#### `public bool GetConnectionStatus()`

Returns true if a proper connection to the Tekla Structures process has been established. Currently, there's no way to re-establish the connection.

**Returns:** `Boolean` — True on success.

[Docs](https://developer.tekla.com/topic/en/18/43/2ba6677a-b8a7-cbfa-2724-e58464054abe)

#### `public void InitializeForm()`

Prepares the data storage for the dialog and scans through the fields.

[Docs](https://developer.tekla.com/topic/en/18/43/70881deb-ea45-96c7-ee73-399bc7b76a90)

#### `public void LoadValues(string FileName)`

Loads the dialog values from a file. To match the files to a certain dialog, the file suffix is set as the dialog type's name.

**Parameters:**
- `FileName` (System.String) — The name of the file.

[Docs](https://developer.tekla.com/topic/en/18/43/df11f56f-8372-2886-e011-b4fcad4ac3de)

#### `public void ModifyValues(string FileName)`

Loads the dialog values from a file and performs Modify() on the loaded values. To match the files to a certain dialog, the file suffix is set as the dialog type's name.

**Parameters:**
- `FileName` (System.String) — The name of the file.

[Docs](https://developer.tekla.com/topic/en/18/43/d20a7c4e-e913-92e5-0925-44015cc68ebf)

#### `public void SaveValues(string fileName)`

Serializes the dialog values to an xml file.

**Parameters:**
- `fileName` (System.String) — The path of the file.

[Docs](https://developer.tekla.com/topic/en/18/43/a817a258-86a2-8616-be00-eda92c7e5f8e)

#### `public void SetAttributeValue(Control Ctrl, Object Value)`

Sets a value for the given control. When the dialog is not shown, setting a property directly for a control (such as textBox1.Text = "text") will not work for controls that have a Tekla Structures AttributeTypeName set. This method is going to have to be used to set the value.

**Parameters:**
- `Ctrl` (System.Windows.Forms.Control) — The control whose value to set.
- `Value` (System.Object) — The new value. It can be of any primitive datatype supported by the control's AttributeType (normally int, double or string).

[Docs](https://developer.tekla.com/topic/en/18/43/30e56a80-1c86-4606-781c-2a91b7bb8884)

#### `public void ShowForm()`

Displays the form.

[Docs](https://developer.tekla.com/topic/en/18/43/79258182-6491-e7e6-99d0-de2072cb85a5)

#### `[ObsoleteAttribute("Does currently nothing. Will be removed in near future.")] public void UpdateValues()`

Rereads and updates all the field values on the form.

[Docs](https://developer.tekla.com/topic/en/18/43/5ab0c9d1-4ddf-1e05-29ca-560e387350e5)

### Properties
- `Localization` (Localization, get) — The localization instance for the dialog. Each dialog has its own localization instance that has read the localization files needed for that dialog.
- `SelectedMeshGrade` (String, get/set) — The selected mesh grade.
- `SelectedMeshName` (String, get/set) — The selected mesh name.

## ModelAccess (class)

The ModelAccess class contains helper methods for connecting to and accessing the model and objects in the model. The class attempts to provide efficient but robust methods for connecting to and verifying the connection to the model.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/e5d4d0c9-e09a-5356-0836-2dc8e0ea5295)

### Constructors
- `public ModelAccess()` — Initializes a new instance of the ModelAccess class

### Methods
#### `public static Model ConnectToModel()`

Gets a model connnection.

**Returns:** `Model` — The model or null if unable to connect.

[Docs](https://developer.tekla.com/topic/en/18/43/112f2b0f-baa5-60ee-0a52-93a9cfaf4c93)

#### `public static Model ConnectToModel(out bool ConnectedToModel)`

Gets a model connection.

**Parameters:**
- `ConnectedToModel` (System.Boolean.) — True if a model connection was made. False otherwise.

**Returns:** `Model` — The model or null if unable to connect.

[Docs](https://developer.tekla.com/topic/en/18/43/e07e7c5c-a968-97d5-b631-babec211bc26)

#### `public static bool ConnectToModel(out Model model)`

Gets a model connection.

**Parameters:**
- `model` (Tekla.Structures.Model.Model.) — The model connection.

**Returns:** `Boolean` — True on success. False otherwise.

[Docs](https://developer.tekla.com/topic/en/18/43/a013c153-5759-8c07-4ca8-d60053b82bc6)

## OkApplyCancel (class)

The OkApplyCancel class represents a control including the Ok-Apply-Cancel button group.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/475b9797-ba28-f3ef-330d-357a17a3bcb0)

### Constructors
- `public OkApplyCancel()` — Initializes a new instance of the OkApplyCancel class.

### Events
#### `ApplyClicked` (`System.EventHandler`)

**Signature:** `public event EventHandler ApplyClicked`

The ApplyClicked event is raised when the Apply button is clicked.

[Docs](https://developer.tekla.com/topic/en/18/43/95713516-c251-7fd4-e346-741aaa0ebcf2)

#### `CancelClicked` (`System.EventHandler`)

**Signature:** `public event EventHandler CancelClicked`

The CancelClicked event is raised when the Cancel button is clicked.

[Docs](https://developer.tekla.com/topic/en/18/43/8548e233-6926-2390-0617-d4d94540a296)

#### `OkClicked` (`System.EventHandler`)

**Signature:** `public event EventHandler OkClicked`

The OkClicked event is raised when the Ok button is clicked.

[Docs](https://developer.tekla.com/topic/en/18/43/4c478d7d-4ea4-2fef-d438-e0e29bb07ea4)

## OkApplyModifyGetOnOffCancel (class)

The OkApplyModifyGetOnOffCancel class represents a control including the Ok-Apply-Modify-Get-On/Off-Cancel button group.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/3fa23eb1-2741-9c1d-c85a-d6f1d4fb1d2d)

### Constructors
- `public OkApplyModifyGetOnOffCancel()` — Initializes a new instance of the OkApplyModifyGetOnOffCancel class.

### Events
#### `ApplyClicked` (`System.EventHandler`)

**Signature:** `public event EventHandler ApplyClicked`

The ApplyClicked event is raised when the Apply button is clicked.

[Docs](https://developer.tekla.com/topic/en/18/43/ae6b798c-8803-e71f-3de1-47785ae8a1c9)

#### `CancelClicked` (`System.EventHandler`)

**Signature:** `public event EventHandler CancelClicked`

The CancelClicked event is raised when the Cancel button is clicked.

[Docs](https://developer.tekla.com/topic/en/18/43/9e1c49a8-987b-0423-b45b-dd812d26e4a9)

#### `GetClicked` (`System.EventHandler`)

**Signature:** `public event EventHandler GetClicked`

The GetClicked event is raised when the Get button is clicked.

[Docs](https://developer.tekla.com/topic/en/18/43/08cd6705-d157-cea5-3ecb-e7d92eaa98dd)

#### `ModifyClicked` (`System.EventHandler`)

**Signature:** `public event EventHandler ModifyClicked`

The ModifyClicked event is raised when the Modify button is clicked.

[Docs](https://developer.tekla.com/topic/en/18/43/88e52e71-d36c-efb4-4b36-b0cb266b7e48)

#### `OkClicked` (`System.EventHandler`)

**Signature:** `public event EventHandler OkClicked`

The OkClicked event is raised when the Ok button is clicked.

[Docs](https://developer.tekla.com/topic/en/18/43/1f905ac4-a3a0-f857-7973-9ca309152d37)

#### `OnOffClicked` (`System.EventHandler`)

**Signature:** `public event EventHandler OnOffClicked`

The OnOffClicked event is raised when the On-Off button is clicked.

[Docs](https://developer.tekla.com/topic/en/18/43/c4e95ecd-3feb-8310-8ce4-3436321b9d9f)

## OkCancel (class)

The OkCancel class represents a control including the Ok-Cancel button group.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/df10d8bf-6b93-f3ed-2f63-1ed2b93eeb0a)

### Constructors
- `public OkCancel()` — Initializes a new instance of the OkCancel class.

### Events
#### `CancelClicked` (`System.EventHandler`)

**Signature:** `public event EventHandler CancelClicked`

The CancelClicked event is raised when the Cancel button is clicked.

[Docs](https://developer.tekla.com/topic/en/18/43/911b4651-6d95-f409-3d8b-84bf6cbeaf29)

#### `OkClicked` (`System.EventHandler`)

**Signature:** `public event EventHandler OkClicked`

The OkClicked event is raised when the Ok button is clicked.

[Docs](https://developer.tekla.com/topic/en/18/43/681bab33-f371-a1aa-b83f-b5aef379f674)

## OrganizerDialog (class)

The OrganizerDialog class represents a "template" of an organizer dialog.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/3345f5b7-07a3-8e44-98d5-7d8dc1251db4)

### Constructors
- `public OrganizerDialog()` — Initializes a new instance of the OrganizerDialog class.

### Methods
#### `public void ApplyValues(string FileName)`

Loads the dialog values from a file and performs Apply() on the loaded values. To match the files to a certain dialog, the file suffix is set as the dialog type's name.

**Parameters:**
- `FileName` (System.String) — The name of the file.

[Docs](https://developer.tekla.com/topic/en/18/43/c4096cdf-60d9-f7b8-1daf-e71de20cf47d)

#### `public bool GetConnectionStatus()`

Returns true if a proper connection to the Tekla Structures process has been established. Currently, there's no way to re-establish the connection.

**Returns:** `Boolean` — True on success.

[Docs](https://developer.tekla.com/topic/en/18/43/2ba6677a-b8a7-cbfa-2724-e58464054abe)

#### `public void InitializeForm()`

Prepares the data storage for the dialog and scans through the fields.

[Docs](https://developer.tekla.com/topic/en/18/43/70881deb-ea45-96c7-ee73-399bc7b76a90)

#### `public void LoadValues(string FileName)`

Loads the dialog values from a file. To match the files to a certain dialog, the file suffix is set as the dialog type's name.

**Parameters:**
- `FileName` (System.String) — The name of the file.

[Docs](https://developer.tekla.com/topic/en/18/43/df11f56f-8372-2886-e011-b4fcad4ac3de)

#### `public void ModifyValues(string FileName)`

Loads the dialog values from a file and performs Modify() on the loaded values. To match the files to a certain dialog, the file suffix is set as the dialog type's name.

**Parameters:**
- `FileName` (System.String) — The name of the file.

[Docs](https://developer.tekla.com/topic/en/18/43/d20a7c4e-e913-92e5-0925-44015cc68ebf)

#### `public void SaveValues(string fileName)`

Serializes the dialog values to an xml file.

**Parameters:**
- `fileName` (System.String) — The path of the file.

[Docs](https://developer.tekla.com/topic/en/18/43/a817a258-86a2-8616-be00-eda92c7e5f8e)

#### `public void SetAttributeValue(Control Ctrl, Object Value)`

Sets a value for the given control. When the dialog is not shown, setting a property directly for a control (such as textBox1.Text = "text") will not work for controls that have a Tekla Structures AttributeTypeName set. This method is going to have to be used to set the value.

**Parameters:**
- `Ctrl` (System.Windows.Forms.Control) — The control whose value to set.
- `Value` (System.Object) — The new value. It can be of any primitive datatype supported by the control's AttributeType (normally int, double or string).

[Docs](https://developer.tekla.com/topic/en/18/43/30e56a80-1c86-4606-781c-2a91b7bb8884)

#### `public void ShowForm()`

Displays the form.

[Docs](https://developer.tekla.com/topic/en/18/43/79258182-6491-e7e6-99d0-de2072cb85a5)

#### `[ObsoleteAttribute("Does currently nothing. Will be removed in near future.")] public void UpdateValues()`

Rereads and updates all the field values on the form.

[Docs](https://developer.tekla.com/topic/en/18/43/5ab0c9d1-4ddf-1e05-29ca-560e387350e5)

### Properties
- `Localization` (Localization, get) — The localization instance for the dialog. Each dialog has its own localization instance that has read the localization files needed for that dialog.

## ProfileCatalog (class)

The ProfileCatalog class represents a group of controls to select profiles using the profile catalogs.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/6f7b5108-d430-c386-35f7-f64f46760563)

### Constructors
- `public ProfileCatalog()` — Initiates a new instance of the control.
- `public ProfileCatalog(string Profile)` — Initiates a new instance of the control selecting the given profile.

### Methods
#### `public void OnValueChanged(Object data)`

Event handler for passing generic catalog data back to dialog

**Parameters:**
- `data` (System.Object) — value data

[Docs](https://developer.tekla.com/topic/en/18/43/4fd06031-82dd-daa8-6e41-51c76a364bbf)

### Properties
- `ButtonText` (String, get/set) — Gets and sets the text of the button control.
- `SelectedProfile` (String, get/set) — The selected profile in the control.

### Events
#### `SelectClicked` (`System.EventHandler`)

**Signature:** `public event EventHandler SelectClicked`

The SelectClicked event is raised when the Select button is clicked.

[Docs](https://developer.tekla.com/topic/en/18/43/c31c8f41-7652-bf90-88cb-540edd11ad15)

#### `SelectionDone` (`System.EventHandler`)

**Signature:** `public event EventHandler SelectionDone`

The SelectionDone event is raised when the profile selection dialog is closed.

[Docs](https://developer.tekla.com/topic/en/18/43/6683d11c-3def-2981-f246-75307aec6183)

## ProfileSelectionForm (class)

The ProfileSelectionForm class represents a dialog to select profiles using the profile catalogs.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/9e77696b-55d6-51e4-9949-efe253f3e5cd)

### Constructors
- `public ProfileSelectionForm(List<LibraryProfileItem> LibraryProfiles, List<ParametricProfileItem> ParametricProfiles)` — Creates a new instance of the ProfileSelectionForm.
- `public ProfileSelectionForm(List<LibraryProfileItem> LibraryProfileItems, List<ParametricProfileItem> ParametricProfileItems, string Profile)` — Creates a new instance of the ProfileSelectionForm selecting the given profile in the tree control.

### Properties
- `SelectedProfile` (String, get/set) — The selected profile.

## PropertiesDialog (class)

The PropertiesDialog class represents a "template" of a properties dialog.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/dedee88e-57fb-3204-92ef-2d4347dca161)

### Constructors
- `public PropertiesDialog()` — Initializes a new instance of the PropertiesDialog class.

### Methods
#### `public void ApplyValues(string FileName)`

Loads the dialog values from a file and performs Apply() on the loaded values. To match the files to a certain dialog, the file suffix is set as the dialog type's name.

**Parameters:**
- `FileName` (System.String) — The name of the file.

[Docs](https://developer.tekla.com/topic/en/18/43/c4096cdf-60d9-f7b8-1daf-e71de20cf47d)

#### `public bool GetConnectionStatus()`

Returns true if a proper connection to the Tekla Structures process has been established. Currently, there's no way to re-establish the connection.

**Returns:** `Boolean` — True on success.

[Docs](https://developer.tekla.com/topic/en/18/43/2ba6677a-b8a7-cbfa-2724-e58464054abe)

#### `public void InitializeForm()`

Prepares the data storage for the dialog and scans through the fields.

[Docs](https://developer.tekla.com/topic/en/18/43/70881deb-ea45-96c7-ee73-399bc7b76a90)

#### `public void LoadValues(string FileName)`

Loads the dialog values from a file. To match the files to a certain dialog, the file suffix is set as the dialog type's name.

**Parameters:**
- `FileName` (System.String) — The name of the file.

[Docs](https://developer.tekla.com/topic/en/18/43/df11f56f-8372-2886-e011-b4fcad4ac3de)

#### `public void ModifyValues(string FileName)`

Loads the dialog values from a file and performs Modify() on the loaded values. To match the files to a certain dialog, the file suffix is set as the dialog type's name.

**Parameters:**
- `FileName` (System.String) — The name of the file.

[Docs](https://developer.tekla.com/topic/en/18/43/d20a7c4e-e913-92e5-0925-44015cc68ebf)

#### `public void SaveValues(string fileName)`

Serializes the dialog values to an xml file.

**Parameters:**
- `fileName` (System.String) — The path of the file.

[Docs](https://developer.tekla.com/topic/en/18/43/a817a258-86a2-8616-be00-eda92c7e5f8e)

#### `public void SetAttributeValue(Control Ctrl, Object Value)`

Sets a value for the given control. When the dialog is not shown, setting a property directly for a control (such as textBox1.Text = "text") will not work for controls that have a Tekla Structures AttributeTypeName set. This method is going to have to be used to set the value.

**Parameters:**
- `Ctrl` (System.Windows.Forms.Control) — The control whose value to set.
- `Value` (System.Object) — The new value. It can be of any primitive datatype supported by the control's AttributeType (normally int, double or string).

[Docs](https://developer.tekla.com/topic/en/18/43/30e56a80-1c86-4606-781c-2a91b7bb8884)

#### `public void ShowForm()`

Displays the form.

[Docs](https://developer.tekla.com/topic/en/18/43/79258182-6491-e7e6-99d0-de2072cb85a5)

#### `[ObsoleteAttribute("Does currently nothing. Will be removed in near future.")] public void UpdateValues()`

Rereads and updates all the field values on the form.

[Docs](https://developer.tekla.com/topic/en/18/43/5ab0c9d1-4ddf-1e05-29ca-560e387350e5)

### Properties
- `Localization` (Localization, get) — The localization instance for the dialog. Each dialog has its own localization instance that has read the localization files needed for that dialog.

## ReinforcementCatalog (class)

The ReinforcementCatalog class represents a group of controls to select rebars using the rebar catalogs.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/ec000661-895b-f21e-e0f1-9ff859cdc495)

### Constructors
- `public ReinforcementCatalog()` — Initiates a new instance of the control.
- `public ReinforcementCatalog(string Size, string Grade, double BendRadius)` — Creates a new instance of the ReinforcementCatalog and will select in the tree the rebar that contains the given parameters (if possible).

### Methods
#### `public void OnValueChanged(Object data)`

Event handler for passing generic catalog data back to dialog

**Parameters:**
- `data` (System.Object) — value data

[Docs](https://developer.tekla.com/topic/en/18/43/c1c88c0f-3b3a-4715-2c9d-626037f46805)

### Properties
- `ButtonText` (String, get/set) — Gets and sets the text of the button control.
- `SelectedRebarBendingRadius` (Double, get/set) — Gets the selected rebar bending radius in the control.
- `SelectedRebarGrade` (String, get/set) — Gets the selected rebar grade in the control.
- `SelectedRebarSize` (String, get/set) — Gets the selected rebar size in the control.

### Events
#### `SelectClicked` (`System.EventHandler`)

**Signature:** `public event EventHandler SelectClicked`

The SelectClicked event is raised when the Select button is clicked.

[Docs](https://developer.tekla.com/topic/en/18/43/82470061-dee9-cc0f-339b-ba6348e19452)

#### `SelectionDone` (`System.EventHandler`)

**Signature:** `public event EventHandler SelectionDone`

The SelectionDone event is raised when the selection has been done.

[Docs](https://developer.tekla.com/topic/en/18/43/b2d01908-8051-5685-eee3-7a43fc5159d5)

## ReinforcementSelectionForm (class)

The ReinforcementSelectionForm class represents a dialog to select rebars using the rebar catalogs.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/cfd7cf88-b8f2-12d7-2d03-cae3147d052c)

### Constructors
- `public ReinforcementSelectionForm(List<RebarItem> RebarList)` — Creates a new instance of the ReinforcementSelectionForm.
- `public ReinforcementSelectionForm(List<RebarItem> RebarList, string Size, string Grade, double BendRadius)` — Creates a new instance of the ReinforcementSelectionForm and will select in the tree the rebar that contains the given parameters (if possible).
- `public ReinforcementSelectionForm(List<RebarItem> RebarList, string Size, string Grade, double BendRadius, string Usage)` — Creates a new instance of the ReinforcementSelectionForm and will select in the tree the rebar that contains the given parameters (if possible).

### Properties
- `Culture` (CultureInfo, get/set) — Gets the culture.
- `SelectedBendingRadius` (Double, get/set) — Gets the selected rebar bending radius.
- `SelectedGrade` (String, get/set) — Gets the selected rebar grade.
- `SelectedSize` (String, get/set) — Gets the selected rebar size.
- `SelectedUsage` (String, get/set) — Gets the selected usage.

## SaveLoad (class)

The SaveLoad class represents a save-load-save as group of controls including the functionality.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/fdeb51b8-3d26-14af-9c6f-e3e3ce4a0678)

### Constructors
- `public SaveLoad()` — Initializes a new instance of the SaveLoad class with the default language and file extensions.

### Properties
- `HelpFileType` (SaveLoad.HelpFileTypeEnum, get/set) — Gets or sets the help file type.
- `HelpKeyword` (String, get/set) — Gets or sets the HelpKeyword that the help file should be opened for. If you do not provide a HelpKeyword and HelpUrl is not set, the default start page for Tekla Structures Help viewer is displayed.
- `HelpUrl` (String, get/set) — Gets or sets the HelpUrl where the help file is located. If you do not provide a HelpUrl, the Tekla Structures help viewer will be displayed with given HelpKeyword.
- `SaveAsText` (String, get/set) — Gets or sets the text in the SaveAs text box.
- `SaveLoadText` (String, get) — Gets the currently selected file name in the Save/Load combo box.
- `UserDefinedHelpFilePath` (String, get/set) — Gets or sets the file path where the UserDefined help file is located. If you do not provide a UserDefinedHelpFilePath, the general Help file, help.chm, will be displayed.

### Events
#### `AttributesLoadClicked` (`System.EventHandler`)

**Signature:** `public event EventHandler AttributesLoadClicked`

The AttributesLoadClicked event is triggered just after the load button has been clicked.

[Docs](https://developer.tekla.com/topic/en/18/43/605c48b7-8d76-5ca5-f254-15e8ade6c9dc)

#### `AttributesLoaded` (`System.EventHandler`)

**Signature:** `public event EventHandler AttributesLoaded`

The AttributesLoaded event is triggered just after attributes have been loaded into the dialog.

[Docs](https://developer.tekla.com/topic/en/18/43/1e6b2e5a-55dd-692c-0068-e76d5f8da2ad)

#### `AttributesSaveAsClicked` (`System.EventHandler`)

**Signature:** `public event EventHandler AttributesSaveAsClicked`

The AttributesSaveAsClicked event is triggered just after the save as button has been clicked.

[Docs](https://developer.tekla.com/topic/en/18/43/4a8c3c63-13d7-b63c-c57d-8ed8630b3541)

#### `AttributesSaveClicked` (`System.EventHandler`)

**Signature:** `public event EventHandler AttributesSaveClicked`

The AttributesSaveClicked event is triggered just after the save button has been clicked.

[Docs](https://developer.tekla.com/topic/en/18/43/0c6c99fe-ac67-ea61-ff6a-c47046d4c202)

#### `AttributesSaved` (`System.EventHandler`)

**Signature:** `public event EventHandler AttributesSaved`

The AttributesSaved event is triggered just after attributes are saved to a file.

[Docs](https://developer.tekla.com/topic/en/18/43/a32e50ef-a48f-316c-d538-0a239ce1db31)

#### `AttributesSavedAs` (`System.EventHandler`)

**Signature:** `public event EventHandler AttributesSavedAs`

The AttributesSavedAs event is triggered just after attributes are "saved as" to a file.

[Docs](https://developer.tekla.com/topic/en/18/43/e0d0e8e0-481f-c008-fc4a-71417f75fd20)

#### `HelpOpenClicked` (`System.EventHandler`)

**Signature:** `public event EventHandler HelpOpenClicked`

The HelpOpenClicked event is triggered just after the help button has been clicked.

[Docs](https://developer.tekla.com/topic/en/18/43/8c5112c8-453f-e697-cb52-9018fc6aec71)

#### `HelpOpened` (`System.EventHandler`)

**Signature:** `public event EventHandler HelpOpened`

The HelpOpened event is triggered just after the help file is opened.

[Docs](https://developer.tekla.com/topic/en/18/43/b54d692b-1556-36df-5fb0-a726b2a98267)

## SaveLoad.HelpFileTypeEnum (enum)

The help file types.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/0f621a6a-223f-9e74-662c-f09ce6316281)

### Values
- `General` = `0` — The general help file; help.chm.
- `UserDefined` = `1` — The user defined help file.

## ShapeCatalog (class)

The ShapeCatalog class represents a group of controls to select Shapes using the Shape catalog.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/0d92e884-932e-e2c7-91b0-752c95686b70)

### Constructors
- `public ShapeCatalog()` — Initializes a new instance of the ShapeCatalog class.
- `public ShapeCatalog(string shape)` — Initializes a new instance of the ShapeCatalog class and will select the Shape (if available) in the tree.

### Methods
#### `public void OnValueChanged(Object data)`

Event handler for passing generic catalog data back to dialog

**Parameters:**
- `data` (System.Object) — value data

[Docs](https://developer.tekla.com/topic/en/18/43/ef1500b0-92e8-3fce-7d09-ac486efe991c)

### Properties
- `ButtonText` (String, get/set) — Gets and sets the text of the button control.
- `SelectedShape` (String, get/set) — Gets or sets the selected Shape in the control.

### Events
#### `SelectClicked` (`System.EventHandler`)

**Signature:** `public event EventHandler SelectClicked`

The SelectClicked event is raised when the Select button is clicked.

[Docs](https://developer.tekla.com/topic/en/18/43/b94e72b1-fa1d-b9aa-db9e-36f2db93c83e)

#### `SelectionDone` (`System.EventHandler`)

**Signature:** `public event EventHandler SelectionDone`

The SelectionDone event is raised when the selection has been done.

[Docs](https://developer.tekla.com/topic/en/18/43/77741b2b-72a2-3319-04f8-cea1cabae359)

## ShapeSelectionForm (class)

The ShapeSelectionForm class represents a dialog to select Shapes using the Shape catalogs.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/2f45ba74-99dc-bd5c-bef3-221281f1d833)

### Constructors
- `public ShapeSelectionForm(List<ShapeItem> shapes)` — Initializes a new instance of the ShapeSelectionForm class.
- `public ShapeSelectionForm(List<ShapeItem> shapes, string shape)` — Initializes a new instance of the ShapeSelectionForm class, selecting the given Shape in the tree control.

### Properties
- `SelectedShape` (String, get/set) — Gets or sets the selected Shape in the control.

## Tree (class)

The Tree class represents a tree view control that can contain images.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/e0116c1a-12bc-7035-20de-a2e3531be1b1)

### Constructors
- `public Tree()` — Initializes a new instance of the Tree class.

## TreeViewDialog (class)

The TreeViewDialog class represents a "template" of a tree view dialog.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/118aa302-7ff6-f30e-62be-c35fc0fe5270)

### Constructors
- `public TreeViewDialog()` — Initializes a new instance of the TreeViewDialog class.

### Methods
#### `public void ApplyValues(string FileName)`

Loads the dialog values from a file and performs Apply() on the loaded values. To match the files to a certain dialog, the file suffix is set as the dialog type's name.

**Parameters:**
- `FileName` (System.String) — The name of the file.

[Docs](https://developer.tekla.com/topic/en/18/43/c4096cdf-60d9-f7b8-1daf-e71de20cf47d)

#### `public bool GetConnectionStatus()`

Returns true if a proper connection to the Tekla Structures process has been established. Currently, there's no way to re-establish the connection.

**Returns:** `Boolean` — True on success.

[Docs](https://developer.tekla.com/topic/en/18/43/2ba6677a-b8a7-cbfa-2724-e58464054abe)

#### `public void InitializeForm()`

Prepares the data storage for the dialog and scans through the fields.

[Docs](https://developer.tekla.com/topic/en/18/43/70881deb-ea45-96c7-ee73-399bc7b76a90)

#### `public void LoadValues(string FileName)`

Loads the dialog values from a file. To match the files to a certain dialog, the file suffix is set as the dialog type's name.

**Parameters:**
- `FileName` (System.String) — The name of the file.

[Docs](https://developer.tekla.com/topic/en/18/43/df11f56f-8372-2886-e011-b4fcad4ac3de)

#### `public void ModifyValues(string FileName)`

Loads the dialog values from a file and performs Modify() on the loaded values. To match the files to a certain dialog, the file suffix is set as the dialog type's name.

**Parameters:**
- `FileName` (System.String) — The name of the file.

[Docs](https://developer.tekla.com/topic/en/18/43/d20a7c4e-e913-92e5-0925-44015cc68ebf)

#### `public void SaveValues(string fileName)`

Serializes the dialog values to an xml file.

**Parameters:**
- `fileName` (System.String) — The path of the file.

[Docs](https://developer.tekla.com/topic/en/18/43/a817a258-86a2-8616-be00-eda92c7e5f8e)

#### `public void SetAttributeValue(Control Ctrl, Object Value)`

Sets a value for the given control. When the dialog is not shown, setting a property directly for a control (such as textBox1.Text = "text") will not work for controls that have a Tekla Structures AttributeTypeName set. This method is going to have to be used to set the value.

**Parameters:**
- `Ctrl` (System.Windows.Forms.Control) — The control whose value to set.
- `Value` (System.Object) — The new value. It can be of any primitive datatype supported by the control's AttributeType (normally int, double or string).

[Docs](https://developer.tekla.com/topic/en/18/43/30e56a80-1c86-4606-781c-2a91b7bb8884)

#### `public void ShowForm()`

Displays the form.

[Docs](https://developer.tekla.com/topic/en/18/43/79258182-6491-e7e6-99d0-de2072cb85a5)

#### `[ObsoleteAttribute("Does currently nothing. Will be removed in near future.")] public void UpdateValues()`

Rereads and updates all the field values on the form.

[Docs](https://developer.tekla.com/topic/en/18/43/5ab0c9d1-4ddf-1e05-29ca-560e387350e5)

### Properties
- `Localization` (Localization, get) — The localization instance for the dialog. Each dialog has its own localization instance that has read the localization files needed for that dialog.

## WpfBoltCatalogSize (class)

The WPFBoltCatalogSize class represents a control to select the bolt size using the bolt catalog. Bolt catalog controls always need to be in pairs, meaning in every dialog there has to be a WpfBoltCatalogStandard and a WpfBoltCatalogSize control.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/574e3bb2-2c07-df98-6479-7cb07dbaf7cb)

### Constructors
- `public WpfBoltCatalogSize()` — Initiates a new instance of the control.

## WpfBoltCatalogStandard (class)

The WpfBoltCatalogStandard class represents a control to select the bolt standard using the bolt catalog. Bolt catalog controls always need to be in pairs, meaning in every dialog there has to be a WpfBoltCatalogStandard and a WpfBoltCatalogSize control.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/d907858b-63ba-60b7-404e-2621c059e74f)

### Constructors
- `public WpfBoltCatalogStandard()` — Initiates a new instance of the control.

### Properties
- `BoltSizes` (CustomObservableCollection<Distance>, get) — Returns bolt sizes in catalog based on selected grade

## WpfComponentCatalog (class)

The WpfComponentCatalog class represents a group of controls to select components using the component catalog.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/c90f5755-fb29-0d66-6674-1fac8f2cd570)

### Constructors
- `public WpfComponentCatalog()` — Initializes a new instance of the WpfComponentCatalog class.

### Methods
#### `public void InitializeComponent()`

InitializeComponent

[Docs](https://developer.tekla.com/topic/en/18/43/e6a95146-4efe-69b4-1ff2-5871046dcc49)

### Properties
- `ButtonHeight` (Double, get/set) — Gets and sets the height of the button control.
- `ButtonText` (String, get/set) — Gets and sets the text of the button control.
- `ButtonWidth` (Double, get/set) — Gets and sets the width of the button control.
- `SelectedName` (String, get/set) — Gets or sets the selected component name in the control.
- `SelectedNumber` (Int32, get/set) — Gets or sets the selected component number in the control.

### Events
#### `SelectClicked` (`System.EventHandler`)

**Signature:** `public event EventHandler SelectClicked`

The SelectClicked event is raised when the Select button is clicked.

[Docs](https://developer.tekla.com/topic/en/18/43/57e90535-aed5-b50d-f2aa-d9436e312296)

#### `SelectionDone` (`System.EventHandler`)

**Signature:** `public event EventHandler SelectionDone`

The SelectionDone event is raised when the selection has been done.

[Docs](https://developer.tekla.com/topic/en/18/43/348083ac-1932-805d-1ded-93d7786faf89)

## WpfCreateApplyCancel (class)

Interaction logic for WPFCreateAppyCancel.xaml. Control is meant to be used with non-dependent plug-ins

[Vendor docs](https://developer.tekla.com/topic/en/18/43/49fc797a-5d42-56cd-390b-b9428a849110)

### Constructors
- `public WpfCreateApplyCancel()` — Constructor

### Methods
#### `public void InitializeComponent()`

InitializeComponent

[Docs](https://developer.tekla.com/topic/en/18/43/ea550edc-83ae-a147-9d1a-87bbf1840878)

### Events
#### `ApplyClicked` (`System.EventHandler`)

**Signature:** `public event EventHandler ApplyClicked`

The ApplyClicked event is raised when the Apply button is clicked.

[Docs](https://developer.tekla.com/topic/en/18/43/24f1d8e1-5a32-e9ec-07d2-9dc360ea58ca)

#### `CancelClicked` (`System.EventHandler`)

**Signature:** `public event EventHandler CancelClicked`

The CancelClicked event is raised when the Cancel button is clicked.

[Docs](https://developer.tekla.com/topic/en/18/43/9abb1f10-36ab-1264-baa4-90806a29a4df)

#### `CreateClicked` (`System.EventHandler`)

**Signature:** `public event EventHandler CreateClicked`

The ApplyClicked event is raised when the Apply button is clicked.

[Docs](https://developer.tekla.com/topic/en/18/43/18a7e64a-9cd0-cd4c-c429-ccade298c1e8)

## WpfFilterCheckBox (class)

The WpfFilterAttribute class represents a checkbox control for enabling/disabling dialog attributes defined with StructuresFieldAttribute.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/a0491cd1-bdc2-1068-4e51-68dbea829372)

### Constructors
- `public WpfFilterCheckBox()` — Initiates a new instance of the control.

### Properties
- `AttributeName` (String, get/set) — Dialog attribute name defined with StructuresFieldAttribute

## WpfMaterialCatalog (class)

The WpfMaterialCatalog class represents a group of controls to select materials using the material catalog.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/c5c22ec5-ec18-07be-d0d1-3aa1c2d0abcf)

### Constructors
- `public WpfMaterialCatalog()` — Initiates a new instance of the control.
- `public WpfMaterialCatalog(string Material)` — Creates a new instance of the MaterialCatalog and will select the material (if available) in the tree.

### Methods
#### `public void InitializeComponent()`

InitializeComponent

[Docs](https://developer.tekla.com/topic/en/18/43/67c07c6d-80a2-e507-6eee-eb9f779a7fd0)

### Properties
- `ButtonHeight` (Double, get/set) — Gets and sets the height of the button control.
- `ButtonText` (String, get/set) — Gets and sets the text of the button control.
- `ButtonWidth` (Double, get/set) — Gets and sets the width of the button control.
- `SelectedMaterial` (String, get/set) — Gets the selected material in the control.

### Events
#### `SelectClicked` (`System.EventHandler`)

**Signature:** `public event EventHandler SelectClicked`

The SelectClicked event is raised when the Select button is clicked.

[Docs](https://developer.tekla.com/topic/en/18/43/b297791c-4da5-0bae-5a81-7453e8d1686e)

#### `SelectionDone` (`System.EventHandler`)

**Signature:** `public event EventHandler SelectionDone`

The SelectionDone event is raised when the selection has been done.

[Docs](https://developer.tekla.com/topic/en/18/43/d68f6463-28a9-f69f-bae4-edccd7ad5a44)

## WpfMeshCatalog (class)

The WpfMeshCatalog class represents a group of controls to select meshes using the mesh catalog.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/c4e9da66-f927-b2d4-39c4-88086a35be0b)

### Constructors
- `public WpfMeshCatalog()` — Initiates a new instance of the control.

### Methods
#### `public void InitializeComponent()`

InitializeComponent

[Docs](https://developer.tekla.com/topic/en/18/43/cd0c4661-c259-a999-918b-24a5bbb524a0)

### Properties
- `ButtonHeight` (Double, get/set) — Gets and sets the height of the button control.
- `ButtonText` (String, get/set) — Gets and sets the text of the button control.
- `ButtonWidth` (Double, get/set) — Gets and sets the width of the button control.
- `SelectedMeshGrade` (String, get/set) — The mesh grade in the control.
- `SelectedMeshName` (String, get/set) — The mesh name in the control.

### Events
#### `SelectClicked` (`System.EventHandler`)

**Signature:** `public event EventHandler SelectClicked`

The SelectClicked event is raised when the Select button is clicked.

[Docs](https://developer.tekla.com/topic/en/18/43/0a69ef90-5c7d-15b9-3e4f-54cc6aa0a0ab)

#### `SelectionDone` (`System.EventHandler`)

**Signature:** `public event EventHandler SelectionDone`

The SelectionDone event is raised when the selection has been done.

[Docs](https://developer.tekla.com/topic/en/18/43/2643be87-58d7-16c8-3ad5-fb99f8ab604e)

## WpfOkApplyModifyGetOnOffCancel (class)

Interaction logic for WpfOkApplyModifyGetOnOffCancel.xaml

[Vendor docs](https://developer.tekla.com/topic/en/18/43/cc8aa487-0ba6-2426-b3b7-8dd26d8ca44e)

### Constructors
- `public WpfOkApplyModifyGetOnOffCancel()` — Constructor

### Methods
#### `public void InitializeComponent()`

InitializeComponent

[Docs](https://developer.tekla.com/topic/en/18/43/45362270-530d-9ea0-8cb4-1a76e52c6e5a)

### Events
#### `ApplyClicked` (`System.EventHandler`)

**Signature:** `public event EventHandler ApplyClicked`

The ApplyClicked event is raised when the Apply button is clicked.

[Docs](https://developer.tekla.com/topic/en/18/43/87b7d848-4700-bd08-56a6-12e19cc59e84)

#### `CancelClicked` (`System.EventHandler`)

**Signature:** `public event EventHandler CancelClicked`

The CancelClicked event is raised when the Cancel button is clicked.

[Docs](https://developer.tekla.com/topic/en/18/43/6325f779-92d8-94d7-f60d-43079c015275)

#### `GetClicked` (`System.EventHandler`)

**Signature:** `public event EventHandler GetClicked`

The GetClicked event is raised when the Get button is clicked.

[Docs](https://developer.tekla.com/topic/en/18/43/1e696b04-76fc-5379-3469-a63218dd55d2)

#### `ModifyClicked` (`System.EventHandler`)

**Signature:** `public event EventHandler ModifyClicked`

The ModifyClicked event is raised when the Modify button is clicked.

[Docs](https://developer.tekla.com/topic/en/18/43/cb899dd4-382e-7883-9300-04265e89ff1f)

#### `OkClicked` (`System.EventHandler`)

**Signature:** `public event EventHandler OkClicked`

The OkClicked event is raised when the Ok button is clicked.

[Docs](https://developer.tekla.com/topic/en/18/43/31b71dde-dae2-31f6-7dd5-c3d7b1846745)

#### `OnOffClicked` (`System.EventHandler`)

**Signature:** `public event EventHandler OnOffClicked`

The OnOffClicked event is raised when the OnOff button is clicked.

[Docs](https://developer.tekla.com/topic/en/18/43/8ef68f87-6e77-c7db-3cd3-8b76291ae01e)

## WpfOkCreateCancel (class)

Interaction logic for WPFOkCreateCancel.xaml

[Vendor docs](https://developer.tekla.com/topic/en/18/43/faf775c3-7dd6-4005-176c-c5835d7e04a1)

### Constructors
- `public WpfOkCreateCancel()` — Constructor

### Methods
#### `public void InitializeComponent()`

InitializeComponent

[Docs](https://developer.tekla.com/topic/en/18/43/b5c8a0a5-f7bf-904f-6cb3-224e6c1c703d)

### Events
#### `CancelClicked` (`System.EventHandler`)

**Signature:** `public event EventHandler CancelClicked`

The CancelClicked event is raised when the Cancel button is clicked.

[Docs](https://developer.tekla.com/topic/en/18/43/6df16027-87de-bd4a-5fe8-7e867f2285d4)

#### `CreateClicked` (`System.EventHandler`)

**Signature:** `public event EventHandler CreateClicked`

The ApplyClicked event is raised when the Apply button is clicked.

[Docs](https://developer.tekla.com/topic/en/18/43/d1e4b314-ad4f-1867-6f97-ab7aeb9fdf45)

#### `OkClicked` (`System.EventHandler`)

**Signature:** `public event EventHandler OkClicked`

The OkClicked event is raised when the Ok button is clicked.

[Docs](https://developer.tekla.com/topic/en/18/43/d72cc6d2-5319-f32b-9d86-da5930651ee3)

## WpfProfileCatalog (class)

The WpfProfileCatalog class represents a group of controls to select profiles using the profile catalog.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/b31164fe-c4cf-bc39-b324-180485e312e7)

### Constructors
- `public WpfProfileCatalog()` — Initiates a new instance of the control.
- `public WpfProfileCatalog(string Profile)` — Initiates a new instance of the control selecting the given profile.

### Methods
#### `public void Dispose()`

Cleans up any resources being used.

[Docs](https://developer.tekla.com/topic/en/18/43/288dfed8-4aa5-35fb-5fa6-3966427ea546)

#### `public void InitializeComponent()`

InitializeComponent

[Docs](https://developer.tekla.com/topic/en/18/43/46888b24-6470-9c28-0ead-4e290291fe4c)

#### `public void OnValueChanged(Object data)`

Event handler for passing generic catalog data back to dialog

**Parameters:**
- `data` (System.Object) — value data

[Docs](https://developer.tekla.com/topic/en/18/43/6e26d06b-7e86-57c4-8fed-e82d758a689f)

### Properties
- `ButtonHeight` (Double, get/set) — Gets and sets the height of the button control.
- `ButtonText` (String, get/set) — Gets and sets the text of the button control.
- `ButtonWidth` (Double, get/set) — Gets and sets the width of the button control.
- `SelectedProfile` (String, get/set) — The selected profile in the control.

### Events
#### `SelectClicked` (`System.EventHandler`)

**Signature:** `public event EventHandler SelectClicked`

The SelectClicked event is raised when the Select button is clicked.

[Docs](https://developer.tekla.com/topic/en/18/43/28c5418c-bf2a-30ee-307a-b88b944a5bb8)

#### `SelectionDone` (`System.EventHandler`)

**Signature:** `public event EventHandler SelectionDone`

The SelectionDone event is raised when the selection has been done.

[Docs](https://developer.tekla.com/topic/en/18/43/4d1356b6-f3e0-efbd-e8a1-3792d943a2d4)

## WpfReinforcementCatalog (class)

The WpfReinforcementCatalog class represents a group of controls to select rebars using the reinforcement catalog.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/cfa74af2-43e5-d7fa-1636-2a2ed941df68)

### Constructors
- `public WpfReinforcementCatalog()` — Initiates a new instance of the control.
- `public WpfReinforcementCatalog(string Size, string Grade, double BendRadius)` — Creates a new instance of the ReinforcementCatalog and will select in the tree the rebar that contains the given parameters (if possible).

### Methods
#### `public void InitializeComponent()`

InitializeComponent

[Docs](https://developer.tekla.com/topic/en/18/43/22908876-e22a-7b17-50fb-09fbf6790d38)

#### `public void OnValueChanged(Object data)`

Event handler for passing generic catalog data back to dialog

**Parameters:**
- `data` (System.Object) — value data

[Docs](https://developer.tekla.com/topic/en/18/43/75b7e280-d530-bad4-bfd3-5cfc3fdb61d5)

### Properties
- `ButtonHeight` (Double, get/set) — Gets and sets the height of the button control.
- `ButtonText` (String, get/set) — Gets and sets the text of the button control.
- `ButtonWidth` (Double, get/set) — Gets and sets the width of the button control.
- `SelectedRebarBendingRadius` (Double, get/set) — Gets the selected rebar bending radius in the control.
- `SelectedRebarGrade` (String, get/set) — Gets the selected rebar grade in the control.
- `SelectedRebarSize` (String, get/set) — Gets the selected rebar size in the control.
- `SelectedUsage` (Int32, get/set) — Gets the selected rebar usage in the control.

### Events
#### `SelectClicked` (`System.EventHandler`)

**Signature:** `public event EventHandler SelectClicked`

The SelectClicked event is raised when the Select button is clicked.

[Docs](https://developer.tekla.com/topic/en/18/43/95d90edf-509d-15a6-a9f4-71f6eaf66be1)

#### `SelectionDone` (`System.EventHandler`)

**Signature:** `public event EventHandler SelectionDone`

The SelectionDone event is raised when the selection has been done.

[Docs](https://developer.tekla.com/topic/en/18/43/323ecda0-3d15-0284-b95a-440d146262f9)

## WpfSaveLoad (class)

Interaction logic for WpfSaveload.xaml

[Vendor docs](https://developer.tekla.com/topic/en/18/43/6af15825-83ac-b471-8428-dce47a36c5aa)

### Constructors
- `public WpfSaveLoad()` — Initializes a new instance of the WpfSaveLoad class with the default language and file extensions.

### Methods
#### `public void InitializeComponent()`

InitializeComponent

[Docs](https://developer.tekla.com/topic/en/18/43/8ac86348-7a2a-a276-0956-4af37c0c03dd)

### Properties
- `HelpFileType` (WpfSaveLoad.HelpFileTypeEnum, get/set) — Gets or sets the help file type.
- `HelpKeyword` (String, get/set) — Gets or sets the HelpKeyword that the help file should be opened for. If you do not provide a HelpKeyword and HelpUrl is not set, the default start page for Tekla Structures Help viewer is displayed.
- `HelpUrl` (String, get/set) — Gets or sets the HelpUrl where the help file is located. If you do not provide a HelpUrl, the Tekla Structures help viewer will be displayed with given HelpKeyword.
- `SaveAsText` (String, get/set) — Gets or sets the text in the SaveAs text box.
- `SaveLoadText` (String, get) — Gets the currently selected file name in the Save/Load combo box.
- `SelectedFileName` (String, get/set) — Gets or sets the text in the SaveAs text box.
- `UserDefinedHelpFilePath` (String, get/set) — Gets or sets the file path where the UserDefined help file is located. If you do not provide a UserDefinedHelpFilePath, the general Help file, help.chm, will be displayed.

### Events
#### `AttributesLoadClicked` (`System.EventHandler`)

**Signature:** `public event EventHandler AttributesLoadClicked`

The AttributesLoadClicked event is triggered just after the load button has been clicked.

[Docs](https://developer.tekla.com/topic/en/18/43/60efdf07-87d4-2a9f-ec57-15b1eb3613e6)

#### `AttributesLoaded` (`System.EventHandler`)

**Signature:** `public event EventHandler AttributesLoaded`

The AttributesLoaded event is triggered just after attributes have been loaded into the dialog.

[Docs](https://developer.tekla.com/topic/en/18/43/d733659e-c437-a6d3-0eea-71348432597f)

#### `AttributesSaveClicked` (`System.EventHandler`)

**Signature:** `public event EventHandler AttributesSaveClicked`

The AttributesSaveClicked event is triggered just after the save button has been clicked.

[Docs](https://developer.tekla.com/topic/en/18/43/f6a7e6fa-0397-07d0-1207-94a897971a07)

#### `AttributesSaved` (`System.EventHandler`)

**Signature:** `public event EventHandler AttributesSaved`

The AttributesSaved event is triggered just after attributes are saved to a file.

[Docs](https://developer.tekla.com/topic/en/18/43/b7872aac-f55d-67ce-8ab5-879920ff05ac)

#### `HelpOpenClicked` (`System.EventHandler`)

**Signature:** `public event EventHandler HelpOpenClicked`

The HelpOpenClicked event is triggered just after the help button has been clicked.

[Docs](https://developer.tekla.com/topic/en/18/43/7c3df761-1b2b-6c99-979d-62491ea8bca3)

#### `HelpOpened` (`System.EventHandler`)

**Signature:** `public event EventHandler HelpOpened`

The HelpOpened event is triggered just after the help file is opened.

[Docs](https://developer.tekla.com/topic/en/18/43/f504ff6c-f5c0-695c-baba-e402ac64c1db)

## WpfSaveLoad.HelpFileTypeEnum (enum)

The help file types.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/ba4f5985-87b5-c2d5-cf17-3215c34a6c62)

### Values
- `General` = `0` — The general help file; help.chm.
- `UserDefined` = `1` — The user defined help file.

## WpfShapeCatalog (class)

The WpfShapeCatalog class represents a group of controls to select shapes using the shape catalog.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/8ded86a9-90a9-c7ba-9e81-a500d188759c)

### Constructors
- `public WpfShapeCatalog()` — Initializes a new instance of the ShapeCatalog class.
- `public WpfShapeCatalog(string shape)` — Initializes a new instance of the ShapeCatalog class and will select the Shape (if available) in the tree.

### Methods
#### `public void InitializeComponent()`

InitializeComponent

[Docs](https://developer.tekla.com/topic/en/18/43/7ea6ca8b-0b2d-50df-69c4-a6edf3aaea70)

#### `public void OnValueChanged(Object data)`

Event handler for passing generic catalog data back to dialog

**Parameters:**
- `data` (System.Object) — value data

[Docs](https://developer.tekla.com/topic/en/18/43/6142c281-2aea-1606-72b4-abf7c44ed66e)

### Properties
- `ButtonHeight` (Double, get/set) — Gets and sets the height of the button control.
- `ButtonText` (String, get/set) — Gets and sets the text of the button control.
- `ButtonWidth` (Double, get/set) — Gets and sets the width of the button control.
- `SelectedShape` (String, get/set) — Gets or sets the selected Shape in the control.

### Events
#### `SelectClicked` (`System.EventHandler`)

**Signature:** `public event EventHandler SelectClicked`

The SelectClicked event is raised when the Select button is clicked.

[Docs](https://developer.tekla.com/topic/en/18/43/0a3c217b-d39b-f5da-4c55-6508b2802ea9)

#### `SelectionDone` (`System.EventHandler`)

**Signature:** `public event EventHandler SelectionDone`

The SelectionDone event is raised when the selection has been done.

[Docs](https://developer.tekla.com/topic/en/18/43/f6e92018-f5e0-9b42-e141-04b5aa0134fc)

## WpfSteelFinishComboBox (class)

The WpfSteelFinishComboBox class represents a control to select the steel finish from the list.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/a7063f50-c3b8-e569-ce78-14be1adc7c71)

### Constructors
- `public WpfSteelFinishComboBox()` — Initiates a new instance of the control.

### Properties
- `Finish` (String, get) — Gets or sets the finish value

