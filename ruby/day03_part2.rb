# frozen_string_literal: true

lines = File.read('input/day03.txt').split("\n")

numbers_included = {}
data = {}
gears_data = Hash.new { |hash, key| hash[key] = [] }

lines.each.with_index do |line, i|
  data[i] = {}
  numbers_included[i] = []
  nums = line.to_enum(:scan, /\d+/)
             .map { Regexp.last_match }
             .map { |m| [(m.offset(0).first..m.offset(0).last).to_a, m[0].to_i] }.to_h
  data[i][:nums] = nums
  gears = line.to_enum(:scan, /\*/)
              .map { Regexp.last_match }
              .map { |m| m.offset(0).first..m.offset(0).last }
              .flatten
  data[i][:gears] = gears
  nums.each { |is, n| gears.each { |g| gears_data["#{i}|#{g}"] << n if is.any? { |ix| g.include?(ix) } } }
  if i.positive?
    data[i - 1][:nums].each do |is, n|
      gears.each { |g| gears_data["#{i}|#{g}"] << n if is.any? { |ix| g.include?(ix) } }
    end
    nums.each do |is, n|
      data[i - 1][:gears].each { |g| gears_data["#{i - 1}|#{g}"] << n if is.any? { |ix| g.include?(ix) } }
    end
  end
end

p gears_data.values.filter_map { |n| n.reduce(:*) if n.count == 2 }.reduce(:+)
