# frozen_string_literal: true

lines = File.read('input/day03.txt').split("\n")

numbers_included = {}
data = {}

lines.each.with_index do |line, i|
  data[i] = {}
  numbers_included[i] = []
  nums = line.to_enum(:scan, /\d+/)
             .map { Regexp.last_match }
             .map { |m| [(m.offset(0).first..m.offset(0).last).to_a, m[0].to_i] }.to_h
  data[i][:nums] = nums
  joiners = line.to_enum(:scan, /[^.\d]/)
                .map { Regexp.last_match }
                .map { |m| m.offset(0) }
                .flatten
  data[i][:joiners] = joiners
  nums.each { |is, n| numbers_included[i] << n unless is.intersection(joiners).empty? }
  if i.positive?
    data[i - 1][:nums].each { |is, n| numbers_included[i - 1] << n unless is.intersection(joiners).empty? }
    nums.each { |is, n| numbers_included[i] << n unless is.intersection(data[i - 1][:joiners]).empty? }
  end
end

p numbers_included.values.flatten.reduce(:+)
