rnd - A program that accepts new-line delimited text over stdin and
randomly chooses one of the items or shuffles the entire list.

# Examples

Random choice from input:

```
echo "one
two
three" | target/debug/rnd
two
```

Shuffle entire list:

```
echo "one
two
three" | target/debug/rnd -s
two
one
three
```

