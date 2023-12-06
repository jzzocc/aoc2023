# frozen_string_literal: true

almanac = File.read('input/day05.txt').split("\n\n")

seed_ranges = almanac.shift.scan(/\d+/).map(&:to_i).each_slice(2).map { |n, l| n...(n + l) }.flatten

maps = almanac.map do |map|
  map.split("\n")
     .drop(1)
     .map { |l| l.scan(/\d+/).map(&:to_i) }
     .map do |line|
       range_length = line.pop
       line.map { |l| (l...(l + range_length)) }.reverse
     end
     .to_h
end

range_count = seed_ranges.count
result = seed_ranges.map.with_index do |range, range_index|
  seed_count = range.count
  range.map.with_index do |seed, seed_index|
    pct = (seed_index + 1).fdiv(seed_count).round(2) * 100
    puts "processing seed #{seed_index + 1} of #{seed_count} (#{pct}%) in range #{range_index + 1} of #{range_count}"
    value = seed
    maps.each do |map|
      map.each do |k, v|
        if k.cover?(value)
          value = v.min + (value - k.min)
          break
        end
      end
    end
    value
  end
end
.flatten

p result.min
