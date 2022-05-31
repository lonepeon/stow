package internal

type Path string

func (p Path) String() string {
	return string(p)
}
