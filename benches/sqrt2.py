import subprocess


def execute_func(file):
    for i in range(0, 1000):
        res = subprocess.run(["target/release/main", "data/sqrt2.tm",
                              "-l", str(i*100)], capture_output=True, text=True)

        s = res.stdout.split('\n')[2]
        num = s.count('0') + s.count('1')
        print(i*100, num, file=file, sep=",")


if __name__ == "__main__":
    sqrt = open("data/sqrt2.csv", 'w+')
    print("steps,digits", file=sqrt)
    execute_func(sqrt)
