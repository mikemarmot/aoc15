from pathlib import Path

rules = {}

with Path('data/input_day19.txt').open() as f:
    for line in f:
        line = line.strip()
        if len(line) == 0:
            continue
        elif '=>' in line:
            l = line.split(' => ')
            rules.update({ l[1] : l[0] })
        else:
            d = line

#print(f'{rules} {d}')

def do_step(input):
    ret = []
    for i in range(len(input)):
        for kr,vr in rules.items():
            if i+len(kr) > len(input) or input[i:i+len(kr)] != kr: continue
            ret.append(''.join([input[:i], vr, input[i+len(kr):]]))
    return ret

abc:set = [d]
while 'e' not in abc:
    print(f'XXX {len(abc)}')
    mabc = []
    for v in abc:
        mabc.extend(do_step(v))
    abc = set(mabc)

