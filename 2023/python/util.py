import os
from typing import Iterable, Tuple

# PATHS
script_dir = os.path.dirname(os.path.abspath(__file__))
input_dir = os.path.join(script_dir,"inputs")
benchmark_dir = os.path.join(script_dir,"benchmark_results")
def sample_input(day):
  return input_file(f"sample{day:02d}.txt")
def real_input(day):
  return input_file(f"day{day:02d}.txt")
def input_file(name):
  return os.path.join(input_dir, name)
def bench_path(day):
  return os.path.join(benchmark_dir, f"day{day:02d}.json")

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

def print_sep(text:str=None) -> None:
  SEP_LEN = 100
  if text:
    num_char = len(text)
    num_spc = SEP_LEN - num_char
    print(c("RED","-"*(num_spc//2)) + c("BLUE",text) + c("RED","-"*(num_spc//2)))
  else:
    print(c("RED","-"*100))
    
def print_highlight(grid, highlights: Iterable[Tuple[Iterable, str]]):
  outstr = []
  for rownum in range(0,len(grid)):
    for colnum in range(0,len(grid[rownum])):
      item = grid[rownum][colnum]
      item = chr(item + ord('a'))
      out = str(item)

      for (highlight_range, highlight_color) in highlights:
        if (rownum,colnum) in highlight_range:
          out = c(highlight_color, out)
      outstr.append(out)
    outstr.append("\n")
  return ''.join(outstr)