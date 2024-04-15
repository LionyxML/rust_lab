# _07_quickreplace


## Prepare a file with some content

``` shell
echo "Hello There!" > test_file.txt
```

## Run:

``` shell
cargo run "There" "Rust" test_file.txt test_file_modified.txt
```

## Compare:

``` shell
diff test_file.txt test_file_modificed.txt
```


