package bot

import (
	"errors"
	"fmt"
	"strings"
)

type Command struct {
	Name string
	Args []string
}

type CommandMeta struct {
	Name        string
	Header      string
	Description string
	ArgCount    int
}

var commands = map[string]CommandMeta{
	"help": {
		Name:        "help",
		Header:      "help <commandName | nil>",
		Description: "This command will display info for a command. Example usage: 'help tb'",
		ArgCount:    -1,
	},
	"march": {
		Name:        "march",
		Header:      "march <elementName>",
		Description: "This command will change March 7th's multipath. Example usage: 'march ice'",
		ArgCount:    1,
	},
	"tb": {
		Name:        "tb",
		Header:      "tb <elementName>",
		Description: "This command will change Trailblazer's multipath. Example usage: 'tb fire'",
		ArgCount:    1,
	},
	"gender": {
		Name:        "gender",
		Header:      "gender <genderName>",
		Description: "This command will change Trailblazer's gender. Example usage: 'gender boy'",
		ArgCount:    1,
	},
}

func getCommandList() string {
	names := make([]string, 0, len(commands))
	for name := range commands {
		names = append(names, name)
	}
	return strings.Join(names, ", ")
}

func (c *Command) validate() error {
	meta, exists := commands[c.Name]
	if !exists {
		return fmt.Errorf("unknown command: %s. available commands: %s", c.Name, getCommandList())
	}

	argLen := len(c.Args)
	switch meta.ArgCount {
	case -1:
		if argLen > 1 {
			return fmt.Errorf("%s command takes zero or one argument", meta.Name)
		}
	case 0:
		if argLen != 0 {
			return fmt.Errorf("%s command takes no arguments", meta.Name)
		}
	default:
		if argLen != meta.ArgCount {
			return fmt.Errorf("%s command takes exactly %d argument(s)", meta.Name, meta.ArgCount)
		}
	}
	return nil
}

func ParseCommand(input string) (Command, error) {
	parts := strings.Fields(input)
	if len(parts) == 0 {
		return Command{}, errors.New("input cannot be empty")
	}
	cmd := Command{
		Name: strings.ToLower(parts[0]),
		Args: parts[1:],
	}
	if err := cmd.validate(); err != nil {
		return cmd, err
	}
	return cmd, nil
}

func GetChatHistory() []string {
	history := make([]string, 1, len(commands)+1)
	history[0] = "Available Commands:"
	for _, meta := range commands {
		history = append(history, meta.Header)
	}
	for i, j := 0, len(history)-1; i < j; i, j = i+1, j-1 {
		history[i], history[j] = history[j], history[i]
	}
	return history
}

func AssertNotEqualUid(playerUid uint32) {
	if Loaded.Uid == playerUid {
		panic("Player.Uid must not be equal with Bot.Uid")
	}
}

func createDefault() Bot {
	return Bot{
		Uid:          777,
		Username:     "LeBot.James",
		HeadIconID:   201402,
		ChatBubbleID: 220005,
	}
}
