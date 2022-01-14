# ahks
An application that allows you to create and configure chat-based AHKs for the Roblox Bhop / Surf games, packaged with premade AHKs to start off. Use the AHKs at your own risk, as this does not return punishment through the StrafesNET rules (due to these packaged AHKs being legal), but this breaks the Roblox TOS.

## Key:
```
<> required
<?> optional
<a|b> a or b
<...> array/multiple of
```
## Usage
```
[util]
list - list all .ahk files in ahk directory
new <name> <call_bind> - creates a new ahk and prompts to set a binding
delete <ahkfile> - deletes said ahk file
setbind <ahkfile> - prompts to set a binding
doubleslash <"true" | "false"> <ahkfile?> - if true, double slash; optionally input specific ahk, default will set all files

[config]
dir - the directory the application scans for, no recursion
setdir - set the new directory for the application
```
