# PNGME

The repository contains my implementation of the `pngme` crate for educational purposes.
Source of idea for the project: [pngme_book](https://picklenerd.github.io/pngme_book/introduction.html).

The program has four commands:
1. Encode a message into a PNG file
2. Decode a message stored in a PNG file
3. Remove a message from a PNG file
4. Print a list of PNG chunks that can be searched for messages


## Usage guide:

`pngme encode ./dice.png ruSt "This is a secret message!`

`pngme decode ./dice.png ruSt`

`pngme remove ./dice.png ruSt`

`pngme print ./dice.png`