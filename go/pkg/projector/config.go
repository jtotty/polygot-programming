package projector

import (
	"fmt"
	"os"
	"path"
)

type Operation = int

const (
	Print Operation = iota
	Add
	Remove
)

type Config struct {
	Args      []string
	Operation Operation
	Config    string
	Pwd       string
}

func getOperation(options *Options) Operation {
	if len(options.Args) == 0 {
		return Print
	}

	if options.Args[0] == "add" {
		return Add
	}

	if options.Args[0] == "remove" {
		return Remove
	}

	return Print
}

func getArgs(options *Options) ([]string, error) {
	if len(options.Args) == 0 {
		return []string{}, nil
	}

	operation := getOperation(options)

	if operation == Add {
		if len(options.Args) != 3 {
			return nil, fmt.Errorf("add operation requires 2 arguments, but received %v", len(options.Args)-1)
		}

		return options.Args[1:], nil
	}

	if operation == Remove {
		if len(options.Args) != 2 {
			return nil, fmt.Errorf("remove operation requires 1 arguments, but received %v", len(options.Args)-1)
		}

		return options.Args[1:], nil
	}

	if len(options.Args) > 1 {
		return nil, fmt.Errorf("print requires 0 or 1 arguements, but recieved %v", len(options.Args))
	}

	return options.Args, nil
}

func getConfig(options *Options) (string, error) {
	if options.Config != "" {
		return options.Config, nil
	}

	configDir, err := os.UserConfigDir()
	if err != nil {
		return "", err
	}

	return path.Join(configDir, "projector", "projector-go.json"), nil
}

func getPwd(options *Options) (string, error) {
	if options.Pwd != "" {
		return options.Pwd, nil
	}

	return os.Getwd()
}

func NewConfig(options *Options) (*Config, error) {
	pwd, err := getPwd(options)
	if err != nil {
		return nil, err
	}

	config, err := getConfig(options)
	if err != nil {
		return nil, err
	}

	args, err := getArgs(options)
	if err != nil {
		return nil, err
	}

	return &Config{
		Pwd:       pwd,
		Config:    config,
		Operation: getOperation(options),
		Args:      args,
	}, nil
}
