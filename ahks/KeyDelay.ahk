SendMode Input
$a::
	{
		Sleep, 7
		SendInput {a down}
	}
	return

$d::
	{
		Sleep, 7
		SendInput {d down}
	}
	return


$a up::
	{
		SendInput {a up}
	}
	return

$d up::
	{
		SendInput {d up}
	}
	return