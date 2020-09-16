function p_order(i, p)
    for iout = 1:(p-1)
        if powermod(i, iout, p) == 1
            return iout
        end
    end
    println("---------ERROR-----------")

end

function good_xy(p)
    z3 = Set(
        powermod(i, 3, p) for i=0:(p-1)
    )
    xy_list = [
        (powermod(2, ix, p) + powermod(3, iy, p) + 5) % p in z3
        for ix = 0:p_order(2, p)-1, iy = 0:p_order(3, p)-1
    ]
    return xy_list
end

function print_good(sheet)
    size_x, size_y = size(sheet)
    for ix=0:size_x-1, iy=0:size_y-1
        if sheet[ix+1, iy+1]
            println((ix, iy))
        end
    end
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
