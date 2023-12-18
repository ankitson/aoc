project('aoc2022', 'cpp',
  version : '0.1',
  default_options : ['warning_level=3', 'cpp_std=c++20'])

exe = executable('aoc2022', 'aoc2022.cpp',
  install : true)
test('basic', exe)

progs = [['day1', ['day1.cpp']]]

foreach p : progs
  exe = executable(p[0], p[1])
  test(p[0], exe)
endforeach


