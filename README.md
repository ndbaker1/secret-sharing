# Shamir's Secret Sharing

## Details 
Secret Sharing implementation in Rust using `num-bigint` to process very large
numbers and uses a default `512-bits` of entropy for secret-curve coefficients.

See references for:
- [Secret Sharing Scheme implementation example for Unix Systems](http://point-at-infinity.org/ssss/)
- [Lagrange Interpolation Polynomial](https://en.wikipedia.org/wiki/Lagrange_polynomial)

## Usage

Example using a (3,5)-threshold scheme, where any 3 of 5 share holders can
decrypt the message.
```bash
❯ shamir-split -t 3 -n 5
Generating shares using a (3,5) scheme with dynamic security level.
Enter the secret: secret text
1-119e3fe075e9f332f37f6be6bc397b0069bcc629d5ee5bf2a6bd4658ffecb3e241d0d9884303fcdff1d90b82ffa1da8fcec0d34cb71862d26b089ba22132d4a23
2-40e450ac8c3b0b067d3880a650cd0f042069c9217fb8aa8dcccad0be741c00792c1c713cca2d65479f0a0e7303c0082d2cac6dd7365eed219306bdcd85bd3a122
3-8dd2326442f3477a9d2b3e3ebdbabc0b240708e6fd5eebd172289f305c8de5c4bee2c71d957c3937079308d00c5a88d819c2cf9f7ddad543af20bdc434e587d71
4-f867e5079a12a88f5357a4b0030282157494857a4ee11fbd96d6b1aeb94263c4fa23db2aa4f078ae2b73fa9a19715c909603f8a58d8c1b38bf569b862eabbdf10
5-180a5689691992e449fbdb3fa20a4612312123edb743f46523ad508398a397a79dddfad63f88a23ad0aace3d12b048356a16fe8e96572bf00c3a85713730fdc5ff
```

Example recovering a secret using 3 of the required shares.
```bash
❯ shamir-combine -t 3
Enter 3 shares separated by newlines:
Share [1/3]: 1-119e3fe075e9f332f37f6be6bc397b0069bcc629d5ee5bf2a6bd4658ffecb3e241d0d9884303fcdff1d90b82ffa1da8fcec0d34cb71862d26b089ba22132d4a23
Share [2/3]: 2-40e450ac8c3b0b067d3880a650cd0f042069c9217fb8aa8dcccad0be741c00792c1c713cca2d65479f0a0e7303c0082d2cac6dd7365eed219306bdcd85bd3a122
Share [3/3]: 4-f867e5079a12a88f5357a4b0030282157494857a4ee11fbd96d6b1aeb94263c4fa23db2aa4f078ae2b73fa9a19715c909603f8a58d8c1b38bf569b862eabbdf10
Resulting secret: secret text
```

