my configuration for `squid` based caching server to prevent downloading RPM packages over and over again while playing around with virtual machines.

store_id_program
--------

using only the filename instead of the complete URL i can use alternate mirrors and still hit the cache, even if the RPM came originally from a different mirror.

Build:

    cd store-id && cargo build --release

squid.conf
--------

i use the default configuration with the exception of refresh_patterns, which i replaced with my own:

    #                 3 month    12 month
    refresh_pattern . 129600 33% 525600

plus the additional configuration:

    cache_dir ufs /var/spool/squid 10000 16 256

    store_id_program /path/to/store-id
    store_id_children 5 startup=1

    maximum_object_size 3 GB

    # cache RPMs DEBs ISOs only
    acl mypackages urlpath_regex \.rpm
    acl mypackages urlpath_regex \.deb
    acl mypackages urlpath_regex \.iso
    cache allow mypackages
    cache deny all
