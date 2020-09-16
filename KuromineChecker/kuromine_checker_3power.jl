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


function sheet_gen3(n::Int)
    m = 3^n
    z³ = Set(
        powermod(i, 3, m) for i=0:n
    )

    ord2 = p_order(2, m)
    periods = (ord2, n)
    candidates = Set(
        (ix, iy)
        for ix=0:ord2-1, iy=0:n
        if lhs(ix, iy, m) in z³
    )

    return Sheet(periods, candidates)
end


function ⊗(sheet1, sheet2)
    size1, size2 = sheet1.periods, sheet2.periods
    len_x = lcm(size1[1], size2[1])
    len_y = max(size1[2], size2[2])
    n_ext1 = len_x ÷ size1[1]
    n_ext2 = len_x ÷ size2[1]

    new_cands1 = Set(
        (x + ix*size1[1], y)
        for (x, y) in sheet1.candidates
        for ix = 0:n_ext1-1
        if is_left(
            x + ix*size1[1], y,
            len_x, len_y,
            sheet1, sheet2
        )
    )

    new_cands2 = Set(
        (x + ix*size2[1], y)
        for (x, y) in sheet2.candidates
        for ix = 0:n_ext2-1
        if is_left(
            x + ix*size2[1], y,
            len_x, len_y,
            sheet2, sheet1
        )
    )

    new_cands = new_cands1 ∩ new_cands2
    return Sheet((len_x, len_y), new_cands)

end

function is_left(ix, iy, len_x, len_y, sheet1, sheet2)

    size1, size2 = sheet1.periods, sheet2.periods
    if iy < size1[2]
        return (
            (ix % len_x, min(iy, size2[2]))
            in sheet2.candidates
        )
    else
        return any(
            (ix, jy) in sheet2.candidates
            for jy = size1[2]:size2[2]
        )
    end

end


function pr(sheet::Sheet)
    println("mod:", sheet.periods)
    println("candidates:")
    println("   ", sheet.candidates)
end


function prod_suc(ns::Int...)
    sheet = sheet_gen3(ns[1])
    for i = 2:length(ns)
        sheet = sheet ⊗ sheet_gen3(ns[i])
    end
    return sheet
end
