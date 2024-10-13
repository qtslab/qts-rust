import pandas as pd
import matplotlib.pyplot as plt

def parse_and_plot(csv_file):
    # 讀取 CSV 檔案
    try:
        df = pd.read_csv(csv_file, header=None)
        # 提取迭代次數和價值數據
        iteration_data = df.iloc[::2, 0].str.split(expand=True)
        iteration_data.columns = ['Iteration', 'Value', 'Weight', 'Length']

        # 確保數據為數值類型
        iteration_data['Iteration'] = pd.to_numeric(iteration_data['Iteration'])
        iteration_data['Value'] = pd.to_numeric(iteration_data['Value'])

        # 繪製圖表
        plt.figure(figsize=(10, 6))
        plt.plot(iteration_data['Iteration'], iteration_data['Value'], marker='o', linestyle='-', color='b')

        # 加入標籤和標題
        plt.xlabel('Iteration')
        plt.ylabel('Value')
        plt.title('Iteration vs Value')

        # 顯示圖表
        plt.grid(True)
        plt.show()

    except Exception as e:
        print(f"解析 CSV 文件時發生錯誤: {e}")

if __name__ == "__main__":
    csv_file = "output.csv"
    parse_and_plot(csv_file)
