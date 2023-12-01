# frozen_string_literal: true

NUMBERS = {
  'one' => '1',
  'two' => '2',
  'three' => '3',
  'four' => '4',
  'five' => '5',
  'six' => '6',
  'seven' => '7',
  'eight' => '8',
  'nine' => '9'
}.freeze
FIRST_REGEX = /(#{NUMBERS.keys.join('|')}|\d)/.freeze
LAST_REGEX = /.*#{FIRST_REGEX}/.freeze

lines = File.read('input/day01.txt').split("\n")

result = lines.map do |line|
  first_number = line.match(FIRST_REGEX).captures.first
  last_number = line.match(LAST_REGEX).captures.last
  ((NUMBERS[first_number] || first_number) + (NUMBERS[last_number] || last_number)).to_i
end
.reduce(&:+)

p result
