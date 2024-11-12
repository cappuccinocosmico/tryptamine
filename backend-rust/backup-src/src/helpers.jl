


c(a::Number)::ComplexF64=complex(Float64(a))
c(a::Number,b::Number)::ComplexF64=complex(Float64(a),Float64(b))

function basic(c,n=2)# returns a polynomial of x^n+c
    coeffs=zeros(Complex,n+1)
    coeffs[1],coeffs[end]=[c,1]
    return Polynomial(coeffs)
end