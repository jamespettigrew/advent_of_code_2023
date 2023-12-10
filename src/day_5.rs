use std::cmp;
use std::collections::HashMap;

#[derive(Debug)]
struct MapRange {
    src_start: usize,
    dest_offset: isize,
    length: usize,
}

struct Map {
    src_type: String,
    ranges: Vec<MapRange>,
}

impl Map {
    fn range_overlap(&self, from: &MapRange) -> Vec<MapRange> {
        let mut results = Vec::<MapRange>::new();
        for r in &self.ranges {
            let overlap_start = cmp::max(from.src_start, r.src_start);
            let overlap_end = overlap_start
                + cmp::min(from.src_start + from.length, r.src_start + r.length)
                - cmp::max(from.src_start, r.src_start);

            if overlap_end > overlap_start {
                results.push(MapRange {
                    src_start: overlap_start,
                    dest_offset: r.dest_offset,
                    length: overlap_end - overlap_start,
                })
            }
        }
        results.sort_by_key(|r| r.src_start);

        // Fill in gaps with 1:1 mappings
        let mut direct_map_ranges = Vec::<MapRange>::new();
        let mut previous_end = from.src_start;
        for r in &results {
            if r.src_start >= previous_end {
                let length = r.src_start - previous_end;
                if length >= 1 {
                    direct_map_ranges.push(MapRange {
                        src_start: previous_end,
                        dest_offset: 0,
                        length: r.src_start - previous_end,
                    });
                }
            }
            previous_end = r.src_start + r.length + 1;
        }
        let from_end = (from.src_start + from.length) as isize;
        let remaining = from_end - (previous_end as isize);
        if remaining > 0 {
            direct_map_ranges.push(MapRange {
                src_start: previous_end,
                dest_offset: 0,
                length: remaining as usize,
            });
        }

        results.append(&mut direct_map_ranges);

        // Shift results by offset
        results
            .iter()
            .map(|r| MapRange {
                src_start: (r.src_start as isize + r.dest_offset) as usize,
                dest_offset: 0,
                length: r.length,
            })
            .collect()
    }

    fn ranges_overlap(&self, from_ranges: Vec<MapRange>) -> Vec<MapRange> {
        from_ranges
            .iter()
            .map(|f| self.range_overlap(f))
            .flatten()
            .collect()
    }
}

fn parse_map(record: &[&str]) -> Map {
    let types: Vec<&str> = record[0].split(" ").next().unwrap().split("-").collect();
    let src_type = types[0].to_string();

    let ranges = record[1..]
        .iter()
        .map(|s| {
            let x: Vec<usize> = s
                .split(" ")
                .filter_map(|x| x.parse::<usize>().ok())
                .collect();
            MapRange {
                src_start: x[1],
                dest_offset: (x[0] as isize) - (x[1] as isize),
                length: x[2],
            }
        })
        .collect();

    Map { src_type, ranges }
}

pub fn solve_a(input: &Vec<&str>) -> usize {
    let seed_ranges: Vec<MapRange> = input[0]
        .split(" ")
        .skip(1)
        .filter_map(|s| s.parse::<usize>().ok())
        .map(|x| MapRange {
            src_start: x,
            dest_offset: 0,
            length: 1,
        })
        .collect();

    let mut delimiter_indices = input
        .iter()
        .enumerate()
        .skip(1)
        .filter(|(_, s)| **s == "")
        .map(|(idx, _)| idx.clone())
        .collect::<Vec<usize>>();
    delimiter_indices.push(input.len());

    let maps_iter = delimiter_indices
        .windows(2)
        .map(|indices| &input[(indices[0] + 1)..indices[1]])
        .map(|record| parse_map(record));
    let maps = HashMap::<String, Map>::from_iter(maps_iter.map(|m| (m.src_type.to_owned(), m)));

    let seed_soil = &maps["seed"];
    let soil_fertilizer = &maps["soil"];
    let fertilizer_water = &maps["fertilizer"];
    let water_light = &maps["water"];
    let light_temperature = &maps["light"];
    let temperature_humidity = &maps["temperature"];
    let humidity_location = &maps["humidity"];

    let mut ranges = seed_soil.ranges_overlap(seed_ranges);
    ranges = soil_fertilizer.ranges_overlap(ranges);
    ranges = fertilizer_water.ranges_overlap(ranges);
    ranges = water_light.ranges_overlap(ranges);
    ranges = light_temperature.ranges_overlap(ranges);
    ranges = temperature_humidity.ranges_overlap(ranges);
    ranges = humidity_location.ranges_overlap(ranges);

    ranges
        .into_iter()
        .min_by_key(|r| r.src_start)
        .unwrap()
        .src_start
}

pub fn solve_b(input: &Vec<&str>) -> usize {
    let seed_values: Vec<usize> = input[0]
        .split(" ")
        .skip(1)
        .filter_map(|s| s.parse::<usize>().ok())
        .collect();
    let seed_ranges: Vec<MapRange> = seed_values
        .chunks(2)
        .map(|c| MapRange {
            src_start: c[0],
            dest_offset: 0,
            length: c[1],
        })
        .collect();

    let mut delimiter_indices = input
        .iter()
        .enumerate()
        .skip(1)
        .filter(|(_, s)| **s == "")
        .map(|(idx, _)| idx.clone())
        .collect::<Vec<usize>>();
    delimiter_indices.push(input.len());

    let maps_iter = delimiter_indices
        .windows(2)
        .map(|indices| &input[(indices[0] + 1)..indices[1]])
        .map(|record| parse_map(record));
    let maps = HashMap::<String, Map>::from_iter(maps_iter.map(|m| (m.src_type.to_owned(), m)));

    let seed_soil = &maps["seed"];
    let soil_fertilizer = &maps["soil"];
    let fertilizer_water = &maps["fertilizer"];
    let water_light = &maps["water"];
    let light_temperature = &maps["light"];
    let temperature_humidity = &maps["temperature"];
    let humidity_location = &maps["humidity"];

    let mut ranges = seed_soil.ranges_overlap(seed_ranges);
    ranges = soil_fertilizer.ranges_overlap(ranges);
    ranges = fertilizer_water.ranges_overlap(ranges);
    ranges = water_light.ranges_overlap(ranges);
    ranges = light_temperature.ranges_overlap(ranges);
    ranges = temperature_humidity.ranges_overlap(ranges);
    ranges = humidity_location.ranges_overlap(ranges);

    ranges
        .into_iter()
        .min_by_key(|r| r.src_start)
        .unwrap()
        .src_start
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_a() {
        let input = vec![
            "seeds: 79 14 55 13",
            "",
            "seed-to-soil map:",
            "50 98 2",
            "52 50 48",
            "",
            "soil-to-fertilizer map:",
            "0 15 37",
            "37 52 2",
            "39 0 15",
            "",
            "fertilizer-to-water map:",
            "49 53 8",
            "0 11 42",
            "42 0 7",
            "57 7 4",
            "",
            "water-to-light map:",
            "88 18 7",
            "18 25 70",
            "",
            "light-to-temperature map:",
            "45 77 23",
            "81 45 19",
            "68 64 13",
            "",
            "temperature-to-humidity map:",
            "0 69 1",
            "1 0 69",
            "",
            "humidity-to-location map:",
            "60 56 37",
            "56 93 4",
        ];
        assert_eq!(solve_a(&input), 35);
    }

    #[test]
    fn test_solve_b() {
        let input = vec![
            "seeds: 79 14 55 13",
            "",
            "seed-to-soil map:",
            "50 98 2",
            "52 50 48",
            "",
            "soil-to-fertilizer map:",
            "0 15 37",
            "37 52 2",
            "39 0 15",
            "",
            "fertilizer-to-water map:",
            "49 53 8",
            "0 11 42",
            "42 0 7",
            "57 7 4",
            "",
            "water-to-light map:",
            "88 18 7",
            "18 25 70",
            "",
            "light-to-temperature map:",
            "45 77 23",
            "81 45 19",
            "68 64 13",
            "",
            "temperature-to-humidity map:",
            "0 69 1",
            "1 0 69",
            "",
            "humidity-to-location map:",
            "60 56 37",
            "56 93 4",
        ];
        assert_eq!(solve_b(&input), 46);
    }
}
