echo:
    ~/Downloads/maelstrom-0.2.3/maelstrom/maelstrom test -w echo --bin ~/code/dist-sys-rust/target/debug/dist-sys-rust --node-count 1 --time-limit 10

uniq:
    ~/Downloads/maelstrom-0.2.3/maelstrom/maelstrom test -w unique-ids --bin ~/code/dist-sys-rust/target/debug/dist-sys-rust --time-limit 30 --rate 1000 --node-count 3 --availability total --nemesis partition

uniq-release:
    ~/Downloads/maelstrom-0.2.3/maelstrom/maelstrom test -w unique-ids --bin ~/code/dist-sys-rust/target/release/dist-sys-rust --time-limit 30 --rate 1000 --node-count 3 --availability total --nemesis partition

broadcast:
    ~/Downloads/maelstrom-0.2.3/maelstrom/maelstrom test -w broadcast --bin ~/code/dist-sys-rust/target/release/dist-sys-rust --node-count 1 --time-limit 20 --rate 10

broadcast-multi:
    ~/Downloads/maelstrom-0.2.3/maelstrom/maelstrom test -w broadcast --bin ~/code/dist-sys-rust/target/release/dist-sys-rust --node-count 5 --time-limit 20 --rate 10
