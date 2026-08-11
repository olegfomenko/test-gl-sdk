# Test Greenlight SDK

Following official documentation: <https://blockstream.github.io/greenlight/getting-started/>

For new node receive the certificate and run:
```rust
gen_seed().unwrap();
```
and save the result to `mnemonic.txt` and `seed.txt` files. Then export 
```shell
export GL_CRT_PATH=./gl-certs/client.crt
export GL_KEY_PATH=./gl-certs/client-key.pem
export GL_SEED_PATH=./hsm/seed.txt
export GL_CRED_PATH=./hsm/credentials.gfs
```
and run
```rust
gl_init().await.unwrap();
```
to register a new node. Finally, run 
```rust
gl_connect().await.unwrap();
```
to connect and execute `getinfo` and `invoice` commands. 

