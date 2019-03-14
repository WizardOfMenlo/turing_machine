import subprocess


def carry_string_size2n(n):
    s = "1"*n + "#1#" + "0"*n + "1"
    return (s, len(s))


def long_c(n):
    s = "##" + "0" * 2 * n
    return (s, len(s))


def execute_func(f, file):
    for i in range(0, 500):
        v = f(i)
        res = subprocess.run(["target/release/main", "data/binadd.tm",
                              "-T", v[0]], capture_output=True, text=True)

        num = res.stdout.split('\n')[1]
        print(v[1], num, file=file, sep=",")


if __name__ == "__main__":
    carry = open("data/carry_binadd.csv", 'w+')
    lng = open("data/long_binadd.csv", 'w+')
    print("n,iter", file=carry)
    print("n,iter", file=lng)
    execute_func(carry_string_size2n, carry)
    execute_func(long_c, lng)
