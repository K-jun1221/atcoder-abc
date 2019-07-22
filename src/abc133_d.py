import numpy

n = int(input())
a = list(map(lambda x: [int(x)], input().split()))

met = []

for i in range(0, n):
    row = [0 for _ in range(0, n)]
    row[i] = 1
    if (i == n-1):
        row[0] = 1
    else:
        row[i+1] = 1
    met.append(row)
A = numpy.matrix(met)
Y = numpy.matrix(a)

ans = []
for j in numpy.linalg.inv(A)*Y:
    ans.append(str(int(j) * 2))

print(" ".join(ans))
