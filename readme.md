# WOW Database Translator
It's an application to translate WOW locale database tables between `zhTW` and `zhCN` by OpenCC.
It only supports table structure for Azerothcore currently.

```
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

  -d, --database <DATABASE>
          Set the default database

  -b, --batch-size <BATCH_SIZE>
          Set the data batch size

          [default: 1000]

  -a, --async
          Enable async execute

  -c, --check
          Run database translation check

  -t, --translate
          Execute database translate

  -l, --log <LOG>
          Set the log level filter

          [default: info]

  -h, --help
          Print help information (use `-h` for a summary)

  -V, --version
          Print version information
```
