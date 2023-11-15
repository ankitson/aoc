import curses
import functools
import sys


import day07
import day12
import util

def main():
  day = sys.argv[1]
  if day[0:3] == 'day':
    day = (day[3:])
  day = int(day)
    
  if day == 7:
    run_day7()
  elif day == 12:
    run_day12()
  else:
    raise Exception(f"{day} not implemented")

def run_day7():
  sample = open(util.sample_input(7),'r').read()
  input1 = open(util.real_input(7),'r').read()
  day07.soln1.main(sample,input1)

def run_day12():
  sample = open(util.sample_input(12),'r').read()
  input1 = open(util.real_input(12),'r').read()
  if len(sys.argv) > 2 and sys.argv[2] == 'noanim':
    day12.soln1.main(sample,input1)
  else:
    try:
      curses.wrapper(functools.partial(day12.soln1.anim_main,sample,input1))
    except curses.error as e:
      print("Curses error. Likely your terminal is too small to display the grid")
      import traceback
      traceback.print_exc()

if __name__ == '__main__':
  main()