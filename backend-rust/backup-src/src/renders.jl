using ColorSchemes
using Images
using ImageInTerminal,Sixel
using DelimitedFiles

# Main
function pretty(colors::Vector)::Function
    function result(t::Tuple{Int64, Int64})::Color
        if t[2]>0
            return (colors[t[2]])[(t[1]%length(colors[t[2]]))+1]
        else
            return color("black")
        end 
    end
    return result
end

setcolorscheme=ColorScheme([[color("black")];(ColorSchemes.Set1_9).colors])

set(b::Int64=-1)::Function=x->Gray(x[2]!=b)

function mset(colors::ColorScheme=setcolorscheme)::Function
    function result(t::Tuple{Int64, Int64})::Color
        if 0<t[2]<length(colors)
            return colors[t[2]]
        else
            return color("white")
        end
    end
    return result
end

struct Recipe
    name::String
    build::Function
    dpoint::ComplexF64
    epsilon::Float64
    zoom::Float64
    center::ComplexF64
#   cutoff::Int64=typemax(Int64)
    prettycolors::Vector{Vector}
    ratio::Float64
end


function render(r::Recipe,res::Int64;maxiter::Int64=200,dir::String="/home/nicole/Documents/tryptamine/renders")::Nothing
    canvas=makecanvas(res,r.zoom,r.center,r.ratio)
    directory=dir*"/$(r.name)"
    @info "Calculating: $(r.name)"
    @time basins,results=(r.build)(r.dpoint,canvas,maxiter,r.epsilon)
    mkpath("$directory/raw")
    write("$directory/raw/basins.raw", string(basins))
    writedlm("$directory/raw/results.raw", results)
    for i in 1:length(r.prettycolors)
        @info "Writing Pretty Render $i"
        save("$directory/$(r.name)-$i.png",(pretty(r.prettycolors[i])).(results),quality=10)
    end
    @info "Writing Julia Set"
    save("$directory/sets/juliaset.png",(set().(results)),quality=10)
    for i in 1:length(basins)
        @info "Writing Fatou Set $(basins[i])"
        save("$directory/sets/fatouset($(getp(basins[i]))).png",(set(i).(results)),quality=10)
    end
    @info "Saved outputs in directory: $directory"
end

render(r::Vector{Recipe},res::Int64,maxiter::Int64,dir::String="/home/nicole/Documents/tryptamine/renders/")=render.(r,res,maxiter,dir)






function qr(f::Function,cp,z;cen::ComplexF64=c(0), r::Int64=500,e::Float64=ϵ,colors::Vector=samplecolors,maxiter::Int64=200,ratio::Float64=1.0)
    canvas=makecanvas(r,Float64(z),cen,ratio)
    basins,results=f(cp,canvas,maxiter,e)
    return pretty(colors).(results)
end

qr(recipe::Recipe,r::Int64=500)=qr(recipe.build,recipe.dpoint,recipe.zoom,cen=recipe.center,e=recipe.epsilon,r=r,ratio=recipe.ratio)


