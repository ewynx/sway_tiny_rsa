# Sway Practice | Tiny RSA (as in tiny numbers to practice with)

The [rsa example](https://ccom.uprrp.edu/~humberto/very-small-rsa-example.html). 

## Run tests
```
forc build
forc test
```

Note that using the standard power function will not work for all messages, therefore we use a power mod function that reduces mod n for every step. 