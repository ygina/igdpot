# igdpot

Start the IGD honeypot, which is written in [Rust](https://www.rust-lang.org/tools/install).

```
# Use the stable toolchain
rustup default stable

# Run the honeypot
cargo run
```

Test the honeypot using a UPnP client written in Python.

```
# Install dependencies
virtualenv -p python3 env
source env/bin/activate
pip install -r requirements.txt

# Run the client
python client.py
```
