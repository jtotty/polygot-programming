package projector_test

import (
	"reflect"
	"testing"

	"github.com/jtotty/projector-go/pkg/projector"
)

func getOptions(args []string) *projector.Options {
	opts := &projector.Options{
		Args:   args,
		Config: "",
		Pwd:    "",
	}

	return opts
}

func testConfig(t *testing.T, args []string, expectedArgs []string, operation projector.Operation) {
	opts := getOptions(args)
	config, err := projector.NewConfig(opts)

	if err != nil {
		t.Errorf("expected no errors but recieved %v", err)
	}

	if !reflect.DeepEqual(expectedArgs, config.Args) {
		t.Errorf("expected args to be %+v but recieved %+v", expectedArgs, config.Args)
	}

	if config.Operation != operation {
		t.Errorf("expected operation was %v but recieved %v", operation, config.Operation)
	}
}

func TestConfigPrint(t *testing.T) {
	testConfig(t, []string{}, []string{}, projector.Print)
}

func TestConfigPrintKey(t *testing.T) {
	testConfig(t, []string{"foo"}, []string{"foo"}, projector.Print)
}

func TestConfigAddKeyValue(t *testing.T) {
	testConfig(t, []string{"add", "foo", "bar"}, []string{"foo", "bar"}, projector.Add)
}

func TestConfigRemoveKeyValue(t *testing.T) {
	testConfig(t, []string{"remove", "foo"}, []string{"foo"}, projector.Remove)
}
