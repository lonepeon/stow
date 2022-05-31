package internal

type Bundle struct {
	name  string
	files []Path
}

func NewBundle(name string) *Bundle {
	return &Bundle{
		name:  name,
		files: nil,
	}
}

func (b *Bundle) Name() string {
	return b.name
}

func (b *Bundle) AddFile(p Path) {
	b.files = append(b.files, p)
}

func (b *Bundle) Files() []Path {
	return b.files
}
