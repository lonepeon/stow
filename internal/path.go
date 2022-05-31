package internal

import "path"

type Path string

func (p Path) String() string {
	return string(p)
}

func (p Path) Join(other Path) Path {
	return Path(path.Join(p.String(), other.String()))
}
}
