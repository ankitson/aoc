import json
import os
import pyperf
import sys

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

if __name__  == '__main__':
  #Pyperf passes anything in sys.argv to the worker processes so we can't use it...
  #use env variable instead
  # day = int(os.environ.get('BENCH_DAY'))
  day = 7
  sample = open(util.sample_input(day),'r').read()
  input1 = open(util.real_input(day),'r').read()
  bench_file = open(util.bench_path(day),'w')

  NPROCESSES = 5
  pyperf_runner = pyperf.Runner(processes=NPROCESSES)

  def bench_it(tag, fn, *args):
    result = pyperf_runner.bench_func(tag,fn,*args)
    #Each process returns seperately, we want to wait for all to finish
    if result and result.get_nrun() == NPROCESSES + 1: 
      text_out = str_result(result, tag)
      bench_file.write(text_out+"\n")
  
  if day == 7:
    bench_it('day07.part1.realinput', day07.soln1.part1,input1,100_000,True)
    bench_it('day07.part2.realinput', day07.soln1.part2,input1,70_000_000, 30_000_000)
  elif day == 12:
    bench_it('day12.part1.realinput', day12.soln1.part1,sample,input1)
    bench_it('day12.part2.realinput', day12.soln1.part2,sample,input1)

  bench_file.close()