package main

import (
	"fmt"
	"log"

	"github.com/Dsniels/projector/pkg/projector"
)

func main() {
	opts, err := projector.GetOpts()
	if err != nil {
		log.Fatalf("unable to get options %v", err)
	}
	config, err := projector.NewConfig(opts)
	if err != nil {
		log.Fatalf("unable to get the config %v", err)
	}
	fmt.Printf("Config: %+v \n", config)
	fmt.Printf("Options: %+v", opts)

}
