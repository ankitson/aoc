import curses
import functools
import sys

import day01
import day02
import day03
import day04
import day05
import day06
import day07
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
  elif day == 3:
    run_day3()
  elif day == 4:
    run_day4()
  elif day == 5:
    run_day5()
  elif day == 6:
    run_day6()
  elif day == 7:
    run_day7()
  else:
    raise Exception(f"Day {day} not implemented")

def run_day1():
  sample = open(util.sample_input(1),'r').read()
  input1 = open(util.real_input(1),'r').read()
  day01.soln1.main(sample,input1)

def run_day2():
  sample = open(util.sample_input(2),'r').read()
  input1 = open(util.real_input(2),'r').read()
  day02.soln1.main(sample,input1)

def run_day3():
  sample = open(util.sample_input(3),'r').read()
  input1 = open(util.real_input(3),'r').read()
  day03.soln1.main(sample,input1)

def run_day4():
  sample = open(util.sample_input(4),'r').read()
  input1 = open(util.real_input(4),'r').read()
  day04.soln1.main(sample,input1)

def run_day5():
  sample = open(util.sample_input(5),'r').read()
  input1 = open(util.real_input(5),'r').read()
  day05.soln1.main(sample,input1)

def run_day6():
  sample = open(util.sample_input(6),'r').read()
  input1 = open(util.real_input(6),'r').read()
  day06.soln1.main(sample,input1)

def run_day7():
  sample = open(util.sample_input(7),'r').read()
  input1 = open(util.real_input(7),'r').read()
  day07.soln1.main(sample,input1)

if __name__ == '__main__':
  main()
