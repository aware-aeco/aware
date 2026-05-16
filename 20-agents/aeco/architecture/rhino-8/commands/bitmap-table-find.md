# bitmap-table-find

Lifecycle: single

This function first attempts to find the file with "name" on the disk.             If it does find it, "fileName" is set to the full path of the file and             the BitmapEntry returned will be null, even if there was a BitmapEntry             with "name" in the bitmap table.             If the function cannot find the file on the disk, it searches the bitmap             table.  If it finds it, the returned BitmapEntry entry will be the entry             in the table with that name.             Additionally, if "createFile" is true, and an entry is found, the file             will be written to the disk and it's full path will be contained in "fileName".
