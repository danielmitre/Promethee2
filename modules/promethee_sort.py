import os
from tifffile import imread
from typing import List, Tuple


def load_tiff_matrix(path: str) -> Tuple[List[List[float]], Tuple[int, int]]:
    """load matrix from the plain text file on the given path

    Args:
        path (str): path to the file

    Returns:
        Tuple[List[List[float]], Tuple[int, int]]: (matrix, (height, width))
    """
    matrix = imread(path)
    height = len(matrix)
    width = len(matrix) if height > 0 else 0
    print(path, matrix, matrix.shape)
    return matrix, matrix.shape


def sort(argument):
    argument = list(argument)
    _, (height, width) = load_tiff_matrix(argument[1])
    partialPaths = []
    threads = 1
    bucket_size = height / max(1, (width * height * 8) / (1024 * 512 * 1024))
    for option in argument:
        if '-size=' in option:
            byte = int(option[6:])
            byte = byte * 512 * 1024
            total_buckets = (width * height * 8) / byte
            if total_buckets == 0:
                total_buckets = 1
            bucket_size = height / total_buckets
        if '-proc=' in option:
            threads = int(option[6:])

    if bucket_size == 0:
        print('Size of sort invalid.')
        return 1
    bucket_size /= threads
    start = 0
    xargs_argument = open('arguments', 'w')

    while start < height:
        line = ' '.join([
            '-sort', argument[1],
            '-start=' + str(start), '-end=' +
            str(min(start + bucket_size, height))
        ]) + '\n'
        partialPaths.append(
            'values.' + str(start) + '-' + str(min(start + bucket_size, height)) + '.' + argument[1] + ' ' +
            'positions.' + str(start) + '-' + str(min(start +
                                                      bucket_size, height)) + '.' + argument[1]
        )
        xargs_argument.write(line)
        start = start + bucket_size
    xargs_argument = None
    os.system('cat arguments | xargs -P ' + str(threads) + ' -n 4 ./run')
    a = ' '.join(['./run', '-sort', argument[1], 'positions.tif', '-kway '])
    b = ' '.join(partialPaths)
    os.system(a + b)


def unsort(argument):
    _, (height, width) = load_tiff_matrix(argument[1])
    partialPaths = []
    threads = 1
    bucket_size = height / max(1, (width * height * 8) / (1024 * 512 * 1024))
    for option in argument:
        if '-size=' in option:
            byte = int(option[6:])
            byte = byte * 512 * 1024
            total_buckets = (width * height * 8) / byte
            if total_buckets == 0:
                total_buckets = 1
            bucket_size = height / total_buckets
        if '-proc=' in option:
            threads = int(option[6:])

    if bucket_size == 0:
        print('Size of sort invalid.')
        return 1
    bucket_size /= threads
    start = 0
    xargs_argument = open('arguments', 'w')

    while start < height:
        line = ' '.join([
            '-sort', '-reverse', argument[1], argument[2],
            '-start=' + str(start), '-end=' +
            str(min(start + bucket_size, height))
        ]) + '\n'
        partialPaths.append(
            'values.' + str(start) + '-' + str(min(start + bucket_size, height)) + '.' + argument[1] + ' ' +
            'positions.' + str(start) + '-' + str(min(start +
                                                      bucket_size, height)) + '.' + argument[1]
        )
        xargs_argument.write(line)
        start = start + bucket_size
    xargs_argument = None
    os.system('cat arguments | xargs -P ' + str(threads) + ' -n 6 ./run')
    a = ' '.join(['./run', '-sort', 'positions.tif', argument[1], '-kway '])
    b = ' '.join(partialPaths)
    os.system(a + b)
