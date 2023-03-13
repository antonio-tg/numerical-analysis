# This program solves a quadratic equation of the second degree

print("This program solves a quadratic equation of the second degree like ax²+bx+c=0, enter the coefficients: ")
a = float(input("Enter the coefficient a: "))
b = float(input("Enter the coefficient b: "))
c = float(input("Enter the coefficient c: "))
print("the roots are")
x0 = (-b + (b**2-4*a*c)**0.5 )/2*a
x1 = (-b - (b**2-4*a*c)**0.5 )/2*a
print(f'x0 = {x0}')
print(f'x1 = {x1}')
