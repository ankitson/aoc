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
import day08
import day09
import day10
import day12
import day13
import day14
import day15
import day16
import day17
import day18
import day21
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
  elif day == 8:
    run_day8()
  elif day == 9:
    run_day9()
  elif day == 10:
    run_day10()
  elif day == 12:
    run_day12()
  elif day == 13:
    run_day13()
  elif day == 14:
    run_day14()
  elif day == 15:
    run_day15()
  elif day == 16:
    run_day16()
  elif day == 17:
    run_day17()
  elif day == 18:
    run_day18()
  elif day == 21:
    run_day21()
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

def run_day8():
  sample = open(util.sample_input(8),'r').read()
  sample2 = open(util.input_file('sample08_2.txt'),'r').read()
  input1 = open(util.real_input(8),'r').read()
  day08.soln1.main(sample,sample2,input1)

def run_day9():
  sample = open(util.sample_input(9),'r').read()
  input1 = open(util.real_input(9),'r').read()
  day09.soln1.main(sample, input1)

def run_day10():
  sample = open(util.sample_input(10),'r').read()
  sample2 = open(util.input_file('sample10_2.txt'),'r').read()
  sample3 = open(util.input_file('sample10_3.txt'),'r').read()
  sample4 = open(util.input_file('sample10_4.txt'),'r').read()
  sample5 = open(util.input_file('sample10_5.txt'),'r').read()
  sample6 = open(util.input_file('sample10_6.txt'),'r').read()
  # sample7 = open(util.input_file('sample10_7.txt'),'r').read()
  # sample8 = open(util.input_file('sample10_8.txt'),'r').read()
  input1 = open(util.real_input(10),'r').read()  

  if len(sys.argv) < 3:
    day10.soln1.main(sample, sample2, sample3, sample4, sample5, sample6, input1)
  else:
    try:
      curses.wrapper(functools.partial(day10.soln1.anim_main,sample,sample2, input1))
    except curses.error as e:
      print("Curses error. Likely your terminal is too small to display the grid")
      import traceback
      traceback.print_exc()

def run_day12():
  sample = open(util.sample_input(12),'r').read()
  input1 = open(util.real_input(12),'r').read()
  day12.soln1.main(sample, input1)

def run_day13():
  sample = open(util.sample_input(13),'r').read()
  input1 = open(util.real_input(13),'r').read()
  day13.soln1.main(sample, input1)

def run_day14():
  sample = open(util.sample_input(14),'r').read()
  input1 = open(util.real_input(14),'r').read()
  day14.soln1.main(sample, input1)

def run_day15():
  sample = open(util.sample_input(15),'r').read()
  input1 = open(util.real_input(15),'r').read()
  day15.soln1.main(sample, input1)

def run_day16():
  sample = open(util.sample_input(16),'r').read()                       
  input1 = open(util.real_input(16),'r').read()
  day16.soln1.main(sample, input1)

def run_day17():
  sample = open(util.sample_input(17),'r').read()
  input1 = open(util.real_input(17),'r').read()
  day17.soln1.main(sample, input1)
def run_day18():
  sample = open(util.sample_input(18),'r').read()
  input1 = open(util.real_input(18),'r').read()
  day18.soln1.main(sample, input1)

def run_day21():
  sample = open(util.sample_input(21),'r').read()
  input1 = open(util.real_input(21),'r').read()
  day21.soln1.main(sample, input1)

if __name__ == '__main__':
  main()
