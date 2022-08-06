## Solution

* * *

#### Approach 1: Pig as a [qubit](https://en.wikipedia.org/wiki/Qubit)

**Intuition**

These "strange" questions are now asked by Google, Baidu and IBM because of their interest in quantum computing. [Quantum bit](https://en.wikipedia.org/wiki/Qubit) (or _qubit_) is the basic unit of quantum information, it's the quantum version of the classical binary bit. Binary bit has only two states : `0` and `1`, and on a very basic level the qubit has more. In such questions we deal with an object (here is a pig) which has more than two states.

**How many states does a pig have**

If there is no time to test, i.e. `minutesToTest / minutesToDie = 0`, the pig has only one state - `alive`.

If `minutesToTest / minutesToDie = 1` then the pig has a time to die from the poison, that means that now there are two states available for the pig : `alive` or `dead`.

One more step. If `minutesToTest / minutesToDie = 2` then there are three available states for the pig : `alive` / `dead after the first test` / `dead after the second test`.

![bla](https://leetcode.com/problems/poor-pigs/Figures/458/pigs.png)

> The number of available states for the pig is `states = minutesToTest / minutesToDie + 1`.

**How many buckets could test `x` pigs with 2 available states**

One pig could test 2 buckets - let's make him drink from the bucket number `1` and then wait `minutesToDie` time. If he is alive - the poison is in the bucket number `2`. If he is dead - the poison is in the bucket number `1`.

![bla](https://leetcode.com/problems/poor-pigs/Figures/458/pigs_bucket.png)

The same way two pigs could test 22\=42^2 = 422\=4 buckets

![bla](https://leetcode.com/problems/poor-pigs/Figures/458/2_pigs.png)

![bla](https://leetcode.com/problems/poor-pigs/Figures/458/2_pigs_results.png)

> Hence if one pig has two available states, `x` pigs could test 2x2^x2x buckets.

**How many buckets could test `x` pigs with `s` available states**

> After the discussion above, the answer is quite obvious : sxs^xsx buckets.

Let's consider as an example one pig with 3 states, _i.e._ `s = minutesToTest / minutesToDie + 1 = 2 + 1 = 3`, and show that he could test `3` buckets.

![bla](https://leetcode.com/problems/poor-pigs/Figures/458/1_pig_2_tries.png)

![bla](https://leetcode.com/problems/poor-pigs/Figures/458/1_pig_results.png)

**Solution**

Hence the problem is to find `x` such that statesx≥buckets\\textrm{states}^x \\ge \\textrm{buckets}statesx≥buckets, where `x` is a number of pigs, `states = minutesToTest / minutesToDie + 1` is a number of states available for each pig, and buckets\\textrm{buckets}buckets is number of buckets.

[The solution is well known](https://en.wikipedia.org/wiki/Exponential_function) : x≥log⁡statesbucketsx \\ge \\log\_{\\textrm{states}}{\\textrm{buckets}}x≥logstates​buckets. To simplify the code let's rewrite the equation with the help of [natural logarithms](https://en.wikipedia.org/wiki/List_of_logarithmic_identities#Changing_the_base) :

x≥log⁡bucketslog⁡states x \\ge \\frac{\\log \\textrm{buckets}}{\\log \\textrm{states}} x≥logstateslogbuckets​

**Implementation**

**Complexity Analysis**

*   Time complexity : O(1)\\mathcal{O}(1)O(1) since it's a [constant time solution](https://stackoverflow.com/a/7317571/7775414).
*   Space complexity : O(1)\\mathcal{O}(1)O(1).