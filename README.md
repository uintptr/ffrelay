# install

```
cargo install ffrelay
```

# Commands

## Create

```
ffrelay new -d test
jlhzxuwdz@mozmail.com
```

## List

```
ffrelay ls
┌──────────┬───────────────────────┬─────────────┬─────────────┬───────────────┬─────────────┬──────────┐
│ id       │ full_address          │ description │ num_blocked │ num_forwarded │ num_replied │ num_spam │
├──────────┼───────────────────────┼─────────────┼─────────────┼───────────────┼─────────────┼──────────┤
│ 16320416 │ jlhzxuwdz@mozmail.com │ test        │ 0           │ 0             │ 0           │ 0        │
└──────────┴───────────────────────┴─────────────┴─────────────┴───────────────┴─────────────┴──────────┘
```

## Delete

```
ffrelay rm 16320416
success
```
