manips = dict()

xs = [
    2,

]


# we could also have no x press but i dont care
for x in range(0,37):
    for z in range(x+1,38):
        icalls = 562
        xcalls = (x * x+1)
        zcalls = (z-x) * 64

        calls = xcalls + zcalls + icalls

        manips[calls] = f"switch overflow x:{x} z: {z}"
print(f"// This file auto generated through gen_switch_manips.py")
print(f"const SWITCH_OVERFLOWS: [(usize, i32, Option<&str>); {len(manips)+1}] = [")
for calls, text in manips.items():
    print(f"    (0, {calls}, Some(\"{text}\")),")
print("];")