import sys

def Task_1(N):
    ak, al = 0, 0
    N = int(input())
    for i in range(N+1):
        if SimplNumb(i):
            ak = i
    al = N
    while not SimplNumb(al):
        al += 1
    return N - ak if N - ak < al - N else al - N

def Task_2():
    a = input()
    c = 0
    for i in range(10):
        if 'R'+str(i) in a and 'W'+str(i) in a and 'B'+str(i) in a:
            c += 1
    return c

def Task_3(a, len):
    step = 1
    X = 0
    MX = a
    for i in range(len):
        if a % 10 == (a // 10) % 10 or a % 10 == X % 10:
            step = 0
        else:
            step += 1
        X = a
        a //= 10
    return MX if step == 0 else 0

def Lenth(a):
    len = 0
    X = a
    while X:
        X //= 10
        len += 1
    return len

def SimplNumb(a):
    if a > 2:
        for i in range(2, a):
            if a % i == 0:
                return False
        return True
    else:
        return False

if __name__ == "__main__":
    Numb = 0
    k = 0
    count = 0
    NumbX = 0
    
    Numb = int(input())
    if Numb == 1:
        print(Task_1(k))
    elif Numb == 2:
        print(Task_2())
    elif Numb == 3:
        NumbX = int(input())
        print("Numbers:")
        while count < NumbX:
            k = int(input())
            print(Task_3(k, Lenth(k)))
            count += 1
        print()
    else:
        print("Try again")