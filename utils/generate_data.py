import os
import yaml
import random
import sha3
import sys


def read_config(filename):
    with open(filename, 'r') as file:
        config = yaml.safe_load(file)
    return config


def generate_random_data(size):
    return bytearray(random.getrandbits(8) for _ in range(size))


def write_page_to_file(path, data):
    with open(path, 'wb') as file:
        file.write(data)


def compute_merkle_root(pages):
    def hash_data(data):
        k = sha3.keccak_256()
        k.update(data)
        return k.digest()

    leaves = [hash_data(page) for page in pages]

    if not leaves:
        return hash_data(b'')

    while len(leaves) > 1:
        new_leaves = []
        for i in range(0, len(leaves), 2):
            combined = leaves[i]
            if i + 1 < len(leaves):
                combined = combined + leaves[i + 1]
            new_leaves.append(hash_data(combined))
        leaves = new_leaves

    return leaves[0]


def write_root_hash(path, root_hash):
    with open(path, 'wb') as file:
        file.write(root_hash)


def main():
    config = read_config(sys.argv[1])
    os.makedirs(config['directory'], exist_ok=True)

    pages = []
    num_files = config['num_pages']
    page_size = config['page_size']

    for i in range(1, num_files + 1):
        file_name = f"{config['directory']}/{config['file_prefix']}{i}"
        data = generate_random_data(page_size)
        write_page_to_file(file_name, data)
        pages.append(data)
        print(f"Generated {file_name}")

    root_hash = compute_merkle_root(pages)
    write_root_hash(f"{config['directory']}/{config['root_hash_file']}", root_hash)
    print(f"True merkle root hash: {root_hash.hex()}")

    faulty_hash = generate_random_data(32)
    write_root_hash(f"{config['directory']}/{config['root_hash_file']}_invalid", faulty_hash)
    print(f"Wrong merkle root hash: {faulty_hash.hex()}")


if __name__ == "__main__":
    main()
