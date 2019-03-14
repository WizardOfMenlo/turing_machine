import subprocess


def repeat_f(n):
    return ("1" * 2 * n, 2*n)


def execute_func(f, file):
    for i in range(0, 500):
        v = f(i)
        res = subprocess.run(["target/release/main", "-n", "data/repeat.tm",
                              "-T", v[0]], capture_output=True, text=True)

        num = res.stdout.split('\n')[1]
        print(v[1], num, file=file, sep=",")


if __name__ == "__main__":
    repeat = open("data/repeat.csv", 'w+')
    print("n,iter", file=repeat)
    execute_func(repeat_f, repeat)
