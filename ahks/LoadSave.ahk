; aka savelock

#Include %A_ScriptDir%\lib\InputExec.ahk
#IfWinActive, Roblox
global numSaves := 0

1::
{
    numSaves++
    exec("save " . numSaves)
}
return

2::
{
    exec("load " . numSaves)
}
return

~Del::
InputBox, numSaves, Jump to which save? (Numbers only)
return

Z::
numSaves--
return

X::
numSaves++
return

~Home::Suspend
