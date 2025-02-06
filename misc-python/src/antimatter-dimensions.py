# Used for a silly game (Antimatter Dimensions) where one challenge discourages buying
# things at the same price as something else (at the same time, at least). This program
# helped me figure out which upgrade to buy next with out having to do any mental math.
# Though I'll only have to do the challenge once, so it's not very clean code
prices = [7, 10, 14, 12, 17, 23, 30, 39]
costs = [3, 4, 5, 6, 8, 10, 12, 15]
income=43

while True:
    x = input(str([f"{i + 1}: {price}" for i, cost, price in zip(range(0, 100), costs, prices) if price + cost not in prices]))
    if not x.isdigit():
        print("Input not recognized")
        print(prices)
        continue

    inpu = int(x)
    if inpu <= 8:
        index = int(x) - 1
        if prices[index] + costs[index] in prices:
            print("You stupid")
            continue
        income = prices[index] + 1
    else:
        income = inpu

    while any(price < income for price in prices):
        for i, price, cost in reversed([*zip(range(1, 100), prices, costs)]):
            if price + cost not in prices and price < income:
                print(f"{i}={price}", end=" :: ")
                prices[i - 1] += cost
                break
        else:
            break
    print()
    
