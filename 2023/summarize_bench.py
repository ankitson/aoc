import collections
import csv
import json
import re
import os
from typing import Iterable

# PRETTY-PRINTING
def c(colors: str | Iterable[str], text: str) -> str:
  color_dict = {
    "PURPLE": "\033[95m",
    "CYAN": "\033[96m",
    "DARKCYAN": "\033[36m",
    "BLUE": "\033[94m",
    "GREEN": "\033[92m",
    "YELLOW": "\033[93m",
    "RED": "\033[91m",
    "BOLD": "\033[1m",
    "UNDERLINE": "\033[4m",
    "END": "\033[0m"
  }

  if type(colors) is str:
    colors = [colors]

  color_text = []
  for k, v in color_dict.items():
    shortk = k.lower()[0:1]
    for color in colors:
      if shortk == color or k == color:
        color_text.append(v)
      
  color_text.append(text + color_dict["END"])
  return ''.join(color_text)

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
      if (not filename.startswith('dhat')) and filename.endswith('.json'):
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

  totals = collections.defaultdict(lambda: collections.defaultdict(int))
  nosums = {}
  for (key,val) in results.items():
    
    parts = re.split("\.|/", key)
    lang, day, soln, inp_type, ident = parts[0], parts[1], parts[2], parts[3], parts[4]
    if key.find("nosum") != -1:
      nosums[key] = val['typical_ns']
      continue
    else:
      totals[lang]['lower_ns'] += val['lower_ns']
      totals[lang]['typical_ns'] += val['typical_ns']
      totals[lang]['upper_ns'] += val['upper_ns']
    
  print(f"Skipped {sum(nosums.values()):.2f}ns of reports - {list(nosums.keys())}")

  with open(CSV_OUTPUT, 'w') as csv_out, open(TXT_OUTPUT,'w') as txt_out:
  # Write summary to CSV
    csv_writer = csv.DictWriter(csv_out, fieldnames=['item','lower_ns','typical_ns','upper_ns',], lineterminator="\n")
    csv_writer.writeheader()

    header_str = f"{'ITEM':<50}{'TIME(ns)':>20}{'TIME(µs)':>20}{'TIME(ms)':>20}{'TIME(s)':>20}\n"
    txt_out.write(header_str)
    print(c(["BLUE","BOLD"],header_str),end="")

    for key in bench_keys:
      result = results[key]
      csv_writer.writerow(result)

      timens = result['typical_ns']
      timeus, timems, times = timens/(1000), timens/(1000_000), timens/(1000_000_000) 
      result_str = f"{result['item']:<50}{timens:>18.2f}ns{timeus:>18.2f}µs{timems:>18.2f}ms{times:>19.2f}s\n"
      txt_out.write(result_str)

      if result_str.startswith('python'):
        print(c("GREEN",result_str),end="")
      elif result_str.startswith('rust'):
        print(c("RED",result_str),end="")
      # elif result_str.endswith('TOTAL'):
        # print(c("BLUE",result_str),end="")
    
    for (k,v) in totals.items():
      item = k+".TOTAL"
      csv_writer.writerow({"item": item, "lower_ns": v['lower_ns'], "typical_ns": v['typical_ns'], "upper_ns": v['upper_ns']})

      result_str = f"{result['item']:<50}{timens:>18.2f}ns{timeus:>18.2f}µs{timems:>18.2f}ms{times:>19.2f}s\n"
      timens = v['typical_ns']
      timeus, timems, times = timens/(1000), timens/(1000_000), timens/(1000_000_000)
      result_str = f"{item:<50}{timens:>18.2f}ns{timeus:>18.2f}µs{timems:>18.2f}ms{times:>19.2f}s\n"
      txt_out.write(result_str)

      print(c("BLUE",result_str),end="")


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

