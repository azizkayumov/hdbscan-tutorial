import numpy as np
import matplotlib.pyplot as plt

def setup_plot(data):
    # Plot config
    plt.clf()
    plt.rc('axes', axisbelow=True)
    plt.grid(zorder=0, linestyle='--', alpha=0.5, color='gray', linewidth=0.5)
    plt.xlim(0, 10)
    plt.ylim(0, 10)
    plt.gca().set_aspect('equal', adjustable='box')
    plt.xticks(np.arange(0, 11, 1))
    plt.yticks(np.arange(0, 11, 1))
    plt.gca().set_aspect('equal', adjustable='box')
    plt.gca().tick_params(length=0)  # Remove little tick lines
    plt.gca().set_xticklabels([])
    plt.gca().set_yticklabels([])

# Plot the data
data = np.loadtxt('data/data.csv', delimiter=',', dtype=float)
setup_plot(data)
for i, (x, y) in enumerate(data):
    plt.scatter(x, y, color='black', marker='o', s=200)
names = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't']
colors = ['red', 'blue', 'green', 'orange', 'purple', 'black']
for i, txt in enumerate(names):
    plt.annotate(txt, (data[i][0], data[i][1]), fontsize=10, color='white', weight='bold', ha='center', va='center')
plt.savefig('plot/data.png', dpi=200, bbox_inches='tight')


# Plot the unsupervised clustering results
labels = np.loadtxt('output/clusters.csv', delimiter=',', dtype=int)
for i, label in enumerate(labels):
    color = colors[label % len(colors)]
    plt.scatter(data[i][0], data[i][1], color=color, marker='o', s=200)
for i, txt in enumerate(names):
    plt.annotate(txt, (data[i][0], data[i][1]), fontsize=10, color='white', weight='bold', ha='center', va='center')
plt.savefig('plot/clusters.png', dpi=200, bbox_inches='tight')


# Plot partial labels
setup_plot(data)
partial_labels = np.loadtxt('data/partial_labels.csv', delimiter=',', dtype=int)
for i, label in enumerate(partial_labels):
    if label == -1:  # Unlabeled points
        color = 'black'
    else:
        color = colors[label % len(colors)]
    plt.scatter(data[i][0], data[i][1], color=color, marker='o', s=200)
for i, txt in enumerate(names):
    plt.annotate(txt, (data[i][0], data[i][1]), fontsize=10, color='white', weight='bold', ha='center', va='center')
plt.savefig('plot/partial_labels.png', dpi=200, bbox_inches='tight')


# Plot the semi-supervised clustering results
setup_plot(data)
labels = np.loadtxt('output/semi_clusters.csv', delimiter=',', dtype=int)
for i, label in enumerate(labels):
    color = colors[label % len(colors)]
    plt.scatter(data[i][0], data[i][1], color=color, marker='o', s=200)
for i, txt in enumerate(names):
    plt.annotate(txt, (data[i][0], data[i][1]), fontsize=10, color='white', weight='bold', ha='center', va='center')
plt.savefig('plot/semi_clusters.png', dpi=200, bbox_inches='tight')


# Plot outlier scores of the data points
setup_plot(data)
outlier_scores = np.loadtxt('output/outlier_scores.csv', delimiter=',', dtype=float)
for i, score in enumerate(outlier_scores):
    if score > 0.5:  # Threshold for outliers
        color = 'red'
    else:
        color = 'black'
    plt.scatter(data[i][0], data[i][1], color=color, marker='o', s=200)
for i, txt in enumerate(names):
    plt.annotate(txt, (data[i][0], data[i][1]), fontsize=10, color='white', weight='bold', ha='center', va='center')
plt.savefig('plot/outlier_scores.png', dpi=200, bbox_inches='tight')