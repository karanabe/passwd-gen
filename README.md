# Password generator


## How to build

```shell
make build
cp ./target/x86_64-unknown-linux-musl/release/passwd-gen /to/your/directory
```


## Usage

```shell
$ passwd-gen -h
Generate passwords automatically

Usage: passwd-gen [OPTIONS]

Options:
  -l, --length <LENGTH>  Length of the password [default: 8]
  -s, --special          Include special characters in the password
  -n, --number <NUMBER>  Number of passwords to generate [default: 20]
  -h, --help             Print help
  -V, --version          Print version

```

```shell
$ passwd-gen
iOT1vdDW        jbW5NdQE        SIM9VQaM        14F5byl7        YXiKb10g
TJQSl8RW        Km9jF5Nr        5CaNfgef        g6YPTLd6        Oo80uHH1
2kowgKNt        JJT25Lgi        XBSeGh1K        neBTOWO8        S1KH4QHd
Vu3d39tV        2QlomEvC        zI08LsrC        AXHlEY3q        2kald6J0
$ passwd-gen -s
p`wdGXi2        <6n#NJ|q        7yCPp{J!        07HqBjg.        3P.b0/:}
YfH*K3lQ        "6wo4Cd%        ;]r?Rge8        5B_qZRu$        #J~3P%b5
[W7Ou7\~        OQyG>|6H        |.Jy=9`X        QF["h-d8        5#B1b=`K
j6z--Sn'        -c-v1BZF        xpW!^TQ9        F=:9j_UQ        rOok:1Xe
$ passwd-gen -l 10
52PWVgqG0D      jeVp3L9AGd      5QO2qJg9Ge      wqFP9xaZdw      paR7kLjMoB
DOki9grzqg      l66wOjCdR3      GmJ7HTkVPb      8vlR38Uhnx      RkadDBZB3P
TEe75gTWeX      MwC8mRvwd6      ImdP8GtOOc      JjeRp8ncJ3      QJqMHav1zU
txaDeoDxx7      PmRoRUy4pW      HrHdOf3Lho      T6cFD03Pd2      FFlGj7zyie
$ passwd-gen -n 10
XIE8XTSe        4pZGhVn8        yGqqz9IY        YmzeJo10        67irl18R
9qf5Bne1        IWlq188a        S21uhuk1        2eH7Wpis        p8ZBo98m
```


### License

<sup>
Licensed under either of <a href="LICENSE-APACHE">Apache License, Version 2.0</a> or <a href="LICENSE-MIT">MIT license</a> at your option.
</sup>

<br>

<sub>
Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
</sub>
