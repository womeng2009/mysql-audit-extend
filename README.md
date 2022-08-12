# mysql-audit-extend

#### 1.Introduce
Notice: `This is a binary application.`

The extension of mysql-audit, the mysql audit plugin for mcafee, enhances the plugin function.

#### 2.Install
```shell
cargo install mysql-audit-extend
```
or
```shell
wget https://gitee.com/seeker_rs/mysql-audit-extend/releases/download/release-0.2.5/mysql-audit-extend
chmod 775 mysql-audit-extend
```

#### 3.Run
```shell
# View help
mysql-audit-extend -h

OUTPUT:
USAGE:
    mysql-audit-extend [OPTIONS]

OPTIONS:
    -f, --max-file <MAX_FILE>    Maximum number of files to keep [default: 10]
    -h, --help                   Print help information
    -m, --max-size <MAX_SIZE>    Maximum file size, Unit: MB [default: 10]
    -p, --path <PATH>            Absolute path to log file [default:
                                 /var/lib/mysql/mysql-audit.json]
    -V, --version                Print version information


# Run with default configuration
mysql-audit-extend

# Run with custom configuration
mysql-audit-extend -p /var/lib/mysql/mysql-audit.json -m 100 -f 30
```

#### 4.Support platform
> Currently only supports Linux operating system.

* Linux