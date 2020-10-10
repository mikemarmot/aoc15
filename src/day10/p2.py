import time

def calc(inp:list):
    last = 0
    res = []
    cnt = 1
    for v in inp:
        if last == 0:
            last = v
            continue
        if last == v:
            cnt += 1
        else:
            res.extend([cnt, last])
            last = v
            cnt = 1
    res.extend([cnt, last])
    return res

start = time.time()
input = [ 3, 1, 1, 3, 3, 2, 2, 1, 1, 3]
for i in range(50):
    input = calc(input)
print("Res: {}".format(len(input)))
print('{:5.3f}s'.format(time.time()-start))