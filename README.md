# xc

[root@7cloudos ~]# xc -h
xc 2.0.0
flyZer0 <flyoney@163.com>


USAGE:
    xc [SUBCOMMAND]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    connet    Connect can be used to remotely connect CPE and display the process on the terminal.
    help      Prints this message or the help of the given subcommand(s)
    show      Use show to obtain CPE information and display it on the current terminal
    update    Use update to update local CPE information
[root@7cloudos ~]# xc connet -h
xc-connet 
Connect can be used to remotely connect CPE and display the process on the terminal.

USAGE:
    xc connet [OPTIONS] <sn>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -c <connetmode>        Use this option to select the remote CPE mode. Otherwise, the default version is SSH.
                           [possible values: ssh, telnet, crt, xshell]
    -m <mode>              Use connet to business the CPE,the default version is nexus. [possible values: nexus, valor,
                           watsons, watsons_ha, tassadar]

ARGS:
    <sn>    cpe serial number
[root@7cloudos ~]# xc show -h
xc-show 
Use show to obtain CPE information and display it on the current terminal

USAGE:
    xc show [OPTIONS] <sn>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -m <mode>        Use show to select the CPE, the default version is nexus. [possible values: nexus, valor, watsons,
                     watsons_ha, tassadar]

ARGS:
    <sn>    cpe serial number

