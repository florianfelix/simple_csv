default:
    zellij run --close-on-exit -- cargo watch -w src -w dataframe/src -x "run -- sample.csv"
