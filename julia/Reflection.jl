module Reflection

    global const libbruges_rs = joinpath(@__DIR__, "lib", "libbruges_rs.dll")
    export critical_angles

    # calculate the critical angle between two mediums
    function critical_angles(vp1::Float64, vp2::Float64; vs2=nothing, degrees::Bool=true)

        ca1 = ccall(
                ("critical_angle", libbruges_rs), Float64, (Float64, Float64), vp1, vp2
        )
        if isnothing(vs2)
            return ca1
        else
            vs2 = Float64(vs2)
            ca2 = ccall(
                ("critical_angle", libbruges_rs), Float64, (Float64, Float64), vp1, vs2
            )
            return (ca1, ca2)
        end
    end
end
