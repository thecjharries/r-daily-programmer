# [3/8/2012] Challenge #20 [easy]

## Source

[Original post](https://old.reddit.com/r/dailyprogrammer/comments/qnkro/382012_challenge_20_easy/) by [u/rya11111](https://old.reddit.com/user/rya11111)

## Prompt
create a program that will find all prime numbers below 2000

## Question

I wonder if the sieve of Eratosthenes would be faster. I like [this Java implementation](https://old.reddit.com/r/dailyprogrammer/comments/qnkro/382012_challenge_20_easy/c3yztec/)

```
public static boolean[] SieveOfEratosthenes(int n)
    {
        boolean[] primes = new boolean[n];
        primes[0] = primes[1] = false;

        for(int i = 2; i < n; i++)
            primes[i] = true;

        for(int i = 2; i * i < n; i++)
            if(primes[i])
                for(int j = 2; i * j < n; j++)
                    if(primes[i * j])
                        primes[i * j] = false;
        return primes;
    }
```

In Go it would probably be a few extra steps.
