#!/usr/bin/env python3

import csv
from math import ceil
import sys

import numpy as np

import matplotlib
import matplotlib.pyplot as plt

from statistics import median

def load_csv(path):
    timings = ([], [])

    with open(path) as f:
        reader = csv.reader(f, delimiter=';')

        for row in reader:
            if len(row) < 2:
                continue
            if row[0] == "PUT":
                timings[0].append(int(row[1]))
            else:
                timings[1].append(int(row[1]))

        assert(len(timings[0]) > 0)
        assert(len(timings[1]) > 0)
        return timings

def draw_graph(data, num_bins, save_to, type):
    assert(len(data) > 0)
    fig, ax = plt.subplots()
    bin_interval = 100
    top = 30 * bin_interval
    bin_count = int(ceil(top / bin_interval))

    bins = [k * bin_interval for k in range(30)]
    n, bins, patches = ax.hist(data, bins, density=True)

    med = median(data)

    ax.axvline(x=med, color='C1')

    plt.xlim([0, top])

    ax.set_xlabel("Duration in ns")
    ax.set_ylabel("Request percentage")
    ax.set_title("Histogram of " + type + " request time")
    fig.tight_layout()
    plt.show()
    fig.savefig(save_to + "_" + type + ".png", bbox_inches='tight')

if __name__ == '__main__':
    file = ""
    out = ""
    for i in range(1, len(sys.argv)):
        if sys.argv[i] == "-o":
            out = sys.argv[i + 1]
        elif sys.argv[i - 1] == "-o":
            continue
        else:
            file = sys.argv[i]

    data = load_csv(file)
    data[1].sort()
    data[0].sort()
    draw_graph(data[0], 50, out, "PUT")
    draw_graph(data[1], 50, out, "GET")
