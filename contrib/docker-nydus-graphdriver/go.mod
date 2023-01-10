module github.com/dragonflyoss/image-service/contrib/nydus_graphdriver

go 1.17

require (
	github.com/docker/docker v20.10.3-0.20211206061157-934f955e3d62+incompatible
	github.com/docker/go-plugins-helpers v0.0.0-20211224144127-6eecb7beb651
	github.com/moby/sys/mountinfo v0.5.0
	github.com/opencontainers/selinux v1.10.1
	github.com/pkg/errors v0.9.1
	github.com/sirupsen/logrus v1.8.1
	golang.org/x/sys v0.0.0-20220722155257-8c9f86f7a55f
)

require (
	github.com/Microsoft/go-winio v0.5.2 // indirect
	github.com/containerd/containerd v1.6.12 // indirect
	github.com/containerd/continuity v0.3.0 // indirect
	github.com/coreos/go-systemd v0.0.0-20191104093116-d3cd4ed1dbcf // indirect
	github.com/docker/distribution v2.8.1+incompatible // indirect
	github.com/docker/go-connections v0.4.0 // indirect
	github.com/docker/go-units v0.4.0 // indirect
	github.com/gogo/protobuf v1.3.2 // indirect
	github.com/golang/protobuf v1.5.2 // indirect
	github.com/klauspost/compress v1.11.13 // indirect
	github.com/moby/sys/mount v0.3.0 // indirect
	github.com/moby/sys/symlink v0.2.0 // indirect
	github.com/opencontainers/go-digest v1.0.0 // indirect
	github.com/opencontainers/image-spec v1.0.3-0.20211202183452-c5a74bcca799 // indirect
	github.com/opencontainers/runc v1.1.2 // indirect
	github.com/opencontainers/runtime-spec v1.0.3-0.20210326190908-1c3f411f0417 // indirect
	github.com/vbatts/tar-split v0.11.1 // indirect
	golang.org/x/net v0.0.0-20220722155237-a158d28d115b // indirect
	google.golang.org/genproto v0.0.0-20220502173005-c8bf987b8c21 // indirect
	google.golang.org/grpc v1.47.0 // indirect
	google.golang.org/protobuf v1.28.0 // indirect
)

replace (
	github.com/containerd/go-runc => github.com/containerd/go-runc v1.0.0
	github.com/docker/distribution => github.com/docker/distribution v2.8.1+incompatible
	github.com/opencontainers/image-spec => github.com/opencontainers/image-spec v1.0.2
	github.com/opencontainers/runc => github.com/opencontainers/runc v1.1.2
)
