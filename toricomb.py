attack_types = ["bad"] * 39  + ["bad"] * 40 + ["bottom"] * 40 + ["fire"] * 40 + ["top"] * 39

#attack_types = ["bad", "bad", "bottom", "fire", "top"]
import itertools


# 0 1 2 3 bottom or top with 2 bottoms
# 4 5 6 bottom or top
# 7 fire
# 8 9 bottom or top with 1 bottom
# 10 11 bottom or top
true = True
false = False

goods = 0
totals = 0
import random



def check_good(attacks):
    numbottoms = 0


    for i in range(4):
            if attacks[i] == "bottom":
                numbottoms += 1
            if attacks[i] not in ("bottom", "top"):
                return false
    if numbottoms < 2:
        return false

    
    for i in range(4,7):
        if attacks[i] not in ("bottom", "top"):
            return false

    if attacks[7] != "fire":
        return false

    numbottoms = 0

    for i in range(8,10):
        if attacks[i] == "bottom":
            numbottoms += 1
        if attacks[i] not in ("bottom", "top"):
            return false
    if numbottoms < 1:
        return false

    for i in range(10,12):
        if attacks[i] not in ("bottom", "top"):
            return false
    
    if attacks[12] != "fire":
        return false
    return true

def check_good_multi(attacks):

    if attacks[12] != "fire":
        return false


    fire_attacks = [i for i, x in enumerate(attacks) if x == "fire"]
    if len(fire_attacks) != 2:
        return false

    first_half = []
    second_half = []
    if fire_attacks[0] == 6:
        # need at least 1 bottoms in first half, 2 in second half

        first_half = attacks[0:6]
        second_half = attacks[7:12]
        if  attacks[0:5].count("bottom") < 1:
            return false
        if attacks[7:9].count("bottom") < 2:
            return false  
    elif fire_attacks[0] == 7:
        # need at least 2 bottoms in first half, 1 in second half
        first_half = attacks[0:7]
        second_half = attacks[8:12]
        if  attacks[0:5].count("bottom") < 2:
            return false
        if attacks[8:10].count("bottom") < 1:

            return false

    elif fire_attacks[0] == 8:
        # need at least 3 bottoms in first half, 0 in second half
        first_half = attacks[0:8]
        second_half = attacks[8:12]
        if  attacks[0:5].count("bottom") < 3:
            return false
    else:
        return false
    for atk in first_half:
        if atk not in ("top", "bottom"):

            return false
    for atk in second_half:
        if atk not in ("top", "bottom"):

            return false
    return true


def seveight():
    totals = 0
    goods = 0
    for atks in itertools.product(attack_types, repeat = 2):
        totals += 1
        bots = atks.count("bottom")
        if bots < 1:
            continue
        tops = atks.count("top")
        if tops + bots < 2:
            continue
        goods += 1
    print(f"seveneight odds: {goods}/{totals}")



goods_multi = 0
goods_single = 0
TOTAL = 100_000_000
for i in range(TOTAL):
    totals += 1
    choices = random.choices(attack_types,k=13)
    gs = check_good(choices)
    if gs:
        goods_single += 1
    gm = check_good_multi(choices)
    if gm:
        goods_multi += 1
    if gs and not gm:
        print(f"gm doesnt like {choices}")

    if totals % 100_000 == 0:
        print(f"{totals//1000}k done")

print(f"{goods_multi} vs {goods_single}")