package main

import (
	"gutenbuild/runtime"

	"github.com/BurntSushi/toml"

	"io/ioutil"
)

func main() {
	pipeline_content_bytes, err := ioutil.ReadFile("./pipeline.toml")
	check(err)

	pipeline_content := string(pipeline_content_bytes)

	var pipeline runtime.Pipeline
	_, err = toml.Decode(pipeline_content, &pipeline)
	check(err)

	pipeline.Run()
}

func check(e error) {
	if e != nil {
		panic(e)
	}
}
