# dnoted

Leave messages for yourself in directories.

```
# cd ~/Documents
~/Documents
/Users/axjns/Documents: I must remember to delete the .crap folder
```

## Installation 

### From Source

Assuming you have Rust installed, you can build and install the binary from source.
```
cargo install --path .
```

Once youve done this, you'll need to add the following to your `.bashrc` or `.zshrc` file:
```
echo 'dnoted check' >> ~/.zshrc
```
Reload your shell and you should be good to go.