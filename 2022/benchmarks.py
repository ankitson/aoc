import csv
import json
import os

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

# Read rust benchmark results
results = {}
BENCHMARK_DIRS = {
  'rust': 'rust/benchmark_results/',
  'python': 'python/benchmark_results'
}

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

# Write summary to CSV
OUTPUT_FILE = 'benchmark_summary.csv'
f = open(OUTPUT_FILE, 'w')
writer = csv.DictWriter(f, fieldnames=['item','lower_ns','typical_ns','upper_ns'], lineterminator="\n")
writer.writeheader()
keys = sorted(results.keys())
for key in keys:
  result = results[key]
  writer.writerow(result)
f.close()

# Write summary to text and print
OUTPUT_FILE = 'benchmark_summary.txt'
with open(OUTPUT_FILE,'w') as f:
  header_str = f"{'ITEM':<50}{'TIME(ns)':>20}{'TIME(ms)':>20}{'TIME(s)':>20}\n"
  f.write(header_str)
  print(header_str,end="")
  for key in keys:
    result = results[key]
    timens = result['typical_ns']
    timems = timens / 1000
    times = timems / 1000
    result_str = f"{result['item']:<50}{timens:>18.2f}ns{timems:>18.2f}ms{times:>19.2f}s\n"
    f.write(result_str)
    print(result_str,end="")
f.close()
