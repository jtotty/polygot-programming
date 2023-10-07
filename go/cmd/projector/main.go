package main

import (
	"fmt"
	"log"

	"github.com/jtotty/projector-go/pkg/projector"
)

func main() {
	options, err := projector.GetOptions()
	if err != nil {
		log.Fatalf("unable to get options %v", err)
	}

	config, err := projector.NewConfig(options)
	if err != nil {
		log.Fatalf("unable to get config %v", err)
	}

	fmt.Printf("config: %+v", config)
}
