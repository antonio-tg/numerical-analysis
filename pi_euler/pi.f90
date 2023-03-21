program euler
! This program give an approach to pi using the solution of Euler to the famous series 1/x²
implicit none
real*8 epsilon, sum, term, i

epsilon = 1E-16
sum = 0
i = 0
term = 1
do while (term > epsilon)
    i = i + 1
    term = 1/(i*i)
    sum = sum + term
end do

write(*,*) "The approach of pi using an epsilon of"
write(*,*) epsilon
write(*,*) "is"
write(*,*) (sum*6)**0.5
end program
