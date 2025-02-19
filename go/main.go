package main

import "fmt"

func returnsError(value int) error {
	return fmt.Errorf("Error")
}

type Foo struct{}

func (f *Foo) thisIsOnFoo() error {
	return fmt.Errorf("Error")
}

func CreateFoo(fail bool) (*Foo, error) {
	if fail {
		return nil, fmt.Errorf("Error")
	}

	return &Foo{}, nil
}

func main() {
	foo, err := CreateFoo(true)

	if err != nil {
		return nil, err
	}
	foo.thisIsOnFoo()
}
