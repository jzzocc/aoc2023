# frozen_string_literal: true

lines = File.read('input/day01.txt').split("\n")

result = lines.map do |line|
  nums = line.gsub(/\D/, '')
  (nums[0] + nums[-1]).to_i
end
.reduce(:+)

p result
