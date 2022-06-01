package internal

type Walker func(Path, WalkerFunc) error

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

func BuildBundle(root Path, bundleName string) (*Bundle, error) {
	return BuildBundleWithWalker(root, bundleName, WalkFileSystem)
}

func BuildBundleWithWalker(root Path, bundleName string, walker Walker) (*Bundle, error) {
	bundle := NewBundle(bundleName)

	bundlePath := root.Join(Path(bundleName))
	err := walker(bundlePath, func(path Path) { bundle.AddFile(path) })

	return bundle, err
}
