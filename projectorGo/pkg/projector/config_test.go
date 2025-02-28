package projector_test

import (
	"reflect"
	"testing"

	"github.com/Dsniels/projector/pkg/projector"
)

func testConfig(t *testing.T, args []string, operation projector.Operation, expectedArgs []string) {
	opts := projector.Opts{
		Args:   args,
		Config: "",
		Pwd:    "",
	}

	config, err := projector.NewConfig(&opts)
	if err != nil {
		t.Errorf("expected to get no error %v", err)
	}

	if !reflect.DeepEqual(expectedArgs, config.Args) {
		t.Errorf("expected args to be %v but got %v", expectedArgs, config.Args)
	}

	if config.Operation != operation {
		t.Errorf("operation expect was %v but got %v", operation, config.Operation)
	}
}

func TestConfigPrintKey(t *testing.T) {
	testConfig(t, []string{"foo"}, projector.Print, []string{"foo"})
}
func TestConfigPrint(t *testing.T) {
	testConfig(t, []string{}, projector.Print, []string{})
}

func TestConfigRemoveKeyValue(t *testing.T) {
	testConfig(t, []string{"rm", "foo"}, projector.Remove, []string{"foo"})
}

func TestConfigAddKeyValue(t *testing.T) {
	testConfig(t, []string{"add", "foo", "bar"}, projector.Add, []string{"foo", "bar"})
}
