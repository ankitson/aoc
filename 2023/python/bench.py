import json
import os
import pyperf
import sys

import bench_config
import day07
import day12
import util

def str_result(result, tag):
  #Write a result in a subset of the format criterion uses so we can parse it the same way
  mean = result.mean()
  stdev = result.stdev()
  unit = result.get_unit()
  assert(unit == 'second')
  mean_ns = mean * (10**9)
  stdev_ns = stdev * (10**9)
  lower = mean_ns - stdev_ns
  upper = mean_ns + stdev_ns
  obj = {'reason': 'benchmark-complete', 'id': tag, 
         'typical': {'unit': 'ns', 
      'lower_bound': lower, 
      'estimate': mean_ns, 
      'upper_bound': upper}}
  return json.dumps(obj)

def main():
  NPROCESSES = 5
  pyperf_runner = pyperf.Runner(processes=NPROCESSES,)

  # Pyperf reuses sys.argv for its workers and i couldnt figure out how to inherit env
  # So, we read the day from another file and set it in the justfile before launch
  day = bench_config.day
  sample = open(util.sample_input(day),'r').read()
  input1 = open(util.real_input(day),'r').read()
  bench_file = open(util.bench_path(day),'w')

  def bench_it(tag, fn, *args):
    result = pyperf_runner.bench_func(tag,fn,*args)
    #Each process returns seperately, we want to wait for all to finish
    if result and result.get_nrun() == NPROCESSES + 1: 
      text_out = str_result(result, tag)
      bench_file.write(text_out+"\n")
  
  if day == 7:
    bench_it('day07.soln1.realinput/part1', day07.soln1.part1,input1,100_000,True)
    bench_it('day07.soln2.realinput/part2', day07.soln1.part2,input1,70_000_000, 30_000_000)
  elif day == 12:
    bench_it('day12.soln1.realinput/part1', day12.soln1.part1,input1)
    bench_it('day12.soln1.realinput/part2', day12.soln1.part2,input1)

  bench_file.close()

if __name__  == '__main__':
  main()