do(array)
{   
    for _, inputvalue in array {
        Sleep, 10
        SendInput %inputvalue%
    }
}

exec(key)
{
    do(["/", "/", key, "{enter}"])
}
