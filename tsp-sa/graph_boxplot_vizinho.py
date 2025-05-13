import pandas as pd
import matplotlib.pyplot as plt

# Listas com os parâmetros
vars_list = [51, 100]
sa_max_list = [1, 5, 10]


for idx, vars_value in enumerate(vars_list):
    # Tamanho e estilo do gráfico
    plt.style.use("seaborn-v0_8-deep")
    fig, axs = plt.subplots(1, 1, figsize=(15, 5), sharey=True)
    data = []
    labels = []

    for sa_max in sa_max_list:
        filename = f"data/boxplot_data_{vars_value}_{sa_max}.csv"
        try:
            df = pd.read_csv(filename)
            if "False Clauses" not in df.columns:
                print(f"Coluna 'False Clauses' não encontrada em {filename}")
                continue
            data.append(df["False Clauses"])
            labels.append(f"{sa_max}%")
        except FileNotFoundError:
            print(f"Arquivo não encontrado: {filename}")
            continue

    axs.boxplot(data, labels=labels, patch_artist=True)
    axs.set_title(f"Problema com {vars_value} cidades")
    axs.set_xlabel("Porcentagem máxima de alterações no vetor vizinho")
    if idx == 0:
        axs.set_ylabel("Caminho mínimo")

    plt.suptitle("Distribuição do Caminho mínimo por alterações no vetor vizinho")
    plt.tight_layout()
    plt.show()