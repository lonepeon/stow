package build

import "fmt"

type Version struct {
	Branch  string
	Commit  string
	State   string
	Version string
}

var (
	branch  = "nobranch"
	commit  = "HEAD"
	state   = "dirty"
	version = "unknown"
)

func (v Version) String() string {
	return fmt.Sprintf("version=%s branch=%s commit=%s state=%s", v.Version, v.Branch, v.Commit, v.State)
}

func GetVersion() Version {
	return Version{
		Branch:  branch,
		Commit:  commit,
		State:   state,
		Version: version,
	}
}
