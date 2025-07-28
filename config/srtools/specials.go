package srtools

var RemembranceIDs = map[uint32]bool{
	8007: true,
	8008: true,
	1402: true,
	1407: true,
	1409: true,
}

var EnhancedIDs = map[uint32]bool{
	1006: true,
	1205: true,
	1005: true,
	1212: true,
}

func isMarchHuntBuffID(buffID uint32) bool {
	return buffID >= 122401 && buffID <= 122403
}
