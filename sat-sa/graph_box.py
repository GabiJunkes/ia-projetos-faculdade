import pandas as pd
import matplotlib.pyplot as plt

for iters in ["20", "100", "250"]:
    data_10 = pd.read_csv("data/boxplot_data_"+iters+"_10.csv")["False Clauses"]
    data_5 = pd.read_csv("data/boxplot_data_"+iters+"_5.csv")["False Clauses"]
    data_1 = pd.read_csv("data/boxplot_data_"+iters+"_1.csv")["False Clauses"]

    plt.figure(figsize=(8, 6))
    plt.boxplot([data_10, data_5, data_1], vert=True, patch_artist=True,
                labels=["10%", "5%", "1%"], 
                boxprops=dict(facecolor="lightblue"))

    plt.xlabel("Taxa de Mudança")
    plt.ylabel("Número de Cláusulas Falsas")
    plt.title("Cláusulas Falsas por Taxa de Mudança de vizinho no SA com "+iters+" variáveis")

    plt.savefig("convergence_comparison_"+iters+".png")
