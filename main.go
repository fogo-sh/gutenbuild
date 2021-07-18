package main

import (
	"flag"
	"io/ioutil"
	"os"

	"github.com/BurntSushi/toml"
	"github.com/fogo-sh/gutenbuild/runtime"
	"github.com/rs/zerolog"
	"github.com/rs/zerolog/log"
)

func main() {
	zerolog.TimeFieldFormat = zerolog.TimeFormatUnix
	should_pretty_logs := flag.Bool("pretty", false, "pretty print logs")
	flag.Parse()
	if *should_pretty_logs {
		log.Logger = log.Output(zerolog.ConsoleWriter{Out: os.Stdout})
	} else {
		log.Logger = zerolog.New(os.Stdout)
	}

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

	err = pipeline.Run()
	if err != nil {
		log.Error().Err(err).Msg("Error running pipeline.")
		os.Exit(1)
	}
}
