

module BrugesRs

    include("Reflection.jl")
    using .Reflection

    function test()
    println("Hello")
end

    export Reflection, test

end # BrugesRs
