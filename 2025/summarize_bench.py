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
    "DIM": "\033[2m",
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
  CSV_OUTPUT = 'bench.csv'
  MD_OUTPUT = 'bench.md'

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
    
  print(f"Skipped {sum(nosums.values()):.2f}ns of reports from total - {list(nosums.keys())}")

  # Write summary to files & stdout
  with open(CSV_OUTPUT, 'w') as csv_out, open(MD_OUTPUT,'w') as md_out:
    csv_writer = csv.DictWriter(csv_out, fieldnames=['item','lower_ns','typical_ns','upper_ns',], lineterminator="\n")
    csv_writer.writeheader()

    md_header_str = f"|{'ITEM':<50}|{'TIME(ns)':>20}|{'TIME(µs)':>20}|{'TIME(ms)':>15}|{'TIME(s)':>15}|\n"
    md_header_str += "|" + "-" * 50 + "|" + "-" * 20 + "|" + "-" * 20 + "|" + "-" * 15 + "|" + "-" * 15 + "|" + "\n" 
    md_out.write(md_header_str)
    print(c(["BLUE","BOLD"],md_header_str),end="")

    for key in bench_keys:
      result = results[key]
      csv_writer.writerow(result)

      timens = result['typical_ns']
      timeus, timems, times = timens/(1000), timens/(1000_000), timens/(1000_000_000) 
      md_str = f"|{result['item']:<50}|{timens:>18.2f}ns|{timeus:>18.2f}µs|{timems:>13.2f}ms|{times:>14.2f}s|\n"
      md_out.write(md_str)

      if md_str.find('nosum') != -1:
        print(c("DIM",md_str),end="")
      elif md_str.find('python') != -1:
        print(c("GREEN",md_str),end="")
      elif md_str.find('rust') != -1:
        print(c("RED",md_str),end="")
    
    for (k,v) in totals.items():
      item = k+".TOTAL"
      csv_writer.writerow({"item": item, "lower_ns": v['lower_ns'], "typical_ns": v['typical_ns'], "upper_ns": v['upper_ns']})

      timens = v['typical_ns']
      timeus, timems, times = timens/(1000), timens/(1000_000), timens/(1000_000_000)      
      md_str = f"|{item:<50}|{timens:>18.2f}ns|{timeus:>18.2f}µs|{timems:>13.2f}ms|{times:>14.2f}s|\n"
      md_out.write(md_str)

      print(c("BLUE",md_str),end="")


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

