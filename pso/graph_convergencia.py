import pandas as pd
import matplotlib.pyplot as plt
file_paths = ["data.csv"]
for file_path in file_paths:
  data = pd.read_csv("data/"+file_path)

  fig, ax1 = plt.subplots(figsize=(10, 5))

  ax1.plot(data["Geracao"], data["BestValue"], marker=",", linestyle="-", color="b", label="Melhor Resultado")
  ax1.set_xlabel("Geração")
  ax1.set_ylabel("Melhor Resultado", color="b")
  ax1.set_yscale("log")
  ax1.tick_params(axis="y", labelcolor="b")
  ax1.grid(True)

  plt.title("PSO com DIM=1000, 30 particulas")
  fig.tight_layout()
  plt.savefig("graphs/"+file_path+".png")