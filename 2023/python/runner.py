import curses
import functools
import sys

import day01
import day02
import util

def main():
  day = sys.argv[1]
  if day[0:3] == 'day':
    day = (day[3:])
  day = int(day)
    
  if day == 1:
    run_day1()
  elif day == 2:
    run_day2()
  else:
    raise Exception(f"Day {day} not implemented")

def run_day1():
  sample = open(util.sample_input(1),'r').read()
  input1 = open(util.real_input(1),'r').read()
  day01.soln1.main(sample,input1)

def run_day2():
  sample = open(util.sample_input(1),'r').read()
  input1 = open(util.real_input(1),'r').read()
  day02.soln1.main(sample,input1)

if __name__ == '__main__':
  main()
