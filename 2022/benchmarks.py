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
BENCHMARK_DIR = 'rust/benchmark_results/'
for filename in os.listdir(BENCHMARK_DIR):
  if filename.endswith('.json'):
    filepath = os.path.join(BENCHMARK_DIR, filename)
    with open(filepath, 'r') as file:
      contents = file.readlines()
      for line in contents:
        item = json.loads(line)
        if item['reason'] == 'benchmark-complete':
          item_id = item['id']
          (lb,typical,ub) = extract_measure(item['typical'])
          results[item_id] = {'item': item_id, 'lower': lb, 'typical': typical, 'upper': ub}

# Write summary to JSON
OUTPUT_FILE = 'benchmark_summary.json'
f = open(OUTPUT_FILE, 'w')
keys = sorted(results.keys())
for key in keys:
  serialized = json.dumps(results[key])
  f.write(serialized+"\n")
f.close()

# Write summary to text and print
OUTPUT_FILE = 'benchmark_summary.txt'
with open(OUTPUT_FILE,'w') as f:
  header_str = f"{'ITEM':50}           {'TIME(ns)':15}     {'TIME(ms)':15}    {'TIME(s)':15}\n"
  f.write(header_str)
  print(header_str,end="")
  for key in keys:
    result = results[key]
    timens = result['typical']
    timems = timens / 1000
    times = timems / 1000
    result_str = f"{result['item']:50}    {timens:15.2f}ns {timems:15.2f}ms {times:15.2f}s\n"
    f.write(result_str)
    print(result_str,end="")
f.close()
