default:
    zellij run --close-on-exit -- cargo watch -w src -x "run -- sample.csv ';'"
