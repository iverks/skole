$pyfile = $args[0]
& "maturin" @("develop", "-m", "smumerix/Cargo.toml")
& "py" @("_python/" + $pyfile)