program general_quadratic_equation

!This program solves a quadratic equation of the second degree
implicit none
real*16 a, b, c, d, x0, x1

write(*,*) "This program solves a quadratic equation of the second degree like ax²+bx+c=0, enter the coefficients:"
write(*,*) "Enter the coefficient a"
read(*,*) a
write(*,*) "Enter the coefficient b"
read(*,*) b
write(*,*) "Enter the coefficient c"
read(*,*) c

!Dicriminant
d = b*b-4.0*a*c

write(*,*) "the roots are"
if (d == 0.0) then
    x0 = -b/2*a
    write(*,*) "the roots are real and equal"
    write(*,*) x0
elseif (d > 0) then
    x0 = (-b + (b**2-4*a*c)**0.5 )/2*a
    x1 = (-b - (b**2-4*a*c)**0.5 )/2*a
    write(*,*) "the roots are"
    write(*,*) x0, x1
else 
    x0 = -b/2*a 
    x1 = (-1*(b**2-4*a*c))**0.5/2*a
    write(*,*) "The roots are complex conjugates:"
    write(*,*) "Real part"
    write(*,*) x0
    write(*,*) "Imaginari part +-"
    write(*,*) x1
endif
end program
