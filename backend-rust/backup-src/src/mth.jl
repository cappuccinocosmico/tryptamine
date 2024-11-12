using Polynomials
using Symbolics
using StatsBase


function generate(f::Function,x::Any,n::Int64)
    result=[x]
    for i in 2:n
        push!(result,f(result[end]))
    end
    return result
end

function miller(n::Integer)::Bool
    k=trailing_zeros(n-1)
    s=(n-1) ÷ big(2^k)
    function btest(b::Int64)::Bool
        x=powermod(b,s,n)
        if x==1 || x == n-1
            return true
        end
        for i in 1:k-1
            x=powermod(x,2,n)
            if x == 1 || x == n-1
                return x!=1
            end
        end
        return false
    end
    for b in [2,3,5,7]
        if !btest(b)
            return false
        end
    end
    return true
end


const p100=[2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89, 97]

function factor(n::Integer)::Vector{Integer}
    ls=[n]
    cont=true
    while cont
        cont=false
        for p in p100
            if ls[1]%p==0 && ls[1] !=p
                ls=[[ls[1] ÷ p];p;Base.rest(ls,2)]
                cont=true
            end
        end
    end
    if ls[1]<10000
        return sort(ls)
    else
        return sort([_factor(ls[1]);Base.rest(ls,2)])
    end
end

function _factor(n::Integer)
    if miller(n)
        return [n]
    else
        a,b=rho(n)
        return [_factor(a);_factor(b)]
    end
end
function rho(n::Integer;b::Int64=2)
    if b>5
        error("Failed to Factor")
    end
    g(x)=mod((x^2+1),n)
    tort=b
    hare=g(b)
    i=c=1
    while gcd(tort-hare,n)==1

         if i==c
            tort=hare
            c=2*c
            i=0
         end
         hare=g(hare)
         i=i+1
    end
    r=gcd(tort-hare,n)
    if r==n
        return rho(n,b=b+1)
    else
        return (r,n ÷r)
    end
end

function eulertotient(n::Int64)
    factored =sort!(factor(n))
    result = 1
    p = 1
    for i in eachindex(factored)
        if i != p
            result *= p=1
            p=i
        else
            result *= p
        end
    end
    return result
end

function listprimes(n::Int64)::Vector{Int64}
    list=[2,3,5,7]
    for i in 11:2:n
        if miller(i)
            push!(list,i)
        end
    end
    return list
end


function windowpane(n::Integer)
    if isinteger(sqrt(n))
        return (Integer(sqrt(n)),Integer(sqrt(n)))
    else
        factors=factor(n)
        if factors[end]>sqrt(n)
            return (factors[end],n ÷ factors[end])
        else
            function reorderfactors(factors::Vector)
                matr=vcat([[val -key] for (key,val) in countmap(factors)]...)
                matr=sortslices(matr,dims=1)
                return [(matr[i,1],-matr[i,2],Float64(log2(-matr[i,2]))) for i in 1:size(matr)[1]]
            end
            fs=reorderfactors(factors)
            @info fs
            function findp(vs::Vector,bound::Float64)
                fl(t)=Int64(floor(t))
                if length(vs)==1
                    return (vs[1])[2]^(minimum([(vs[1])[1],fl(bound/(vs[1])[3])]))
                else
                    k,p,lp=vs[1]
                    return maximum([p^i*findp(vs[2:end],bound-i*lp) for i in 0:minimum([k,fl(bound/lp)])])
                end
            end
            p=findp(fs,Float64(log2(n)/2))
            return (p,n ÷ p)
        end
    end
end


function findfracs_(x::Float64,bound::Float64 =2^-31)
    n, eps = 1 , abs(x-round(x))
    results=[Rational(round(x))]
    while true
        s=round(Int64,x*n)
        if abs(x-s/n)<eps
            push!(results,s//n)
            eps=abs(x-s/n)
            if eps < bound
                return results
            end
        end
        n=n+1
    end
end


r(;b::Integer=big(2)^100)=rand(1:b)
r(n;b::Integer=big(2)^100)=[r(b=b) for i=1:n]


N=251
q=128
p=3

struct NTRUkey

end

keygen()::NTRUKey=keygen(rand(RandomDevice(),Int128))
function keygen(seed::Int128)::NTRUkey
    N=251
    q=128
    p=3
    rng=Random.Xoshiro(seed)
    g=rand(rng,-1:1,127)
    switch=true
    while switch
        f=rand(rng,-1:1,127)
    end
end



# a+b 


function polyinv(f,n)# Finds the inverse of the polynomial p mod n
    f=mod(f,n)
    fermat=[[-1];zeros(Int64,n-2);[1]]
    c=polybezout(f,fermat,n)
    if degree(c[3])==0
        return c[1]
    else
        return "Failure"
    end
end

function polyegcd(f,g,n) # returns a*f+b*g=gcd(f,g)
    if degree(f)<degree(g)
        return polyegcd(g,f,n)
    end
    rs=[f,g]
    as=[Polynomial(1),Polynomial(0)]
    bs=[Polynomial(0),Polynomial(1)]
    qi=Polynomial(0)
    @info "$(rs[2]) = $(as[2]) * f + $(bs[2]) * g : $(rs[2] == mod(as[2] * f + bs[2] * g,n))"
    while degree(rs[2])>0
        qi,rs[2],rs[1]=[polyfldmod(rs[1],rs[2],n);[rs[2]]]
        as=[as[2],mod(as[1]-qi*as[2],n)]
        bs=[as[2],mod(as[1]-qi*as[2],n)]
        @info "$(rs[2]) = $(as[2]) * f + $(bs[2]) * g : $(rs[2] == mod(as[2] * f + bs[2] * g,n))"
    end
    if degree(rs[2])==0
        return mod.(([as[2],bs[2],rs[2]].*invmod((rs[2])[0],n)),n)
    else
        return [as[1],bs[1],rs[1]]
    end
end

function Base.mod(p::Polynomial,n::Integer)
    return Polynomial(mod.(p.coeffs,n))
end
polypower(n::Int64)=Polynomial([zeros(Int64,n);[1]])

function polyfldmod(f,g,n) # f = p*g + r
    f=mod(f,n)
    g=mod(g,n)
    r=f
    p=Polynomial([0])
    q=invmod(g[end],n)
    lim=degree(g)
    while degree(r) ≥ lim
        term = mod(r[end]*q,n)*polypower(degree(r)-lim)
        p = p + term
        r = mod(r - term*g,n)
    end
    return [p,mod(r,n)]
end


reversenumber(x::BigInt)=sum([reverse(digits(x))[k]*10^(k-1) for k=1:length(digits(x))])

palindrometerm(x::Int64)=palindrometerm(big(x))
function palindrometerm(x::BigInt)
    y=x
    i=1
    yr=reversenumber(y)
    while y!=yr
#        @info("$y+$yr = $(y+yr)")
        y=y+yr
        yr=reversenumber(y)
        i=i+1
    end
    return (i,x,y)
end


function findcoprime(n::Int64)::Vector{Int64}
    result = Int64[]
    for i in 1:n
        if gcd(i,n)==1
            push!(result,i)
        end
    end
    return result
end

function findgenerators(n::Int64)::Vector{Int64}
    totient = length(findcoprime(n))-1
    result = Int64[]
    for i in 1:totient
        if gcd(i,n)==1
            push!(result,i)
        end
    end
    return result
end

testhypothesis(n::Int64)= (n,length(findcoprime(n)),length(findgenerators(n)))
testhypothesis2(n::Int64)= (n,findcoprime(n),findgenerators(n))

# 1 2 3 4 5 6 7 8 9 10 11
# 1 1 2 1 4 2 6 2 4 4 10 



#=
function check1(f,g,n)
    a,b=polyfldmod(f,g,n)
    return mod(f,n)==mod(g*a+b,n)
end
=#
# poly f g there exists polys a,b such that fa+gb=gcd(f,g) letting g=p x^n for all x^n should work if not
#=
using Primes
function primetest(n)
    for i in n
        if Primes.isprime(i)!=prime_fermat(i)
            println("Mismatch on: $i")
        end
    end
end
=#
#=
function msin(x::Float64)::Float64
    cosmac=Polynomial([1,0,-1//2,0,1//24,0,-1//720])
    sinmac=Polynomial([0,1,0,-1//6,0,1//120,0,-1//5040])
    if 0>x>2*π
        return msin(mod(x,2*π))
    elseif x≥π
        return -msin(x-π)
    elseif x≥(π/2)
        return msin(π-x)
    elseif x<(π/4)
        return sinmac(x)
    else
        return cosmac(π/2-x)
    end
end
=#

#=
val(c::Complex{Num})::Complex{Float64}=complex(Symbolics.value(real(c)),Symbolics.value(imag(c)))

function taylor(object,var,n::Int,x::Number=complex(0.0))
    var =Symbolics.get_variables(object)[1]
    result = [Symbolics.substitute(object,var=>x)]
    d = object
    @info "Help 0"
    for i in 1:n
        @info "Help $i"
        d=Symbolics.derivative(d,var)
        push!(result,Symbolics.substitute(d,var=>x)/factorial(i))
    end
    return Polynomial(val.(result))
end

=#
