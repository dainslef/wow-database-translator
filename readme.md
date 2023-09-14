# WOW Database Translator
It's an application to translate WOW locale database tables between `zhTW` and `zhCN` by OpenCC.
It supports table structures for Azerothcore and MaNGOS.

```
WOW database translator
It's an application to translate WOW locale database tables between zhTW and zhCN.

Usage: wow-database-translator [OPTIONS]

Options:
      --host <HOST>
          Set the database target address

          [default: 127.0.0.1]

      --port <PORT>
          Set the database target port

          [default: 3306]

  -u, --username <USERNAME>
          Set the database login username

          [default: root]

  -p, --password <PASSWORD>
          Set the database login password

          [default: password]

  -b, --batch-size <BATCH_SIZE>
          Set the data batch size

          [default: 1000]

  -a, --async
          Enable async execute

  -c, --check
          Run database translation check

  -t, --translate
          Execute database translate

  -s, --server-type <SERVER_TYPE>
          Set up the server type

          [default: azeroth-core]
          [possible values: mangos0, mangos2, azeroth-core]

  -l, --log <LOG>
          Set the log level filter

          [default: info]

  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version
```
