package atbash

import "strings"

var (
	alphabet = "abcdefghijklmnopqrstuvwxyz"
)

func Atbash(input string) string {
	var cipher []string

	normalized := ""
	for _, i := range input {
		if i >= 'a' && i <= 'z' {
			normalized += string('z' - i + 'a')
		} else if i >= 'A' && i <= 'Z' {
			normalized += string('Z' - i + 'a')
		} else if i >= '0' && i <= '9' {
			normalized += string(i)
		}

		if len(normalized) == 5 {
			cipher = append(cipher, normalized)
			normalized = ""
		}
	}

	if len(normalized) > 0 {
		cipher = append(cipher, normalized)
	}

	return strings.Join(cipher, " ")
}
