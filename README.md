# parallel_untar
This is a small tool for extracting tons of `tar` files, which is tedious but also somehow sophisticated, because without care, parallelization going wild will eat all resources while giving garbage after a long run, and (no offense) shell scripts are ugly.

The intention of this tool is so, but you can for sure customize the code to execute commands with controllable parallelization, since the code is not long and with comments.
## Compilation & Usage
To compile, make sure you have the latest Rust and run
```shell
cargo build --release
```
To use it, you can run it via `cargo` with `cargo run --release <tar_file_dir> <target_dir>`.

Or, you can just copy and run the binary at `target/release/parallel_untar`.

### Example
For example, I have
```
dataset/
    parallel_untar
    tars/
        a.tar
        b.tar
    images/
```
After `cd dataset`, just run 

```
./parallel_untar "./tars/*.tar" "./images"
```
Then we will have
```
dataset/
    parallel_untar
    tars/
        a.tar
        b.tar
    images/
        a/
            image1.jpg
            image2.jpg
        b/
            etc.jpg
```

## Performance
In case you are not familiar with drives etc., simply increasing the number of threads may not give you better performance,

like `rayon::ThreadPoolBuilder::new().num_threads(64).build_global().unwrap();`

since the speed of your drive is very likely to be the bottleneck unless you don't use parallelization at all.