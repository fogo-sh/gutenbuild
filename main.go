package main

import (
	"io/ioutil"
	"os"

	"github.com/BurntSushi/toml"
	"github.com/fogo-sh/gutenbuild/runtime"
	"github.com/rs/zerolog"
	"github.com/rs/zerolog/log"
)

func main() {
	zerolog.TimeFieldFormat = zerolog.TimeFormatUnix
	log.Logger = zerolog.New(os.Stdout)

	log.Print("starting gutenbuild")

	pipeline_content_bytes, err := ioutil.ReadFile("./pipeline.toml")
	if err != nil {
		log.Error().Err(err).Msg("error reading pipeline.toml")
		os.Exit(1)
	}

	pipeline_content := string(pipeline_content_bytes)

	pipeline := new(runtime.Pipeline)
	_, err = toml.Decode(pipeline_content, &pipeline)
	if err != nil {
		log.Error().Err(err).Msg("error decoding pipeline.toml")
		os.Exit(1)
	}

	pipeline.Run()
}
