import pandas as pd
import matplotlib.pyplot as plt
file_paths = ["data.csv"]
for file_path in file_paths:
  data = pd.read_csv("data/"+file_path)

  fig, ax1 = plt.subplots(figsize=(10, 5))

  ax1.plot(data["Geracao"], data["BestValue"], marker=",", linestyle="-", color="b", label="Caminho mínimo")
  ax1.set_xlabel("Geração")
  ax1.set_ylabel("Caminho mínimo", color="b")
  ax1.tick_params(axis="y", labelcolor="b")
  ax1.grid(True)

  plt.title("Convergência do Simulated Annealing para TSP")
  fig.tight_layout()
  plt.savefig("graphs/"+file_path+".png")


# Graficos de convergencia
# Tabela com média e desviopadrao (mean +- std)
# Grafico box-plot 
# Considerar SAMax = 1, 5 e 10 para experimentacao
# Criterio de parada tem q ser iteracao e nao temperatura
# Boxplot:
# diferentes valores de SAMax
# diferentes equacoes de energia
# Diferentes rotinas de geracao de vizinhos 
