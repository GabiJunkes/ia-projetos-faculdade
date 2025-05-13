import pandas as pd

# Lista com os valores de vars
vars_list = [51, 100]

# Itera pelos arquivos e calcula média e desvio padrão
for vars_value in vars_list:
    filename = f"data/boxplot_data_{vars_value}_10.csv"
    try:
        # Lê o CSV
        df = pd.read_csv(filename)
        
        # Verifica se a coluna "False Clauses" existe
        if "False Clauses" not in df.columns:
            print(f"Coluna 'False Clauses' não encontrada em {filename}")
            continue
        
        # Calcula média e desvio padrão
        media = df["False Clauses"].mean()
        desvio = df["False Clauses"].std()

        print(f"Arquivo: {filename}")
        print(f"  Média: {media:.2f}")
        print(f"  Desvio Padrão: {desvio:.2f}\n")

    except FileNotFoundError:
        print(f"Arquivo não encontrado: {filename}")