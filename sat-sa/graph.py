import pandas as pd
import matplotlib.pyplot as plt
file_paths = ["convergence_data_20_1.csv", "convergence_data_100_1.csv", "convergence_data_250_1.csv"]
for file_path in file_paths:
  data = pd.read_csv("data/"+file_path)

  fig, ax1 = plt.subplots(figsize=(10, 5))

  ax1.plot(data["Generation"], data["False Clauses"], marker=",", linestyle="-", color="b", label="Cláusulas Falsas")
  ax1.set_xlabel("Geração")
  ax1.set_ylabel("Número de Cláusulas Falsas", color="b")
  ax1.tick_params(axis="y", labelcolor="b")
  ax1.grid(True)

  ax2 = ax1.twinx()
  ax2.plot(data["Generation"][::10], data["Temperature"][::10], marker=".", linestyle="-", color="r", alpha=0.2, label="Temperatura")
  ax2.set_ylabel("Temperatura", color="r")
  ax2.tick_params(axis="y", labelcolor="r")

  plt.title("Convergência do Simulated Annealing para SAT")
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