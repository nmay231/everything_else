function splitArray(nums: number[], k: number): number {
    let optimalSplit = 0;
    for (const n of nums) {
        optimalSplit += n;
    }

    optimalSplit = Math.max((optimalSplit / k) | 0, ...nums);
    let upperBound = -1;
    const sums = Array.from<number>({length: k}).fill(0);
    const splits = [] as number[];
    for (const i in nums) {
        const n = nums[i];
        sums[splits.length] += n;
        const enoughNumsLeft = nums.length - (+i) > k - splits.length;
        if (sums[splits.length] >= optimalSplit || !enoughNumsLeft) {
            upperBound = Math.max(upperBound, sums[splits.length])
            splits.push(+i);
        }
    }
    console.log(sums, splits, optimalSplit, upperBound)
    if (splits.length < k - 1) {
        throw Error("Welp... Off by one error")
    }

    let lowerBound: number;
    outer: for (lowerBound = optimalSplit; lowerBound < upperBound; lowerBound++) {
        for (let i = k - 1; i > 0; i--) {
            let endOfLeftArray: number;
            // 
            while ([console.log(i, sums, splits, lowerBound)] && sums[i] + nums[endOfLeftArray = splits[i - 1]] <= lowerBound && endOfLeftArray >= i) {
                sums[i] += nums[endOfLeftArray];
                sums[i - 1] -= nums[endOfLeftArray];
                splits[i - 1] -= 1;
            }

            if (sums[i] > lowerBound) {
                continue outer;
            }
        }
        if (sums[0] <= lowerBound) {
            console.log("inner", sums, splits, optimalSplit, upperBound);
            return lowerBound;
        }
    }
    console.log("outer", sums, splits, optimalSplit, upperBound)
    return lowerBound;

    // let one = -1;
    // while (true) {
    //     let minmaxAttempt = -1;
    //     const entries = Object.entries(splits);
    //     if (one === -1) {
    //         entries.reverse();
    //     }
    //     for (const [i, index] of entries) {
            
    //     }
    //     one = -one;
    // }
};

console.log("expect 140:", splitArray([1,60,25,25,1,1,30,110,40,70,37], 4))
// console.log("expect 140:", splitArray([25, 25, 30, 50, 40, ], 4))
