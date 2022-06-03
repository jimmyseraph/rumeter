# RuMeter-CLI

A simple CLI tool for load test.

## Usage

- Download the lastest release from [here][releases]

[releases]: https://github.com/jimmyseraph/rumeter/releases

- Run rumeter in command line. 

## Options

You can run `rumeter http -h` to get all options:
```Shell
$ rumeter http -h
Help - http
Run http protocol load test.

    eg: rumeter http -m post -u http://127.0.0.1/api/login -H Content-Type=application/json -b "\{\"username\": \"liudao\", \"password\":\"123456\"\}" -n 10 -c 10 -l demo.rtl

ARGUMENTS:
    -m, --method        Request method, default is GET. [optional]
    -u, --url   Request url.
    -H, --headers       Request header, split with '::'. eg: Content-Type=application/json::User-Agent=Mozilla/5.0      [optional]
    -b, --body  Request body    [optional]
    -n, --number        Thread number
    -c, --count Loop count, if duration is available, this option will be ignored.      [optional]
    -d, --duration      Runing duration, in seconds.    [optional]
    -l, --log   Specify Rumeter test log file
```

## Example
```Shell
$ rumeter http -m post -u http://127.0.0.1:8088/sayhi -H Content-Type=application/json -b "\{\"name\": \"liudao\", \"age\":18\}" -n 10 -c 10 -l demo.rtl
```