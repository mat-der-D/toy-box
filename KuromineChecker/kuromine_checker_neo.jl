function p_order(i::Int, p::Int)
    if gcd(i, p) != 1
        errmsg = "gcd of (i, p) must be 1."
        throw(DomainError((i, p), errmsg))
    end
    for iout = 1:(p-1)
        if powermod(i, iout, p) == 1
            return iout
        end
    end
end


lhs(x, y, p) = (
    powermod(2, x, p)
    + powermod(3, y, p)
    + 5
) % p


mutable struct Sheet
    periods::NTuple{2,Int}
    candidates::Set{NTuple{2,Int}}
end


function sheet_gen(p::Int)
    z³ = Set(
        powermod(n, 3, p) for n=0:p-1
    )

    ord2 = p_order(2, p)
    ord3 = p_order(3, p)
    periods = (ord2, ord3)
    candidates = Set(
        (ix, iy)
        for ix=0:ord2-1, iy=0:ord3-1
        if lhs(ix, iy, p) in z³
    )

    return Sheet(periods, candidates)

end


function sheet_gen(x::Int, y::Int, mod_x::Int, mod_y::Int)
    periods = (mod_x, mod_y)
    candidates = Set(
        (x % mod_x, y % mod_y),
    )
    return Sheet(periods, candidates)
end


function ⊗(sheet1, sheet2)
    size1, size2 = sheet1.periods, sheet2.periods
    lcm_x, lcm_y = lcm.(size1, size2)
    n_ext1 = (lcm_x, lcm_y) .÷ size1
    n_ext2 = (lcm_x, lcm_y) .÷ size2

    new_cands = Set(
        (x + ix*size1[1], y + iy*size1[2])
        for (x, y) in sheet1.candidates
        for ix = 0:n_ext1[1]-1, iy = 0:n_ext1[2]-1
        if (
            (x + ix*size1[1], y + iy*size1[2])
            .% (size2[1], size2[2])
            in sheet2.candidates
        )
    )
    return Sheet((lcm_x, lcm_y), new_cands)
end


function pr(sheet::Sheet)
    println("mod:", sheet.periods)
    println("candidates:")
    println("   ", sheet.candidates)
end


function prod_suc(ps::Int...)
    sheet = sheet_gen(ps[1])
    for ip = 2:length(ps)
        sheet = sheet ⊗ sheet_gen(ps[ip])
    end
    return sheet
end
