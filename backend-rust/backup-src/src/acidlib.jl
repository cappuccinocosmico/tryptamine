# using Distributed
using PolynomialRoots
using Polynomials



ϵ = .0001

# (-2,-1,0,1)= Superatractive, Attractive,Neutral ,Repelling
struct FixedPoint
    ps::Vector{Number}
    type::Int
    function FixedPoint(ps::Vector{Number},dx::Number)
        function rectify(p::Number)::Number
            if isinf(p)
                return Inf
            elseif isnan(p)
                return NaN
            else
                return complex(p)
            end
        end
        function fixedpointclassify(dx)
            dx=abs2(dx)
            if dx == 0
                return -2
            elseif dx < 1
                return -1
            elseif dx == 1
                return 0
            else
                return 1
            end
        end
        return new(rectify.(ps),fixedpointclassify(dx))
    end
end



function Base.string(fp::FixedPoint)::String
    return "Test"
end
Base.length(fp::FixedPoint)=length(fp.ps)
FixedPoint(p::Number,dx::Number=1.0)=FixedPoint(Number[p],dx)
FixedPoint(ps::Vector{ComplexF64},dx::Number)=FixedPoint(Array{Number}(ps),dx)

FixedPoint(t::Tuple)=FixedPoint(t...)

getp(fp::FixedPoint)::Number=(fp.ps)[1]
isbasin(fp::FixedPoint)::Bool=fp.type<0
filterbasins(fps::Array{FixedPoint})=filter(isbasin,fps)
eq(a::FixedPoint,b::FixedPoint)=isequal(getp(a),getp(b))




function filterout(as::Array,bs::Array,epsilon::Float64=ϵ) # returns as-bs
    if length(bs)==0
        return as
    else
        aboutin(a)::Bool=!foldl(|,(x->abs2(a-x)<epsilon).(bs))
        return filter(aboutin,as)
    end
end





function getpolyfp(poly::Polynomial,cycle::Int64) # get the julia set thing for f(z)=z^n+c
    cycle>6 ? (@warn "Cycles greater then 6 have been known to cause NaN's and melted computers") : Nothing

    fixedpoints=FixedPoint[FixedPoint(Inf,0.0)]
    pointlist=Complex[]
    iter=Polynomial([0,1])
    for i in 1:cycle
        iter=iter(poly)
        deriv=derivative(iter)
        bigrootlist=filterout(PolynomialRoots.roots(Complex.((iter+Polynomial([0,-1])).coeffs)),pointlist)
        append!(pointlist,bigrootlist)
        k=1
        while k ≤ length(bigrootlist)
            blaah=[bigrootlist[k]]
            for l in 1:i-1
                push!(blaah,poly(blaah[end]))
            end
            push!(fixedpoints,FixedPoint(blaah,deriv(blaah[1])))
            if k != length(bigrootlist)
                bigrootlist=[[bigrootlist[1:k]]; filterout(bigrootlist[k+1:end],blaah)]
            end
            k+=1
        end
    end
    return fixedpoints
end

function getnewton(poly)
    basins=[FixedPoint(Inf,2.0)]
    rootlist=PolynomialRoots.roots(Complex.(poly.coeffs))
    basins=[basins;[FixedPoint(root,0.0) for root=rootlist]]
    return (x->x-poly(x)/derivative(poly)(x),basins)
end


#=
function explicitcycleeq(func,iter)
    iter>5 ? throw(DomainError(iter,"Argument must not set computer on fire")) :
    z=S.get_variables(func)[1]
    f=z
    for i in 1:iter
        f=S.substitute(f,z=>func)
    end
    return 0~f-z
end
=#













function makecanvas(r::Int64,z::Float64=1.0,a::ComplexF64=0.0*im,ratio::Float64=1.0)::Matrix{ComplexF64}
    return a .+ ((z/ratio) .* [complex(n,-m) for m=-1:2/(r):1,n=-ratio:2/(r):ratio])
end


function _julia(iterable::Function,canvas::Matrix{ComplexF64},basins::Vector{FixedPoint},maxiter::Int,epsilon::Float64)::Matrix{Tuple{Int64, Int64}}
    function generateneighborhoods(basins::Vector{FixedPoint},epsilon::Float64)::Vector{Function}
        result=Function[]
        for fp in basins
            if getp(fp)==Inf
                push!(result,x->abs2(x)>100/epsilon)
            else
                push!(result,x-> abs2(x-getp(fp))<epsilon)
            end
        end
        return result
    end
    neighborhoods=generateneighborhoods(basins,epsilon)
    function bigor(xs::Vector{Function})::Function
        if length(xs)==1
            return xs[1]
        else
            return function (p::Complex)
                result = xs[1](p)
                for i in xs[2:end]
                        result=result | i(p)
                end
                return result
            end
        end
    end
    test=bigor(neighborhoods)
    skips::Int64=1
    function holo(c::ComplexF64)::Tuple{ComplexF64,Int64}
        x=c
        for i in skips:skips:maxiter
            for k in 1:skips
                x=iterable(x)
            end
            if test(x)
                return (x,i)
            end
        end
        return (x,maxiter)
    end
    rawresults = holo.(canvas)
    function classifyresults((x,i)::Tuple{ComplexF64,Int64})::Tuple{Int64, Int64}
        for k in 1:length(basins)
            if neighborhoods[k](x)
                return (i,k)
            end
        end
        return (typemax(Int64),-1)
    end
    results = classifyresults.(rawresults)
    return results
end


function polyj(poly::Polynomial,canvas::Matrix{ComplexF64},maxiter::Int64,epsilon::Float64=ϵ)::Tuple{Vector{FixedPoint},Matrix{Tuple{Int64, Int64}}}
    basins=filterbasins(getpolyfp(poly,6))
    f(x::ComplexF64)::ComplexF64=poly[x]
    results=_julia(f,canvas,basins,maxiter,epsilon)
    return refactorbasins(basins,results) 
end
polyj(c::ComplexF64,canvas::Matrix{ComplexF64},maxiter::Int64,epsilon::Float64=ϵ)=polyj(Polynomial([c,0,1]),canvas,maxiter,epsilon)

function sinj(c::ComplexF64,canvas::Matrix{ComplexF64},maxiter::Int64,epsilon::Float64=ϵ)::Tuple{Vector{FixedPoint},Matrix{Tuple{Int64, Int64}}}
    f(x::ComplexF64)::ComplexF64=c*sin(x)
    roots=getsinroots(c)
    basins=filterbasins(roots)
    results=_julia(f,canvas,basins,maxiter,epsilon)
    return refactorbasins(basins,results) 
end


function refactorbasins(basins::Vector{FixedPoint},matrix::Matrix{Tuple{Int64, Int64}})
    counts=countmap((v->v[2]).(matrix),alg=:dict)
    basinkeys=sort(collect(keys(counts)))
    if basinkeys[1]==-1
        basinkeys=basinkeys[2:end]
    end
    pos=indexin(Inf,(getp).(basins))
    if length(pos)[1] != nothing
        counts[pos[1]]=2^62
    end
    sort!(basinkeys,by=x->counts[x],rev=true)
    function f((i,n)::Tuple{Int64, Int64})::Tuple{Int64, Int64}
        if n==-1
            return (i,-1)
        else
            return (i,indexin(n,basinkeys)[1])
        end
    end
    return (basins[basinkeys],f.(matrix))
end



function uniqueishpointsarrays(as,cycles::Int64=1) # filters out all the points in as that are within ϵ of each other
    result=[as[1]]
    bs=as[2:end]
    while length(bs)≥2
        for c in result[end]
            filter!(x->abs2(x[1]-c)>epsilon,bs)
        end
        push!(result,bs[1])
        bs=bs[2:end]
    end
    append!(result,bs)
    return result
end

function uniqueishpoints(as,epsilon::Float64=ϵ)
    result=[as[1]]
    bs=as[2:end]
    while length(bs)≥2
        c=result[end]
        filter!(x->abs2(x-c)>epsilon,bs)
        push!(result,bs[1])
        bs=bs[2:end]
    end
    append!(result,bs)
    return result
end

function getsinroots(c::Complex)      
    cycles=2
    test=x->c*sin(c*sin(x))
    dtest=x->c^2*cos(x)*cos(c*sin(x  ))
    listvals(x)=generate(y->c*sin(sin(y)),x,cycles)
    roots = uniqueishpointsarrays((listvals).(roots),cycles)
    sort!(roots,by=(x->abs2(c*cos(x))),rev=true)
    return [[FixedPoint(Inf,0.0)];map(x->FixedPoint(x,c*cos(x)),roots)]
end
 
function bruteforce(f::Function,fp::Function;size::Float64=20.0,res::Int64=40,eps::Float64=.05)
    sample=vec(makecanvas(res,size))
    newtonize(x::ComplexF64)::ComplexF64=x-f(x)/fp(x)
    sample=newtonize.(sample)
    sample=newtonize.(sample)
    filter!(x->abs2(fp(x))>9,sample)
    for n in 1:4
        for i in 1:10
            sample=newtonize.(sample)
        end
        filter!(x->abs2(fp(x))>9,sample)
        sample=uniqueishpoints(sample,eps)
    end
    filter!(x->abs2(f(x))<eps,sample)
    return sample
end
go
# scratchpad

function doublesintaylor(c,n::Int64)
    c=ComplexF64(c)
    poly=[0.0]
    @variables p,sc
    f = sc*sin(sc*sin(p))-p
    poly=[Symbolics.value(substitute(f,[p=>0,sc=>c]))]
    for i in 1:n
        @info f
        f=Symbolics.derivative(f,p)
        push!(poly,(Symbolics.value(substitute(f,[p=>0,sc=>c]))/factorial(i)))
    end
    return Polynomial(poly)
end
function scratchmaclorin(n::Int64)
    poly=[0.0]
    @variables p
    f = sin(sin(p))-p
    for i in 1:n
        f=Symbolics.derivative(f,p)
        push!(poly,Symbolics.value(substitute(f,p=>0))/factorial(n))
    end
    return Polynomial(poly)
end






#=

function getsinroots(c::Complex)
    cycles=2
#    exprs=c*sin(testvar)-testvar
#    @info "$exprs"
#    functaylor = taylor(exprs,7)
    functaylor=doublesintaylor(c,12)
    roots=PolynomialRoots.roots(functaylor.coeffs)
    newtonize(x)=x-(c*sin(sin(x))-x)/(c*cos(x)*cos(sin(x))-1)
    for i in 1:10
        roots=newtonize.(roots)
    end
    listvals(x)=generate(y->c*sin(sin(y)),x,cycles)
    roots = uniqueishpoints((listvals).(roots),cycles)
    sort!(roots,by=(x->abs2(c*cos(x))),rev=true)
    return [[FixedPoint(Inf,0.0)];map(x->FixedPoint(x,c*cos(x)),roots)]
end
function _orderedbasins(xs::Matrix{Tuple{Int64, Int64}})::Vector{FixedPoint}
    ys=countmap((x->x.fp).(xs))
    y=collect(keys(ys))
    return sort(y,by=x->ys[x],rev=true)
end

function orderedbasins(xs::Matrix{Tuple{Int64, Int64}})::Vector{FixedPoint}
    y=_orderedbasins(xs)
    pos=findall(x->isinf(getp(x)),y)
    ys=[[y[pos[1]]];deleteat!(y,pos)]
    return filter((x->!isnan(getp(x))),ys)
end
function backstopsintaylor(c,n) # temporary backstop until Symbolics bug is fixed,
    result = zeros(ComplexF64,n+1)
    result[2]=c-1
    for i in 4:2:n+1
        result[i]=c*((-1)^((i-2)÷2)/factorial(i-1))
    end
    return Polynomial(result)
end
=#