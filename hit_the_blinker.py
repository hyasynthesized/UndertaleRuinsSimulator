HALTED = 0
UNHALTED =  1

TALKING = 0
BLINKING = 1
def blink_compensate(input: str) -> int:
    tbstate = HALTED
    torstate = TALKING
    frame = 0
    numblinks = 0
    time_spent_blinking = 0
    for char in input:
        
        if torstate == TALKING:
            if frame == 8:
                frame = 0
            if frame == 0 and tbstate == HALTED:
                torstate = BLINKING
                numblinks += 1
            frame += 1
        elif torstate == BLINKING:
            time_spent_blinking += 1
            if tbstate == UNHALTED:
                frame = 1
                torstate= TALKING
                time_spent_blinking = 0
        match char:
            case "z":
                tbstate = UNHALTED
            case "x":
                tbstate = HALTED
    return numblinks


print(blink_compensate("xzxzxzxzxzxzxzxz"))