package runtime

type Volume struct {
	Host  string
	Guest string
}

type Stage struct {
	ModuleName string
	Volumes    []Volume
}

type Module struct {
	Path string
}

type Stages struct {
	Stage []Stage
}

type Pipeline struct {
	Output  string
	Modules map[string]Module
	Stages  Stages
}
