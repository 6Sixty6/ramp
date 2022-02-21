<h1 align="center">
    <img src="https://user-images.githubusercontent.com/86065741/155012868-6ca9b274-87f2-4c57-b4d8-7860e41a64e3.png" width="150px"><br>
    RAMP - port scanner
</h1>

<p align="center">
    ramp a port scanning tool written in rust 🦀 scans port in just seconds without missing any of them
</p>

<p align="center">
    	<img src="https://img.shields.io/badge/Version-1.0.0-A897F9?style=for-the-badge" alt="Version">
</p>

## Usage
`ramp -h`
```
USAGE:
    ramp [FLAGS] [OPTIONS] <target>

FLAGS:
    -f, --full       Scan all 65535 ports
    -h, --help       Prints help information
    -V, --version    Prints version information
    -v, --verbose    Display detailed information

OPTIONS:
    -c, --concurrency <concurrency>    Concurrency [default: 1002]
    -t, --timeout <timeout>            Connection timeout [default: 3]

ARGS:
    <target>    The target to scan
```

## Speed
```
> time ./ramp -v hackerone.com

▄▄▄   ▄▄▄· • ▌ ▄ ·.  ▄▄▄·
▀▄ █·▐█ ▀█ ·██ ▐███▪▐█ ▄█
▐▀▀▄ ▄█▀▀█ ▐█ ▌▐▌▐█· ██▀·
▐█•█▌▐█ ▪▐▌██ ██▌▐█▌▐█▪·•
.▀  ▀ ▀  ▀ ▀▀  █▪▀▀▀.▀
    ramp by SixSyxtySix
    https://github.com/6Sixty6/ramp
---------------------------------

Scanning common 1002 ports of hackerone.com. Concurrency: 1002. Timeout: 3
53
80
443
8080
8443

real	0m3.052s
user	0m0.019s
sys	    0m0.038s
```

## Build
- note: rust has to be installed
```sh
git clone https://github.com/6Sixty6/ramp.git
cd rrep
cargo build
```

## Credits
```
https://github.com/0xlilith
```
## Contributions 🎉
##### Any contributions are accepted through PR