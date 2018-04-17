# Bits to Die

Converts entropy measured in bits to "die" (number of dice rolls). 

## What this does / Use case

You want to get a feel for how hard it is to "guess" a non-diceware password with dice. You know the password is roughly X bits of entropy. This Rust script takes that X bits and converts in to a different (and completely made up) measure of entropy that I'm calling "die". It represents the number of 6-sided die it would theoretically (I think) take to reproduce that password exactly. In other words it estimates the length of the password encoded in base 6.

## Example

`@[5k[ybL8%\/E~A` has about 87.98 bits of entropy, according to KeePassXC's random password generator (which I believe uses [zxcvbn](https://github.com/dropbox/zxcvbn) for measuring entropy). Entering `87.98` bits into this program will inform you that that's equivalent to about 34.04 "die" of entropy.

Using a dictionary of 7776 words, `probiotic-troubling-breath-salaried-plank-docile` has an entropy of 77.55 bits. This script reports that that is equivalent to 30 die, which is in fact the number of rolls you'd need to generate a 6-word passphrase using the diceware method.

## Disclaimers

I'm reading [_An Introduction to Information Theory: Symbols, Signals and Noise_ by John R. Pierce](https://www.amazon.com/Introduction-Information-Theory-Symbols-Mathematics/dp/0486240614) and thought this would be a good exercise to further my understanding of entropy, passwords, and Rust. I don't know if this is scientifically accurate or useful, so, you know, don't rely on it beyond a curiosity. 


