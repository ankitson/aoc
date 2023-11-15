import csv
import json
import os

def main():
  results = {}
  BENCHMARK_DIRS = {
    'rust': 'rust/benchmark_results/',
    'python': 'python/benchmark_results/'
  }
  CSV_OUTPUT = 'benchmark_summary.csv'
  TXT_OUTPUT = 'benchmark_summary.txt'

  for (lang,BENCHMARK_DIR) in BENCHMARK_DIRS.items():
    for filename in os.listdir(BENCHMARK_DIR):
      if filename.endswith('.json'):
        filepath = os.path.join(BENCHMARK_DIR, filename)
        with open(filepath, 'r') as file:
          contents = file.readlines()
          for line in contents:
            item = json.loads(line)
            if item['reason'] == 'benchmark-complete':
              item_id = lang + "." + item['id']
              (lb,typical,ub) = extract_measure(item['typical'])
              results[item_id] = {'item': item_id, 'lower_ns': lb, 'typical_ns': typical, 'upper_ns': ub}

  bench_keys = sorted(results.keys(),key=lambda id: str(id.split(".")[1:]))

  with open(CSV_OUTPUT, 'w') as csv_out, open(TXT_OUTPUT,'w') as txt_out:
  # Write summary to CSV
    csv_writer = csv.DictWriter(csv_out, fieldnames=['item','lower_ns','typical_ns','upper_ns'], lineterminator="\n")
    csv_writer.writeheader()

    header_str = f"{'ITEM':<50}{'TIME(ns)':>20}{'TIME(ms)':>20}{'TIME(s)':>20}\n"
    txt_out.write(header_str)

    print(header_str,end="")
    for key in bench_keys:
      result = results[key]
      csv_writer.writerow(result)

      timens = result['typical_ns']
      timems, times = timens/1000, timens/(1000_000)
      result_str = f"{result['item']:<50}{timens:>18.2f}ns{timems:>18.2f}ms{times:>19.2f}s\n"
      txt_out.write(result_str)

      print(result_str,end="")

def extract_measure(dict):
  unit = dict['unit']
  mult = 1
  if unit != 'ns':
    if unit == 'ms':
      mult = 1/1000
    elif unit == 's':
      mult = 1/1000_000
    else:
      raise Exception(f"unknown unit {unit} in dict {dict}")
  estimate = float(dict['estimate']) * mult
  lower_bound = float(dict['lower_bound']) * mult
  upper_bound = float(dict['upper_bound']) * mult
  return (lower_bound,estimate,upper_bound)

if __name__ == '__main__':
  main()
