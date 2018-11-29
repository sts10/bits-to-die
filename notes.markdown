
https://www.reddit.com/r/Passwords/comments/8ca2wg/security_of_a_diceware_passphrase_where_some/?st=jg060aut&sh=8f03daf6


Accidentally typed two words of my Diceware passphrase into a browser the other day. It probably didn't get sent anywhere but I figure it's better to be safe than sorry, so I created an entirely new phrase. But I'm interested in what the theoretical security of the original phrase is, even if we assume those two words are compromised.

Specifically, it's a 7 word Diceware phrase (although an adversary would not know this). And I typed words 4 and 5 into the browser, so an adversary would know that those two words are together, but not that they are words 4+5. As far as the adversary would know, they could be words 1+2, 2+3, 3+4 etc. All he would know is that those two words form part of the passphrase, and they are grouped together. What sort of entropy would that passphrase have compared to the original 7 words when they are all unknown?



---
Interesting question! Here's my guess:

First, let's assume your attacker knows it's a 7-word phrase. I think this is standard practice when evaluating the entropy of diceware passphrases.

There are six possible position for your two words that the attacker knows. You have 5 words that are still unknown to the attacker. Thus the attacker has to guess every possible 5-word passphrase 6 times.

A straight-forward 5-word diceware passphrase (using a pool of 7776 words) has an entropy of 64.62 bits (`log_base2(7776^5)`). I _think_ the fact that your attacker has to check each phrase 6 times means that we add log_base2(6) to 64.62 bits, giving us  64.64+2.58 = 67.20 bits. Conclusion: your sort-of compromise 7-word phrase is not much stronger than a 5-word passphrase. 

This is my understanding of entropy. I'm no expert though. Fascinating question. 
