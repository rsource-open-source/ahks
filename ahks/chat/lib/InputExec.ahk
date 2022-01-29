do(PARA)
{   
    for _, inputvalue in PARA {
        Sleep, 15
        SendInput %inputvalue%
    }
}

exec(PARA)
{
    do(["/", PARA, "{enter}"])
}
