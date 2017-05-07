# rupick

## usage

    rupick [OPTIONS] [file]

### positional arguments:
#### file
source filepath

### optional arguments:
#### -h,--help
show this help message and exit
#### -n
number of choice

## example

    cat << _EOB_ | rupick -n 2
    > 1
    > 2
    > 3
    > 4
    > 5
    > 6
    > _EOB_
    2
    5