# Cracker for https://zenertest.bensparks.co.uk/ (PATCHED)

```
Subject: Hacking the Zener Card Test
Body:

Hi Ben and Matt.

I hope it is ok to send this mail to both of you. I thought you both 
would be interested...

After watching the latest video on the Stand-up Maths YouTube channel 
(https://www.youtube.com/watch?v=VwIKKBL4ldQ) I started to investigate 
whether the Zener Card Test at https://zenertest.bensparks.co.uk/ could 
actually be "hacked".

Even though the random number generator might not biased, it clearly 
uses PHP 7.1.0+'s `mt_rand` or `rand` function called as `rand(1, 5)` N 
times in a row starting from the same seed, where N is the total number 
of cards.

By guessing say ~15 of the first cards randomly, the remaining ~10 cards 
can easily be predicted using tools such as 
https://github.com/openwall/php_mt_seed

Here are some of the game ids and scores I managed to get only by using 
my laptop:

2025-09-23 19:13:06 UTC
Game id: 72221
Got 3/15 by guessing.
Cracking seed...
Found seed: 2514918050
Final score: 13/25

2025-09-23 19:14:18 UTC
Game id: 72288
Got 5/15 by guessing.
Cracking seed...
Found seed: 4229918377
Final score: 15/25

2025-09-23 19:15:28 UTC
Game id: 72353
Got 1/15 by guessing.
Cracking seed...
Found seed: 1427307147
Final score: 11/25

2025-09-23 19:16:38 UTC
Game id: 72399
Got 4/15 by guessing.
Cracking seed...
Found seed: 3523626724
Final score: 14/25

2025-09-23 19:17:49 UTC
Game id: 72452
Got 3/15 by guessing.
Cracking seed...
Found seed: 322286491
Final score: 13/25

2025-09-23 19:19:06 UTC
Game id: 72504
Got 2/15 by guessing.
Cracking seed...
Found seed: 673848177
Final score: 12/25

2025-09-23 19:20:54 UTC
Game id: 72560
Got 3/15 by guessing.
Cracking seed...
Found seed: 3814184517
Final score: 13/25

2025-09-23 19:26:52 UTC
Game id: 72686
Got 4/15 by guessing.
Cracking seed...
Found seed: 3203382320
Final score: 14/25

2025-09-23 19:29:52 UTC
Game id: 72778
Got 2/15 by guessing.
Cracking seed...
Found seed: 451060119
Final score: 12/25

2025-09-23 19:31:37 UTC
Game id: 72848
Got 2/15 by guessing.
Cracking seed...
Found seed: 94187623
Final score: 12/25

2025-09-23 19:32:45 UTC
Game id: 72907
Got 5/15 by guessing.
Cracking seed...
Found seed: 2712729699
Final score: 15/25

2025-09-23 19:33:56 UTC
Game id: 72950
Got 1/15 by guessing.
Cracking seed...
Found seed: 275375272
Final score: 11/25

All my games were played from my IP address <REDACTED> so feel free 
to remove those from your statistics.

Let me know if you would like more details.

Kind regards,
Asger Hautop Drewsen
```
