# rupick
random pickup tool from stdin/file

## usage

    rupick [OPTIONS] [file]

### positional arguments:
|argument|description|
|---|---|
|*file*|source filepath|

### optional arguments:
|argument|description|
|---|---|
|-h,--help|show this help message and exit|
|-n *N*|number of choice(default *N*=1)|
|-d|deny duplicates|


## example

    $ cat << _EOB_ | rupick -n 10
    > 1
    > 2
    > 3
    > 4
    > 5
    > 6
    > _EOB_
    4
    1
    1
    1
    6
    3
    1
    1
    4
    4


    $ cat << _EOB_ > sample
    > 1
    > 2
    > 3
    > 4
    > 5
    > 6
    > _EOB_
    $ rupick -d -n 10 sample
    5
    1
    2
    6
    4
    3