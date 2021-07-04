package main

import (
	"github.com/fogo-sh/gutenbuild/runtime"

	"github.com/BurntSushi/toml"

	"os"

	"github.com/rs/zerolog"
	"github.com/rs/zerolog/log"

	"io/ioutil"
)

func main() {
	zerolog.TimeFieldFormat = zerolog.TimeFormatUnix
	log.Logger = zerolog.New(os.Stdout)

	log.Print("starting gutenbuild")

	pipeline_content_bytes, err := ioutil.ReadFile("./pipeline.toml")
	if err != nil {
		log.Error().Err(err).Msg("error reading pipeline.toml")
	}

	pipeline_content := string(pipeline_content_bytes)

	var pipeline runtime.Pipeline
	_, err = toml.Decode(pipeline_content, &pipeline)
	if err != nil {
		log.Error().Err(err).Msg("error decoding pipeline.toml")
	}

	pipeline.Run()
}
