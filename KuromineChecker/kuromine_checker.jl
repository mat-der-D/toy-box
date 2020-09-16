"""
p_order returns the order of i module p.
"""
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


function sheet_gen(p::Int)
    z_cubed = Set(
        powermod(n, 3, p) for n=0:p-1
    )

    ord2 = p_order(2, p)
    ord3 = p_order(3, p)
    sheet = [
        lhs(ix, iy, p) in z_cubed
        for ix=0:ord2-1, iy=0:ord3-1
    ]
    return sheet
end


function sheet_gen(x::Int, y::Int, mod_x::Int, mod_y::Int)

    sheet = [
        (x % mod_x == ix) & (y % mod_y == iy)
        for ix = 0:mod_x-1, iy = 0:mod_y-1
    ]
    return sheet

end


function ⊗(sheet1, sheet2)
    size1, size2 = size.((sheet1, sheet2))
    lcm_x, lcm_y = lcm.(size1, size2)
    n_ext1 = (lcm_x, lcm_y) .÷ size1
    n_ext2 = (lcm_x, lcm_y) .÷ size2

    sheet1_ext = repeat(sheet1, n_ext1...)
    sheet2_ext = repeat(sheet2, n_ext2...)

    return sheet1_ext .& sheet2_ext
end


function ⊕(sheet1, sheet2)
    size1, size2 = size.((sheet1, sheet2))
    lcm_x, lcm_y = lcm.(size1, size2)
    n_ext1 = (lcm_x, lcm_y) .÷ size1
    n_ext2 = (lcm_x, lcm_y) .÷ size2

    sheet1_ext = repeat(sheet1, n_ext1...)
    sheet2_ext = repeat(sheet2, n_ext2...)

    return sheet1_ext .| sheet2_ext
end


function candidates(sheet)
    size_x, size_y = size(sheet)
    cand = [
        (ix, iy)
        for ix = 0:size_x-1, iy = 0:size_y-1
        if sheet[ix+1, iy+1]
    ]
    return cand
end


function pr(sheet)
    cand = candidates(sheet)
    println("mod:", size(sheet))
    println("Candidates:")
    println("  ", cand)
end


function prod_suc(ps::Int...)
    sheet = sheet_gen(ps[1])
    for ip = 2:length(ps)
        sheet = sheet ⊗ sheet_gen(ps[ip])
    end
    return sheet
