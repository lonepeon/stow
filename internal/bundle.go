package internal

type Walker func(Path, WalkerFunc) error

type Package struct {
	name  string
	files []Path
}

func NewPackage(name string) *Package {
	return &Package{
		name:  name,
		files: nil,
	}
}

func (p *Package) Name() string {
	return p.name
}

func (p *Package) AddFile(path Path) {
	p.files = append(p.files, path)
}

func (p *Package) Files() []Path {
	return p.files
}

func BuildPackage(root Path, pkgName string) (*Package, error) {
	return BuildPackageWithWalker(root, pkgName, WalkFileSystem)
}

func BuildPackageWithWalker(root Path, pkgName string, walker Walker) (*Package, error) {
	pkg := NewPackage(pkgName)

	pkgPath := root.Join(Path(pkgName))
	err := walker(pkgPath, func(path Path) { pkg.AddFile(path) })

	return pkg, err
}
