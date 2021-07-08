package runtime

import (
	"fmt"
	"io/ioutil"
	"os"
	"path/filepath"

	"github.com/bytecodealliance/wasmtime-go"
	"github.com/rs/zerolog/log"
)

func (p *Pipeline) Run() error {
	log.Print("setting up store and engine")

	engine := wasmtime.NewEngine()
	store := wasmtime.NewStore(engine)

	log.Print("creating temp directory")
	dir, err := ioutil.TempDir("", "out")
	if err != nil {
		return fmt.Errorf("error creating temp directory: %w", err)
	}
	defer os.RemoveAll(dir)
	stdoutPath := filepath.Join(dir, "stdout")
	stderrPath := filepath.Join(dir, "stderr")

	for _, stage := range p.Stages.Stage {
		log.Print("processing ", stage.ModuleName)

		stage_module := p.Modules[stage.ModuleName]
		wasm_bytes, err := ioutil.ReadFile(stage_module.Path)
		if err != nil {
			return fmt.Errorf("couldn't read module for %s: %w", stage.ModuleName, err)
		}

		log.Print(stage.ModuleName)

		module, err := wasmtime.NewModule(store.Engine, wasm_bytes)
		if err != nil {
			return fmt.Errorf("couldn't compile module for %s: %w", stage.ModuleName, err)
		}

		linker := wasmtime.NewLinker(engine)
		err = linker.DefineWasi()
		if err != nil {
			return fmt.Errorf("couldn't compile module for %s: %w", stage.ModuleName, err)
		}

		wasiConfig := wasmtime.NewWasiConfig()
		wasiConfig.SetStdoutFile(stdoutPath)
		wasiConfig.SetStderrFile(stderrPath)
		for _, volume := range stage.Volumes {
			wasiConfig.PreopenDir(volume.Host, volume.Guest)
		}

		store.SetWasi(wasiConfig)
		instance, err := linker.Instantiate(store, module)
		if err != nil {
			return fmt.Errorf("couldn't compile module for %s: %w", stage.ModuleName, err)
		}

		nom := instance.GetExport(store, "_start").Func()
		_, err = nom.Call(store)
		if err != nil {
			return fmt.Errorf("%s: %w", stage.ModuleName, err)
		}

		stdout, err := ioutil.ReadFile(stdoutPath)
		if err != nil {
			return fmt.Errorf("couldn't read stdout file for module %s: %w", stage.ModuleName, err)
		}
		stderr, err := ioutil.ReadFile(stderrPath)
		if err != nil {
			return fmt.Errorf("couldn't read stderr file for module %s: %w", stage.ModuleName, err)
		}

		if len(stdout) != 0 {
			log.Info().Str("stdout", string(stdout)).Msg(stage.ModuleName)
		}
		if len(stderr) != 0 {
			log.Error().Str("stderr", string(stderr)).Msg(stage.ModuleName)
		}
	}
	return nil
}
