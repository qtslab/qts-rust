import os
import pandas as pd
import numpy as np
import matplotlib.pyplot as plt

def read_and_average(directory, num_files):
    all_values = []
    all_weights = []

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
                iteration_data['Weight'] = pd.to_numeric(iteration_data['Weight'])

                # 添加到 all_values 和 all_weights 列表中
                all_values.append(iteration_data['Value'].values)
                all_weights.append(iteration_data['Weight'].values)
            except Exception as e:
                print(f"Error processing file {file_path}: {e}")

    # 對所有測試結果取平均
    if all_values and all_weights:
        all_values = np.array(all_values)
        all_weights = np.array(all_weights)
        average_values = np.mean(all_values, axis=0)
        average_weights = np.mean(all_weights, axis=0)
        return average_values, average_weights
    else:
        print(f"No valid data found in {directory}")
        return None, None

def plot_comparison(qts_avg, qts_weight_avg, ae_qts_avg, ae_qts_weight_avg):
    plt.figure(figsize=(10, 6))

    # 繪製 QTS 和 AE-QTS 的「價值」曲線
    plt.plot(qts_avg, label="QTS Value", marker='o', linestyle='-', color='b')
    plt.plot(ae_qts_avg, label="AE-QTS Value", marker='x', linestyle='-', color='r')

    # 繪製 QTS 和 AE-QTS 的「重量」曲線
    plt.plot(qts_weight_avg, label="QTS Weight", marker='o', linestyle='--', color='b', alpha=0.5)
    plt.plot(ae_qts_weight_avg, label="AE-QTS Weight", marker='x', linestyle='--', color='r', alpha=0.5)

    # 加入標籤和標題
    plt.xlabel('Iteration')
    plt.ylabel('Average Value / Weight')
    plt.title('Comparison of QTS and AE-QTS (Value and Weight)')
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
    qts_avg, qts_weight_avg = read_and_average(qts_dir, num_files)

    print("Calculating average for AE-QTS...")
    ae_qts_avg, ae_qts_weight_avg = read_and_average(ae_qts_dir, num_files)

    # 如果都成功讀取，則繪製比較圖表
    if qts_avg is not None and ae_qts_avg is not None:
        plot_comparison(qts_avg, qts_weight_avg, ae_qts_avg, ae_qts_weight_avg)
    else:
        print("Error in calculating averages.")

if __name__ == "__main__":
    main()
