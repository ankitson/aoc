LINES = open('../inputs/day03.txt').read

gears = {}

LINES.each_line.each_with_index do |line, j|
  line.scan(/[^\s\d.]/) do
    # NOTE: `$~.begin(0)` is the beginning index of the regex match (
    # and `$~.end(0)` is one past the end of the match)
    gears[$~.begin(0) + j.i] = []
  end
end

LINES.each_line.each_with_index do |line, j|
  line.scan(/\d+/) do
    ($~.begin(0) - 1..$~.end(0)).each do |y|
      (j - 1..j + 1).each do |x|
        gears[y + x.i]&.push $&.to_i
      end
    end
  end
end

p gears.sum { |_key, ids| ids.length == 2 ? ids.reduce(:*) : 0 }