package runtime

import (
	"fmt"
	"io/ioutil"
	"os"
	"path/filepath"

	"github.com/bytecodealliance/wasmtime-go"
)

func check(e error) {
	if e != nil {
		panic(e)
	}
}

func (p *Pipeline) Run() {
	engine := wasmtime.NewEngine()
	store := wasmtime.NewStore(engine)
	dir, err := ioutil.TempDir("", "out")
	check(err)
	defer os.RemoveAll(dir)
	stdoutPath := filepath.Join(dir, "stdout")

	for _, stage := range p.Stages.Stage {
		wasm_bytes, err := ioutil.ReadFile(p.Modules[stage.ModuleName].Path)
		check(err)

		module, err := wasmtime.NewModule(store.Engine, wasm_bytes)
		check(err)

		linker := wasmtime.NewLinker(engine)
		err = linker.DefineWasi()

		check(err)

		wasiConfig := wasmtime.NewWasiConfig()
		wasiConfig.SetStdoutFile(stdoutPath)

		store.SetWasi(wasiConfig)
		instance, err := linker.Instantiate(store, module)
		check(err)

		nom := instance.GetExport(store, "_start").Func()
		_, err = nom.Call(store)
		check(err)
	}
	out, err := ioutil.ReadFile(stdoutPath)
	check(err)
	fmt.Print(string(out))
}
