import matplotlib.pyplot as plt


class SIRModel:
    def __init__(self, n, m, beta, gamma):
        if not (
            (n > 0) and (m > 0) and (0 < beta < 1) and (0 < gamma < 1)
            and (beta*m < 1)
        ):
            raise Exception(
                "N > 0, M > 0, 0 < beta < 1, 0 < gamma < 1, beta*M < 1 "
                "must be satisified, but actually "
                "(N, M, beta, gamma, beta*M) = "
                f"({n}, {m}, {beta}, {gamma}, {beta*m})"
            )
        self.N = n
        self.M = m
        self.BETA = beta
        self.GAMMA = gamma

    @property
    def initial_sir(self):
        return self.N, self.M, 0
        
    def next_sir(self, s_now, i_now, r_now):
        s_next = s_now - self.BETA*s_now*i_now
        i_next = i_now + self.BETA*s_now*i_now - self.GAMMA*i_now
        r_next = r_now + self.GAMMA*i_now
        if self.BETA * i_now >= 1:
            raise BetaIExcess()
        return s_next, i_next, r_next


def input_with_validation(input_msg, type_):
    while True:
        input_val = input(input_msg)
        try:
            val = type_(input_val)
        except:
            print(f"input value interpretable as a {type_.__name__}")
        else:
            return val

    
class BetaIExcess(Exception):
    pass


if __name__ == "__main__":
    msg = "input {} (float):"
    N = input_with_validation(msg.format("N"), float)
    M = input_with_validation(msg.format("M"), float)
    BETA = input_with_validation(msg.format("beta"), float)
    GAMMA = input_with_validation(msg.format("gamma"), float)
    while True:
        MAX_STEP = input_with_validation(msg.format("maximum n"), int)
        if MAX_STEP < 1:
            print("maximum n must be >=1")
        else:
            break
    
    model = SIRModel(N, M, BETA, GAMMA)
    s, i, r = model.initial_sir
    n_series = [1]
    s_series = [s]
    i_series = [i]
    r_series = [r]

    for n in range(2, MAX_STEP+1):
        try:
            s, i, r = model.next_sir(s, i, r)
        except BetaIExcess:
            print(
                "time stepping was stopped because "
                "beta*I_n exceeded 1"
            )
            break
        else:
            n_series.append(n)
            s_series.append(s)
            i_series.append(i)
            r_series.append(r)

    _, ax = plt.subplots()

    ax.plot(n_series, s_series, label="S_n")
    ax.plot(n_series, i_series, label="I_n")
    ax.plot(n_series, r_series, label="R_n")
    ax.legend()

    title_str = (
        f"N={model.N}, "
        f"M={model.M}, "
        f"beta={model.BETA}, "
        f"gamma={model.GAMMA}"
    )
    ax.set(title=title_str)
    ax.set(xlabel="n", ylabel="S_n, I_n, R_n")
    plt.show()
