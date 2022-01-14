do(array)
{   
    for _, inputvalue in array {
        Sleep, 15
        SendInput %inputvalue%
    }
}

exec(key)
{
    do(["/", key, "{enter}"])
}
