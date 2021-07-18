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
		log.Debug().Str("stage", "module:reading").Str("module", stage.ModuleName).Send()

		stage_module := p.Modules[stage.ModuleName]
		wasm_bytes, err := ioutil.ReadFile(stage_module.Path)
		if err != nil {
			return fmt.Errorf("couldn't read module for %s: %w", stage.ModuleName, err)
		}

		log.Debug().Str("stage", "module:linking").Str("module", stage.ModuleName).Send()

		module, err := wasmtime.NewModule(store.Engine, wasm_bytes)
		if err != nil {
			return fmt.Errorf("couldn't compile module for %s: %w", stage.ModuleName, err)
		}

		linker := wasmtime.NewLinker(engine)
		err = linker.DefineWasi()
		if err != nil {
			return fmt.Errorf("couldn't compile module for %s: %w", stage.ModuleName, err)
		}

		log.Debug().Str("stage", "module:config").Str("module", stage.ModuleName).Send()

		wasiConfig := wasmtime.NewWasiConfig()
		wasiConfig.SetStdoutFile(stdoutPath)
		wasiConfig.SetStderrFile(stderrPath)
		for _, volume := range stage.Volumes {
			if _, err := os.Stat(volume.Host); os.IsNotExist(err) {
				return fmt.Errorf("host file not found for %s: %s", stage.ModuleName, volume.Host)
			}
			wasiConfig.PreopenDir(volume.Host, volume.Guest)
		}

		store.SetWasi(wasiConfig)
		instance, err := linker.Instantiate(store, module)
		if err != nil {
			return fmt.Errorf("couldn't compile module for %s: %w", stage.ModuleName, err)
		}

		log.Debug().Str("stage", "module:start").Str("module", stage.ModuleName).Send()

		nom := instance.GetExport(store, "_start").Func()
		_, module_error := nom.Call(store)

		stdout, err := ioutil.ReadFile(stdoutPath)
		if err != nil {
			return fmt.Errorf("couldn't read stdout file for module %s: %w", stage.ModuleName, err)
		}
		stderr, err := ioutil.ReadFile(stderrPath)
		if err != nil {
			return fmt.Errorf("couldn't read stderr file for module %s: %w", stage.ModuleName, err)
		}

		if len(stdout) != 0 {
			log.Info().Str("stdout", string(stdout)).Str("module", stage.ModuleName).Send()
		}
		if len(stderr) != 0 {
			log.Error().Str("stderr", string(stderr)).Str("module", stage.ModuleName).Send()
		}

		if module_error != nil {
			return fmt.Errorf("%s: %w", stage.ModuleName, err)
		}

		log.Debug().Str("stage", "module:finished").Str("module", stage.ModuleName).Send()

	}
	return nil
}
