run day part:
    cd {{day}} && cargo run --bin {{part}}

test day:
    cd {{day}} && cargo watch -x test 

create day:
    cargo generate --path ./daily-template --name {{day}}
