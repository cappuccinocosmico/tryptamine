# Misc
crop=1
cyclify(cs::ColorScheme)::ColorScheme=ColorScheme(vcat([cs[i] for i in crop+1:length(cs)],[cs[length(cs)-i] for i in 1:length(cs)-1-crop]))
viewcolors(c)=hcat(((x->x.colors).(c))...)

# Colors
trans=cyclify.([ColorSchemes.PuBu_8,ColorSchemes.RdPu_8,])
samplecolors=cyclify.([ColorSchemes.RdPu_9,ColorSchemes.YlGnBu_9,ColorSchemes.YlOrRd_9,ColorSchemes.Greens_9,ColorSchemes.Purples_9])
# Intresting
m1=Recipe("marblesinthesky",sinj,c(1.2,.1),.001,3*pi,c(0),[samplecolors],3)
d1=Recipe("duality1",sinj,c(1.2,.1),.001,1.1,c(0),[samplecolors],1)
wallpaper=Recipe("test",sinj,c(2.05,.5),.0002,1.0,c(0),[samplecolors],16/9)







