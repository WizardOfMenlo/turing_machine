import subprocess


def generate_nested_paren_of_size_2n(n):
    return "("*n + ")"*n


def generate_flat_paren_of_size_2n(n):
    return "()"*n


if __name__ == "__main__":
    nested = open("data/nested_parendata.csv", 'w+')
    flat = open("data/flat_parendata.csv", 'w+')
    print("n,iter", file=nested)
    print("n,iter", file=flat)
    for i in range(0, 500):
        res_nested = subprocess.run(["target/release/main", "data/paren.tm",
                                     "-T", generate_nested_paren_of_size_2n(i)], capture_output=True, text=True)
        res_flat = subprocess.run(["target/release/main", "data/paren.tm",
                                   "-T", generate_flat_paren_of_size_2n(i)], capture_output=True, text=True)
        num_nested = res_nested.stdout.split('\n')[1]
        num_flat = res_flat.stdout.split('\n')[1]
        print(i*2, num_nested, file=nested, sep=",")
        print(i*2, num_flat, file=flat, sep=",")
