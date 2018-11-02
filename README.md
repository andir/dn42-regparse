# Regparse

Regparse consists of two parts, a library and multiple application binaries.


## Compilation

To compile everything just run the following:

    cargo build --release

## Binaries

In this section i'll describe all the application binaries which can be found in `src/bin/`.

### main

A "testing" / "debug" binary which is mostly used for parser validation since it just parses
the entire registry and then exits.

### roagen

A small application that parses the routes and prints them in the config syntax for
ROA validation by bird (2.x - support for bird 1.x can be achieved by piping the output
through `sed 's/route/roa/g'`).

Example output:

    route 10.25.0.0/16 max 29 as 64858;
    route 172.20.4.128/26 max 29 as 4242420049;
    route 172.31.0.200/32 max 32 as 64654;
    route 172.31.0.200/32 max 32 as 4242422718;
    route 172.31.0.200/32 max 32 as 4242422480;
    route 172.22.141.0/28 max 29 as 64737;
    route 10.23.0.0/16 max 29 as 65210;
    route 172.23.245.0/24 max 29 as 76175;
    route 172.23.177.64/26 max 29 as 4242421987;
    route 172.20.4.64/27 max 29 as 4242422684;
    [...]
    route fdfc:3e4f:f3c0::/48 max 64 as 4242420020;
    route fd42:830:420::/48 max 64 as 4242420203;
    route fd42:4242:23::/48 max 64 as 4242420510;
    route fd42:2950:202::/48 max 64 as 4242422950;
    route fd42:2950:202::/48 max 64 as 202265;
    route fd14:b4dc:4b1e::/64 max 64 as 65052;
    route fdec:c0f1:afda::/64 max 64 as 65115;
    route fda0:bbe1:38d::/48 max 64 as 4242420160;
    route fdfe:1647:a2bb::/48 max 64 as 4242421978;
    route fd23:dead:beef::/48 max 64 as 65432;

### roasrv

roasrv serves a similar purpose to roagen, but it's slightly more complex: roasrv in conjunction with CloudFlares gortr can be used to perform ROA validation using the RPKI-to-Router protocol. For this, roasrv performs the following tasks:

* roasrv will regularly pull updates to a local (git) copy of the dn42 registry, and re-parse it if it detects a change
* The parsed data is then converted into a json string which is made accessible to gortr using a builtin webserver

See the Yamakaja/roasrv-pkg repository for instructions on how to setup roasrv with gortr

Example data:

```json
{
  "roas": [
    {
      "prefix": "10.0.0.0/16",
      "maxLength": 29,
      "asn": "AS65079"
    },
    {
      "prefix": "10.1.0.0/17",
      "maxLength": 29,
      "asn": "AS65530"
    },
    {
      "prefix": "fdff:feed:c0de::/48",
      "maxLength": 64,
      "asn": "AS4242420138"
    },
    {
      "prefix": "fd42:c066:e081::/48",
      "maxLength": 64,
      "asn": "AS4242420092"
    }
  ]
}
```
