This is a CLI tool that will take a file which lists items and assigned weights
to those items. Then randomly choose and print an item based on the weights
given.

# Building for your own target

```
cargo build --release
```

# Usage

```
./decision_maker --file weighted_list_path
```

# Creating the weighted list

The weighted list will be a file of key value pairs, separated by an `=`.

The only accepted format is as follows:

```
item_one = 100
item_two = 50
```

In this case, `item_one` will have a 2/3 chance of being chosen. Whereas
`item_two` will have a 1/3 chance.

If you wish to display the weights of an existing list, you can run the program
with the `-g` flag.
