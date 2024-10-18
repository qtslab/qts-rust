import os
import pandas as pd
import numpy as np
import matplotlib.pyplot as plt

def read_and_average(directory, num_files):
    all_iterations = []

    for i in range(1, num_files + 1):
        file_path = os.path.join(directory, f"{i}.csv")
        if os.path.exists(file_path):
            # 讀取 CSV 文件
            try:
                df = pd.read_csv(file_path, header=None)
                iteration_data = df.iloc[::2, 0].str.split(expand=True)
                iteration_data.columns = ['Iteration', 'Value', 'Weight', 'Length']

                # 確保數據為數值類型
                iteration_data['Value'] = pd.to_numeric(iteration_data['Value'])

                # 添加到 all_iterations 列表中
                all_iterations.append(iteration_data['Value'].values)
            except Exception as e:
                print(f"Error processing file {file_path}: {e}")

    # 對所有測試結果取平均
    if all_iterations:
        all_iterations = np.array(all_iterations)
        average_values = np.mean(all_iterations, axis=0)
        return average_values
    else:
        print(f"No valid data found in {directory}")
        return None

def plot_comparison(qts_avg, ae_qts_avg):
    plt.figure(figsize=(10, 6))

    plt.plot(qts_avg, label="QTS", marker='o', linestyle='-', color='b')
    plt.plot(ae_qts_avg, label="AE-QTS", marker='x', linestyle='-', color='r')

    # 加入標籤和標題
    plt.xlabel('Iteration')
    plt.ylabel('Average Value')
    plt.title('Comparison of QTS and AE-QTS')
    plt.legend()

    # 顯示圖表
    plt.grid(True)
    plt.show()

def main():
    # QTS 和 AE-QTS 的資料夾路徑
    qts_dir = "csv/qts"
    ae_qts_dir = "csv/ae-qts"

    # 設定測試檔案數量
    num_files = 1000

    # 讀取並計算平均值
    print("Calculating average for QTS...")
    qts_avg = read_and_average(qts_dir, num_files)

    print("Calculating average for AE-QTS...")
    ae_qts_avg = read_and_average(ae_qts_dir, num_files)

    # 如果都成功讀取，則繪製比較圖表
    if qts_avg is not None and ae_qts_avg is not None:
        plot_comparison(qts_avg, ae_qts_avg)
    else:
        print("Error in calculating averages.")

if __name__ == "__main__":
    main()
