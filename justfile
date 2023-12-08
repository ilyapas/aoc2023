run day part:
    cd {{day}} && cargo run --bin {{part}} --release

test day:
    cd {{day}} && cargo watch -x test 

create day:
    cargo generate --path ./daily-template --name {{day}}
