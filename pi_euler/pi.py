# This program give an approach to pi using the solution of Euler to the famous series 1/x²

epsilon = 1E-16
sum = 0
i = 0
term = 1
while term > epsilon :
    i += 1
    term = 1/(i*i)
    sum += term

print(f'The approach of pi using an epsilon of {epsilon} is {(sum*6)**0.5}')
